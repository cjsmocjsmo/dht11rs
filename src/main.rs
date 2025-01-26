use chrono::{Local, Timelike};
use dht_mmap_rust::{Dht, DhtType};
use rusqlite::{params, Connection, Result};
use std::path::Path;
use reqwest;
use serde::Deserialize;

// use std::process::Command;
// use std::str;

#[derive(Debug)]
struct SensorData {
    tempc: String,
    tempf: String,
    tempo: String,
    humi: String,
    date: String,
    time: String,
    timestamp: String,
}

#[derive(Deserialize, Debug)]
struct OpenMeteoResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
}

// fn current_temp() -> Result<f64, Box<dyn std::error::Error>> {
fn current_temp() -> Result<String, Box<dyn std::error::Error>> {
    let latitude = 47.37349;
    let longitude = -122.94207;
    
    // Construct Open-Meteo API URL
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        latitude, longitude
    );

    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err("Request failed".into());
    }

    let weather: OpenMeteoResponse = response.json()?;
    let outsidetemp = weather.current_weather.temperature;
    let outtemp = outsidetemp * 9.0 / 5.0 + 32.0;
    let outtempf = format!("{:.1}", outtemp);
    Ok(outtempf)
    // Ok(weather.current_weather.temperature)
}

// fn outside_temp() -> String {
//     let base_url = "https://api.weather.gov/points";
//     let latitude = 47.37849;
//     let longitude = -122.94207;
//     let url = format!("{}/{},{}", base_url, latitude, longitude);
//     println!("URL: {}", url);

//     let client = reqwest::blocking::Client::new();
//     let res = client.get(url).send().unwrap();
//     println!("Response: {:?}", res);
//     let json: serde_json::Value = res.json().unwrap();
//     let forecast_url = json["properties"]["forecast"].as_str().unwrap();

//     forecast_url.to_string()
// }

fn read_data(d: String, t: String, ts: String, ot: String) -> Result<SensorData, String> {

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
    let tempo = ot;

    let sensor_data = SensorData {
        tempc,
        tempf,
        tempo, 
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
    let db_path = Path::new("/usr/share/dht11rs/sensor_data.db");
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
    
        if minute == 0 && second == 0 {
            let outside_temp = match current_temp() {
                Ok(temp) => temp,
                Err(e) => {
                    eprintln!("Failed to get outside temperature: {}", e);
                    continue;
                }
            };
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone(), outside_temp.clone()) {
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
            let outside_temp = match current_temp() {
                Ok(temp) => temp,
                Err(e) => {
                    eprintln!("Failed to get outside temperature: {}", e);
                    continue;
                }
            };
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone(), outside_temp.clone()) {
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
            let outside_temp = match current_temp() {
                Ok(temp) => temp,
                Err(e) => {
                    eprintln!("Failed to get outside temperature: {}", e);
                    continue;
                }
            };
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone(), outside_temp.clone()) {
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
            let outside_temp = match current_temp() {
                Ok(temp) => temp,
                Err(e) => {
                    eprintln!("Failed to get outside temperature: {}", e);
                    continue;
                }
            };
            let mut datavec:Vec<SensorData> = vec![];
            match read_data(date.clone(), time.clone(), timestamp.clone(), outside_temp.clone()) {
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