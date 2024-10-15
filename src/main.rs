use rumqttc::{Client, MqttOptions, Publish, QoS};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::{thread, time::Duration};
use dotenv::dotenv;


// VERY RISKY!!!!! BE CAREFUL
//static TOPIC_EXECUTION: &str = "code_execution";
static TOPIC_NOTIFY: &str = "notify";
static TOPIC_SPECS: &str = "specs";
static TOPIC_LIST: [&str; 1] = [TOPIC_NOTIFY];


#[derive(Serialize, Deserialize)]
struct Notification{
    summary: String,
    body: String
}

#[derive(Serialize, Deserialize)]
struct Specs{
    free_memory: u64,
    free_swap: u64,
    used_swap: u64,
    total_swap: u64,
    physical_core_count: Option<usize>,
    total_memory: u64,
    available_memory: u64,
    cpus: Vec<CPU>,
    global_cpu_usage: f32,
}
#[derive(Serialize, Deserialize)]
struct CPU{
    name: String,
    frequency: u64,
    vendor_id: String,
    brand: String,
}

fn on_publish_packet(publish: Publish) {
    let payload = String::from_utf8(publish.payload.to_vec()).unwrap_or("Undefined".to_owned());
    if publish.topic == TOPIC_NOTIFY {
        println!("{:?}: {:?}", publish.topic, publish.payload);
        let noti: Notification = serde_json::from_str(&payload).unwrap_or(Notification{body: "".to_owned(), summary: "".to_owned()});

        notify_rust::Notification::new()
        .summary(&noti.summary)
        .body(&noti.body)
        .show().unwrap();
    }
    
}


fn main() {
    dotenv().ok();
    let ip: String = std::env::var("IP").unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();
    println!("CONNECTING TO: {ip}:{port}");
    let mut mqttoptions = MqttOptions::new("", ip, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);

    //client.subscribe(TOPIC_EXECUTION, QoS::AtLeastOnce).unwrap();
    for topic in TOPIC_LIST {
        client.subscribe(topic, QoS::AtLeastOnce).unwrap();
    }
    
    thread::spawn(move || loop {
        let sys = System::new_all();
        let specs = Specs{
            free_memory: sys.free_memory(),
            total_memory: sys.total_memory(),
            available_memory: sys.available_memory(),
            cpus: sys.cpus().into_iter().map(|cpu| {
                CPU{
                    name: cpu.name().to_owned(),
                    frequency: cpu.frequency(),
                    vendor_id: cpu.vendor_id().to_owned(),
                    brand: cpu.brand().to_owned(),
                }
            }).collect(),
            global_cpu_usage: sys.global_cpu_usage(),
            physical_core_count: sys.physical_core_count(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            free_swap: sys.free_swap(),
        };
        let json = serde_json::to_string(&specs).unwrap();
        client.publish(TOPIC_SPECS, QoS::AtLeastOnce, true, json.as_bytes()).unwrap();
        thread::sleep(Duration::from_millis(std::env::var("SPECS_INTERVAL").unwrap().parse().unwrap()));
     });


    for (_i, notification) in connection.iter().enumerate() {
        println!("{:?}", notification);

        if let Ok(event) = notification {
            match event {
                rumqttc::Event::Incoming(packet) => {
                    match packet {
                        rumqttc::Packet::Publish(publish) => {
                            on_publish_packet(publish);
                        },
                        _=> {},
                    }
                },
                _ => {},
            }
        }

    }
}
