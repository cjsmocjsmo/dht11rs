// use dht_mmap_rust::{Dht, DhtType};

// fn main() {
//     // The sensor is a DHT11 connected on pin 23
//     let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

//     // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
//     // the read succeeds.
//     // For more information, see documentation on `read()`
//     let reading = dht.read().unwrap();

//     // let temp_c = reading.temperature();
//     // let temperature_f = temp_c * 9.0 / 5.0 + 32.0;

//     println!("{:.2}", reading.humidity());
// }

use chrono::{Datelike, Local, Timelike, TimeZone};
use dht_mmap_rust::{Dht, DhtType};
use rusqlite::{params, Connection, Result};
// use std::f32::MIN;
use std::fs;
use std::path::Path;
// use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct SensorData {
    tempc: String,
    tempf: String,
    humi: String,
    date: String,
    time: String,
}

fn read_data(mut dht: Dht, d: String, t: String) -> SensorData {
    // let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();
    let reading = dht.read().unwrap();
    let temp = reading.temperature();
    let tempc = format!("{:.2}", temp);
    let tempff = temp * 9.0 / 5.0 + 32.0;
    let tempf = format!("{:.2}", tempff);
    let hum = reading.humidity();
    let humi = format!("{:.2}", hum);
    let date = d;
    let time = t;

    SensorData {
        tempc,
        tempf,
        humi,
        date,
        time,
    }
}

fn main() -> Result<()> {
    // Get the current date
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let date = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H:%M").to_string();
    let minute = now.minute();

    let dhtt = Dht::new(DhtType::Dht11, 2).unwrap();

    // Define the paths
    let db_path = Path::new("/usr/share/dht11rs/dht11rs/sensor_data.db");
    let db_dir = Path::new("/usr/share/dht11rs/db/");

    // Check if it's the first of the month
    if day == 1 {
        // Create the db directory if it doesn't exist
        if !db_dir.exists() {
            fs::create_dir_all(db_dir).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        }

        // Format the new file name
        let previous_month = if month == 1 {
            Local.with_ymd_and_hms(year - 1, 12, 1, 0, 0, 0).unwrap()
        } else {
            Local.with_ymd_and_hms(year, month - 1, 1, 0, 0, 0).unwrap()
        };
        let new_file_name = format!("{}{}.db", previous_month.format("%B"), previous_month.year());
        let new_file_path = db_dir.join(new_file_name);

        // Rename the old database file
        if db_path.exists() {
            fs::rename(db_path, new_file_path).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        }
    }

    // Initialize the SQLite database
    let conn = Connection::open(&db_path)?;

    // Create the tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor (
            id INTEGER PRIMARY KEY,
            tempc TEXT NOT NULL,
            tempf TEXT NOT NULL,
            humi TEXT NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensorhour (
            id INTEGER PRIMARY KEY,
            tempc TEXT NOT NULL,
            tempf TEXT NOT NULL,
            humi TEXT NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL
        )",
        [],
    )?;
    
    // let date = Local::now().format("%Y-%m-%d").to_string();
    // let time = Local::now().format("%H:%M").to_string();
    // let minute = Local::now().minute();

    if minute == 0 {
        let mut datavec:Vec<SensorData> = vec![];
        let data = read_data(dhtt, date, time);
        datavec.push(data);
        conn.execute(
            "INSERT INTO sensor (tempc, tempf, humi, date, time) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time],
        )?;
        conn.execute(
            "INSERT INTO sensorhour (tempc, tempf, humi, date, time) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time],
        )?;
    } else if minute == 15 || minute == 30 || minute == 45 {
        let mut datavec:Vec<SensorData> = vec![];
        let data = read_data(dhtt, date, time);
        datavec.push(data);
        conn.execute(
            "INSERT INTO sensor (tempc, tempf, humi, date, time) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time],
        )?;
    }

    Ok(())
}