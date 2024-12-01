#! /bin/sh

sudo systemctl stop dht11rs.service;
cd /usr/share/dht11rs/dht11rs;
git pull;
cargo build --release;
sudo cp -pvr /usr/share/dht11rs/dht11rs/target/release/dht11rs /usr/bin/dht11rs;
rm /usr/share/dht11rs/dht11rs/sensor_data.db;
touch /usr/share/dht11rs/dht11rs/sensor_data.db;
sudo systemctl daemon-reload;
sudo systemctl start dht11rs.service;





