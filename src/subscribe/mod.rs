use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use rumqttc::{Client, Publish, QoS};
pub(crate) mod notification;
pub(crate) mod shutdown;

type SubscribeFn = fn(client: Arc<Mutex<Client>>, payload: String);

pub struct Subscribe {
    device_id: String,
    subscriptions: HashMap<String, SubscribeFn>,
}

impl Subscribe {
    pub fn new(device_id: String) -> Self {
        Subscribe {
            device_id,
            subscriptions: HashMap::new(),
        }
    }

    pub fn add_query(mut self, topic: String, _fn: SubscribeFn) -> Self {
        self.subscriptions.insert(topic.clone(), _fn);
        self.subscriptions.insert(format!("{topic}-{}", self.device_id), _fn);
        return self;
    }

    /// Try to find any existing topic and execute its functions
    pub fn execute(&self, client: Arc<Mutex<Client>>, publish: Publish) {
        match self.subscriptions.get(&publish.topic) {
            Some(subscription) => {
                let payload: String =
                    String::from_utf8(publish.payload.to_vec()).unwrap_or("Undefined".to_owned());
                log::debug!("{:?}: {:?}", publish.topic, payload);
                
                subscription(client, payload)
            }
            None => {
                log::debug!("no method found for {}", publish.topic)
            },
        }
    }

    pub fn subscribe(self, client: Arc<Mutex<Client>>) -> Self {
        match client.lock() {
            Ok(client) => {
                for (topic, _) in self.subscriptions.iter() {
                    client.subscribe(topic, QoS::AtLeastOnce).unwrap();
                    client.subscribe(format!("{topic}-{}", self.device_id), QoS::AtLeastOnce).unwrap();
                }
            }
            Err(e) => log::error!("{e}"),
        }
        self
    }
}
