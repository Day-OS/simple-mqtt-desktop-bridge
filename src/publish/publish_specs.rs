use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::{publish, topics};

use super::IPublishQuery;

#[derive(Serialize, Deserialize)]
struct OnceSpecsStruct {
    total_swap: u64,
    total_memory: u64,
    physical_core_count: Option<usize>,
    cpus: Vec<CPU>,
}


#[derive(Serialize, Deserialize)]
struct SpecsStruct {
    free_memory: u64,
    free_swap: u64,
    used_swap: u64,
    available_memory: u64,
    global_cpu_usage: f32,
    used_memory: u64,
}
#[derive(Serialize, Deserialize)]
struct CPU {
    name: String,
    frequency: u64,
    vendor_id: String,
    brand: String,
}
publish!(OnceSpecs, topics::ONCE_TOPIC_SPECS.to_string(), QoS::AtLeastOnce, true, || -> Vec<u8> {
    let sys = System::new_all();
    let specs = OnceSpecsStruct {
        total_memory: sys.total_memory(),
        cpus: sys
            .cpus()
            .into_iter()
            .map(|cpu| CPU {
                name: cpu.name().to_owned(),
                frequency: cpu.frequency(),
                vendor_id: cpu.vendor_id().to_owned(),
                brand: cpu.brand().to_owned(),
            })
            .collect(),
        physical_core_count: sys.physical_core_count(),
        total_swap: sys.total_swap(),
    };
    let json = serde_json::to_string(&specs).unwrap();
    return json.as_bytes().to_vec();
});


publish!(UpdateSpecs, topics::TOPIC_SPECS.to_string(), QoS::AtLeastOnce, true, || -> Vec<u8> {
    let sys = System::new_all();
    let specs = SpecsStruct {
        free_memory: sys.free_memory(),
        available_memory: sys.available_memory(),
        global_cpu_usage: sys.global_cpu_usage(),
        used_swap: sys.used_swap(),
        free_swap: sys.free_swap(),
        used_memory: sys.used_memory(),
    };
    let json = serde_json::to_string(&specs).unwrap();
    return json.as_bytes().to_vec();
});
