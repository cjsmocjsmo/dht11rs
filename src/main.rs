use chrono::{Local, Timelike};
use dht_mmap_rust::{Dht, DhtType};
use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::process::Command;
use std::str;

#[derive(Debug)]
struct SensorData {
    tempc: String,
    tempf: String,
    humi: String,
    date: String,
    time: String,
    timestamp: String,
}

fn outside_temp() -> String {
    let output = Command::new("python3")
        .arg("/usr/share/dht11rs/dht11rs/outtemp.py")
        // .arg(47.37849)
        // .arg("-122.94207")
        .output()
        .expect("Failed to execute Python script");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Failed to parse output");
        println!("{}", stdout);
        // stdout.trim().parse::<f32>().expect("Failed to parse temperature")
        stdout.trim().to_string()
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Failed to parse error output");
        panic!("Python script error: {}", stderr);
    }
}

fn read_data(d: String, t: String, ts: String) -> Result<SensorData, String> {

    let mut dht = match Dht::new(DhtType::Dht11, 2) {
        Ok(dht) => dht,
        Err(e) => return Err(format!("Failed to create DHT sensor: {:?}", e)),
    };

    let reading = match dht.read() {
        Ok(reading) => reading,
        Err(e) => return Err(format!("Failed to read from DHT sensor: {:?}", e)),
    };

    let temp = reading.temperature();
    println!("Temperature: {:.1}°C", temp);
    let tempc = format!("{:.1}", temp);

    let tempff = temp * 9.0 / 5.0 + 32.0;
    let tempf = format!("{:.1}", tempff);

    let hu = reading.humidity();
    let humi = format!("{:.1}", hu);

    let date = d;
    let time = t;
    let timestamp = ts;

    let sensor_data = SensorData {
        tempc,
        tempf,
        humi,
        date,
        time,
        timestamp,
    };

    println!("\n{:?}", sensor_data);

    Ok(sensor_data)
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor (
            id INTEGER PRIMARY KEY,
            tempc TEXT NOT NULL,
            tempf TEXT NOT NULL,
            tempo TEXT NOT NULL,
            humi TEXT NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            timestamp TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensorhour (
            id INTEGER PRIMARY KEY,
            tempc TEXT NOT NULL,
            tempf TEXT NOT NULL,
            tempo TEXT NOT NULL,
            humi TEXT NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            timestamp TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let db_path = Path::new("/usr/share/dht11rs/dht11rs/sensor_data.db");
    let conn = Connection::open(&db_path)?;
    let _ = create_tables(&conn)?;

    let foo = true;
    while foo {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M").to_string();
        let timestamp = now.format("%Y-%m-%d-%H:%M:%S").to_string();
        let minute = now.minute();
        let second = now.second();

        let outside_temp = outside_temp();

    
    
        if minute == 0 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone()) {
                Ok(data) => {
                    datavec.push(data);
                    conn.execute(
                        "INSERT OR IGNORE INTO sensor (tempc, tempf, tempo, humi, date, time, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![datavec[0].tempc, datavec[0].tempf, outside_temp, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp],
                    )?;
                    conn.execute(
                    "INSERT OR IGNORE INTO sensorhour (tempc, tempf, tempo, humi, date, time, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![datavec[0].tempc, datavec[0].tempf, outside_temp, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp],
            )?;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        } else if minute == 15 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone()) {
                Ok(data) => {
                    datavec.push(data);
                    conn.execute(
                        "INSERT OR IGNORE INTO sensor (tempc, tempf, tempo, humi, date, time, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![datavec[0].tempc, datavec[0].tempf, outside_temp, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp],
                    )?;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        } else if minute == 30 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone()) {
                Ok(data) => {
                    datavec.push(data);
                    conn.execute(
                        "INSERT OR IGNORE INTO sensor (tempc, tempf, tempo, humi, date, time, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![datavec[0].tempc, datavec[0].tempf, outside_temp, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp],
                    )?;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        } else if minute == 45 && second == 0 {
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone()) {
                Ok(data) => {
                    datavec.push(data);
                    conn.execute(
                        "INSERT OR IGNORE INTO sensor (tempc, tempf, tempo, humi, date, time, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![datavec[0].tempc, datavec[0].tempf, outside_temp, datavec[0].humi, datavec[0].date, datavec[0].time, datavec[0].timestamp],
                    )?;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }

    Ok(())
}