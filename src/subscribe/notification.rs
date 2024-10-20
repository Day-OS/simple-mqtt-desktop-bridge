use std::sync::{Arc, Mutex};

use rumqttc::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Notification {
    summary: String,
    body: String,
}

pub fn on_notification_request(_: Arc<Mutex<Client>>, payload: String) {
    let noti: Notification = serde_json::from_str(&payload).unwrap_or(Notification {
        body: "".to_owned(),
        summary: "".to_owned(),
    });
    notify_rust::Notification::new()
        .summary(&noti.summary)
        .body(&noti.body)
        .show()
        .unwrap();
}
