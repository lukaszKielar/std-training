use mqtt_messages::{hello_topic, temperature_data_topic, Command, RGB8};
use rand::Rng;
use rumqttc::{Client, Event, MqttOptions, Packet, QoS};
use std::{error::Error, thread, time::Duration};

const UUID: &str = get_uuid::uuid();

#[derive(Debug)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
}

fn main() -> Result<(), Box<dyn Error>> {
    dbg!(UUID);

    let mut mqttoptions = MqttOptions::new(UUID, CONFIG.mqtt_host, 1883);
    mqttoptions.set_credentials(CONFIG.mqtt_user, CONFIG.mqtt_pass);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe(temperature_data_topic(UUID), QoS::AtMostOnce)?;
    client.subscribe(hello_topic(UUID), QoS::AtMostOnce)?;

    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let r = rng.gen();
            let g = rng.gen();
            let b = rng.gen();
            let color = RGB8::new(r, g, b);
            println!("Setting new color: {}", color);
            let command = Command::BoardLed(color);
            client
                .publish(command.topic(UUID), QoS::AtLeastOnce, false, command.data())
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Iterate to poll the eventloop for connection progress
    for notification in connection.iter().flatten() {
        // if you want to see *everything*, uncomment:
        // println!("Notification = {:#?}", notification);

        if let Event::Incoming(Packet::Publish(publish_data)) = notification {
            if publish_data.topic == hello_topic(UUID) {
                println!("Board says hi!");
            }

            if publish_data.topic == temperature_data_topic(UUID) {
                let data: &[u8] = &publish_data.payload;
                let data: Result<[u8; 4], _> = data.try_into();

                if let Ok(data) = data {
                    let temp: f32 = f32::from_be_bytes(data);
                    println!("Board temperature: {:.2}°C", temp)
                }
            }
        }
    }
    Ok(())
}
