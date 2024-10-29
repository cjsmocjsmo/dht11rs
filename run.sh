#! /bin/sh

# if [ -d /usr/systemd/system/dht11rs.service ]; then
#     sudo systemctl stop dht11rs.service;
# fi


    cd /usr/share/dht11res/dht11res;
    git pull;
    cargo build --release;
    sudo cp -pvr ./target/release/dht11res /usr/bin/dht11res;
    rm ./sensor_data.db;
    touch ./sensor_data.db;
    # sudo systemctl daemon-reload;
    # sudo systemctl start dht11rs.service;
