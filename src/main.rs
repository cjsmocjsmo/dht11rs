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

use dht_mmap_rust::{Dht, DhtType};
use rusqlite::{params, Connection, Result};
use std::path::Path;
use chrono::Local;

fn main() -> Result<()> {
    // Initialize the SQLite database
    let path = Path::new("/usr/share/dht11rs/dht11rs/sensor_data.db");
    let conn = Connection::open(&path)?;

    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor (
            id INTEGER PRIMARY KEY,
            tempc TEXT NOT NULL,
            tempf TEXT NOT NULL,
            humi REAL NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL
        )",
        [],
    )?;

    // The sensor is a DHT11 connected on pin 23
    let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

    // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
    // the read succeeds.
    // For more information, see documentation on `read()`
    let foo = true;
    while foo {
        let reading = dht.read().unwrap();

        let temp = reading.temperature();
        let tempc = format!("{:.2}", temp);
        let tempff = temp * 9.0 / 5.0 + 32.0;
        let tempf = format!("{:.2}", tempff);
        let humi = reading.humidity();
        let date = Local::now().format("%Y-%m-%d").to_string();
        let time = Local::now().format("%H:%M").to_string();
        
        // Insert the data into the database
        conn.execute(
            "INSERT INTO sensor (tempc, tempf, humi, date, time) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tempc, tempf, humi, date, time],
        )?;

        // println!("Temperature (C): {}", tempc);
        // println!("Temperature (F): {}", tempf);
        // println!("Humidity: {:.2}%", humi);
        // println!("Timestamp: {}", timestamp);
        // println!("Idx: {}", idx);

        std::thread::sleep(std::time::Duration::from_secs(300));
        }

    Ok(())
}