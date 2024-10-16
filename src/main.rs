use publish::PublishQueries;
use rumqttc::{Client, MqttOptions};
use subscribe::Subscribe;
use std::{sync::{Arc, Mutex}, time::Duration};
use dotenv::dotenv;
mod topics;
mod subscribe;
mod publish;

fn main() {
    dotenv().ok();
    let ip: String = std::env::var("IP").unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();
    println!("CONNECTING TO: {ip}:{port}");
    let mut mqttoptions = MqttOptions::new("", ip, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    let arc_mutex_client = Arc::new(Mutex::new(client));

    PublishQueries::new()
        .add_query(publish::publish_specs::publish_specs)
        .execute(arc_mutex_client.clone());

    let subs = Subscribe::new()
        .add_query(topics::TOPIC_NOTIFY.to_owned(), subscribe::notification::on_notification_request)
        .subscribe(arc_mutex_client.clone());
    

    //basically a while there are new packets arriving
    for (_i, notification) in connection.iter().enumerate() {

        if let Ok(event) = notification {
            if let rumqttc::Event::Incoming(packet) = event{
                if let  rumqttc::Packet::Publish(publish) = packet {
                    subs.execute(arc_mutex_client.clone(), publish);
                }
            }
        }

    }
}
