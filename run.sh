#! /bin/sh

# if [ -d /usr/systemd/system/dht11rs.service ]; then
#     sudo systemctl stop dht11rs.service;
# fi

git pull;
cargo build --release;
sudo cp -pvr /usr/share/dht11res/dht11res/target/release/dht11res /usr/bin/dht11res;
rm /usr/share/dht11res/dht11rs/sensor_data.db;
touch /usr/share/dht11res/dht11rs/sensor_data.db;
# sudo systemctl daemon-reload;
# sudo systemctl start dht11rs.service;





