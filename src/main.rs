extern crate chrono;
extern crate rumqtt;
use std::env;

use rumqtt::{MqttCallback, MqttClient, MqttOptions, QoS};


fn main() {
    let client_id = env::var("MQTT_ID").unwrap_or("".to_string());
    let username = env::var("MQTT_USERNAME").unwrap_or("".to_string());
    let secret = env::var("MQTT_SECRET").unwrap_or("".to_string());
    let broker = env::var("MQTT_BROKER").unwrap_or("localhost:1883".to_string());
    let client_options = MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(0)
        .set_pub_q_len(1024)
        .set_client_id(client_id.as_ref())
        .set_user_name(username.as_ref())
        .set_password(secret.as_ref())
        .set_will_qos(QoS::Level2)
        .set_broker(&broker)
        .set_will_retain(true);

    let callback = |msg| {
        println!("Received payload: {:?}", msg);
    };
    let mq_cbs = MqttCallback::new().on_message(callback);
    println!("Start mq_client");
    let mut mq_client = MqttClient::start(client_options, Some(mq_cbs)).expect("Coudn't start");
    println!("Start mq_client");

    let mut i = 0;
    loop {
        println!("TEST {}", i);
        i+=1;
        mq_client
            .publish(
                &format!("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40"),

                QoS::Level1,
                format!("{} {}", 0.0, 1.0).into_bytes(),
                )
            .expect("Couldn't publish");
    }
}


