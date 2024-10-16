use std::sync::{Arc, Mutex};
use rumqttc::{Client, QoS};
use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::topics;


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

pub fn publish_specs(client: Arc<Mutex<Client>>) {
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
    match client.try_lock() {
        Ok(client) => {
            client.publish(topics::TOPIC_SPECS, QoS::AtLeastOnce, true, json.as_bytes()).unwrap();
        },
        Err(e) => log::error!("{e}"),
    }
}