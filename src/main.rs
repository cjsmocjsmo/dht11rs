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
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
struct SensorData {
    tempc: String,
    tempf: String,
    humi: String,
    date: String,
    time: String,
    timestamp: String,
    cputemp: String,
}

fn read_cpu_temp() -> String {
    let output = Command::new("vcgencmd")
        .arg("measure_temp")
        .output()
        .expect("Failed to execute command");

    let temp = String::from_utf8(output.stdout).unwrap();
    let temp = temp.replace("temp=", "").replace("'C\n", "");
    let temp = temp.parse::<f32>().unwrap();
    // let temp = temp * 9.0 / 5.0 + 32.0;
    let temp = format!("{:.1}", temp);

    temp
}

fn read_data(d: String, t: String, ts: String) -> SensorData {
    let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();
    let reading = dht.read().unwrap();

    let temp = reading.temperature();
    let tempc = format!("{:.1}", temp);

    let tempff = temp * 9.0 / 5.0 + 32.0;
    let tempf = format!("{:.1}", tempff);

    let hu = reading.humidity();
    let humi = format!("{:.1}", hu);

    let date = d;
    let time = t;
    let timestamp = ts;

    let cputemp = read_cpu_temp();

    SensorData {
        tempc,
        tempf,
        humi,
        date,
        time,
        timestamp,
        cputemp,
    }
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor (
            id INTEGER PRIMARY KEY,
            tempc TEXT NOT NULL,
            tempf TEXT NOT NULL,
            humi TEXT NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            timestamp TEXT NOT NULL UNIQUE,
            cputemp TEXT NOT NULL
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
            time TEXT NOT NULL,
            timestamp TEXT NOT NULL UNIQUE,
            cputemp TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

fn rotate_db_file(year: u32, month: u32, day: u32, db_path: String, db_dir: String) -> Result<(), rusqlite::Error> {
   
    // Check if it's the first of the month
    if day == 1 {
        // Create the db directory if it doesn't exist
        if !Path::new(&db_dir).exists() {
            fs::create_dir_all(db_dir.clone()).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        }

        // Format the new file name
        let previous_month = if month == 1 {
            Local.with_ymd_and_hms((year - 1) as i32, 12, 1, 0, 0, 0).unwrap()
        } else {
            Local.with_ymd_and_hms(year as i32, (month - 1) as u32, 1, 0, 0, 0).unwrap()
        };
        let new_file_name = format!("{}{}.db", previous_month.format("%B"), previous_month.year());
        let new_file_path = Path::new(&db_dir).join(new_file_name);

        // Rename the old database file
        if Path::new(&db_dir).exists() {
            fs::rename(db_path, new_file_path).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let foo = true;
    while foo {
        let now = Local::now();
        let year = now.year() as u32;
        let month = now.month();
        let day = now.day();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M").to_string();
        let timestamp = now.format("%Y-%m-%d-%H:%M:%S").to_string();
        let minute = now.minute();
        let second = now.second();

        let db_path = Path::new("/usr/share/dht11rs/dht11rs/sensor_data.db");
        let db_dir = Path::new("/usr/share/dht11rs/db/").to_path_buf();

        let _ = rotate_db_file(year, month, day, db_path.to_str().unwrap().to_string(), db_dir.to_str().unwrap().to_string())?;

        let conn = Connection::open(&db_path)?;
        let _ = create_tables(&conn)?;

    
    
        if minute == 0 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            let data = read_data(date.clone(), time.clone(), timestamp.clone());
            datavec.push(data);
            conn.execute(
                "INSERT INTO sensor (tempc, tempf, humi, date, time, timestamp, cputemp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp, datavec[0].cputemp],
            )?;
            conn.execute(
                "INSERT INTO sensorhour (tempc, tempf, humi, date, time, timestamp, cputemp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp, datavec[0].cputemp],
            )?;
        } else if minute == 15 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            let data = read_data(date.clone(), time.clone(), timestamp.clone());
            datavec.push(data);
            conn.execute(
                "INSERT INTO sensor (tempc, tempf, humi, date, time, timestamp, cputemp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp, datavec[0].cputemp],
            )?;
        } else if minute == 30 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            let data = read_data(date.clone(), time.clone(), timestamp.clone());
            datavec.push(data);
            conn.execute(
                "INSERT INTO sensor (tempc, tempf, humi, date, time, timestamp, cputemp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp, datavec[0].cputemp],
            )?;
        } else if minute == 45 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            let data = read_data(date.clone(), time.clone(), timestamp.clone());
            datavec.push(data);
            conn.execute(
                "INSERT INTO sensor (tempc, tempf, humi, date, time, timestamp, cputemp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![datavec[0].tempc, datavec[0].tempf, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp, datavec[0].cputemp],
            )?;
            
        }
    }

    Ok(())
}