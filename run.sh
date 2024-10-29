#! /bin/sh

# if [ -d /usr/systemd/system/dht11rs.service ]; then
#     sudo systemctl stop dht11rs.service;
# fi
current_dir=$(pwd)

echo $current_dir

# if [ $current_dir === "/usr/share/dht11res/dht11rs" ]; then
#     git pull;
#     cargo build --release;
#     sudo cp -pvr /usr/share/dht11res/dht11res/target/release/dht11res /usr/bin/dht11res;
#     rm /usr/share/dht11res/dht11rs/sensor_data.db;
#     touch /usr/share/dht11res/dht11rs/sensor_data.db;
#     # sudo systemctl daemon-reload;
#     # sudo systemctl start dht11rs.service;
# else
#     cd /usr/share/dht11res;
#     git clone https://github.com/cjsmocjsmo/dht11rs.git;
#     cd dht11rs;
#     cargo build --release;
#     sudo cp -pvr /usr/share/dht11res/dht11rs/target/release/dht11res /usr/bin/dht11res;
#     rm /usr/share/dht11res/dht11rs/sensor_data.db;
#     touch /usr/share/dht11res/dht11rs/sensor_data.db;
#     # sudo systemctl daemon-reload;
#     # sudo systemctl start dht11rs.service;
# fi




