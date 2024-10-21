use dotenv::dotenv;
use publish::PublishQueries;
use rumqttc::{Client, MqttOptions};
use simple_logger::SimpleLogger;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use subscribe::Subscribe;
mod publish;
mod subscribe;
mod topics;

fn main() {
    dotenv().ok();
    SimpleLogger::new().init().unwrap();
    let ip: String = std::env::var("IP").expect("IP environment variable must be set");
    let port: u16 = std::env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse()
        .expect("PORT must be a number");
    let device_id: String =
        std::env::var("DEVICE_ID").expect("DEVICE_ID environment variable must be set");
    println!("CONNECTING TO: {ip}:{port}");
    let mut mqttoptions = MqttOptions::new("", ip, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    let arc_mutex_client = Arc::new(Mutex::new(client));

    PublishQueries::new(device_id.clone())
    .add_query_once(Box::new(publish::publish_specs::OnceSpecs))
        .add_query_once(Box::new(publish::publish_specs::OnceSpecs))
        .add_query(Box::new(publish::publish_specs::UpdateSpecs))
        .add_query(Box::new(publish::publish_mpris::UpdateMPRIS))
        .add_query(Box::new(publish::publish_last_update::LastUpdate))
        .execute(arc_mutex_client.clone());

    let subs = Subscribe::new(device_id)
        .add_query(
            topics::TOPIC_NOTIFY.to_owned(),
            subscribe::notification::on_notification_request,
        )
        .add_query(
            topics::TOPIC_SLEEP.to_owned(),
            subscribe::shutdown::on_sleep_request,
        )
        .subscribe(arc_mutex_client.clone());

    //basically a while there are new packets arriving
    for (_i, notification) in connection.iter().enumerate() {
        if let Ok(event) = notification {
            if let rumqttc::Event::Incoming(packet) = event {
                if let rumqttc::Packet::Publish(publish) = packet {
                    subs.execute(arc_mutex_client.clone(), publish);
                }
            }
        }
    }
}
