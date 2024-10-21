use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{publish, topics};

use super::IPublishQuery;

#[derive(Serialize)]
struct LastUpdateBody{
    last_update: f64
}

publish!(LastUpdate, topics::TOPIC_UPDATE.to_string(), QoS::AtLeastOnce, true, || -> Vec<u8> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let body = LastUpdateBody{
        last_update: since_the_epoch.as_secs_f64()
    };
    return serde_json::to_string(&body).unwrap().into_bytes().to_vec();
});