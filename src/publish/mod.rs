use rumqttc::{Client, QoS};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
pub(crate) mod publish_specs;
pub(crate) mod publish_mpris;
pub(crate) mod publish_last_update;



#[macro_export]
macro_rules! publish {
    ($struct_name:ident, $topic:expr, $qos:expr, $retain:expr, $func:expr) => {
        pub(crate) struct $struct_name;

        impl IPublishQuery for $struct_name{
            fn get_topic_name(&self) -> String{
                $topic
            }
            fn process_data(&self) -> Vec<u8>{
                $func()
            }
            fn get_qos(&self) -> QoS{
                $qos
            }
            fn retain(&self) -> bool{
                $retain
            }
        }
    }
}

pub(crate) trait IPublishQuery: Send{
    fn get_topic_name(&self) -> String;
    fn process_data(&self) -> Vec<u8>;
    fn get_qos(&self) -> QoS;
    fn retain(&self) -> bool;
    fn publish_query(&self, client: Arc<Mutex<Client>>, device_id: String) {
        match client.try_lock() {
            Ok(client) => {
                let topic = format!("{}-{}", self.get_topic_name(), device_id);
                log::info!("Publishing to {topic}");
                if let Err(e) = client.publish(
                    topic,
                    self.get_qos(),
                    self.retain(),
                    self.process_data(),
                ) {
                    log::error!("{e}");
                }
            }
            Err(e) => log::error!("{e}"),
        }
    }
}



pub struct PublishQueries {
    device_id: String,
    // queries that will executed only ONCE
    once_queries: Vec<Box<dyn IPublishQuery>>,
    queries: Vec<Box<dyn IPublishQuery>>,
}

impl PublishQueries {
    pub fn new(device_id: String) -> Self {
        PublishQueries {
            device_id,
            queries: vec![],
            once_queries: vec![],
        }
    }

    pub fn add_query_once(mut self, query: Box<dyn IPublishQuery>) -> Self {
        self.once_queries.push(query);
        return self;
    }

    pub fn add_query(mut self, query: Box<dyn IPublishQuery>) -> Self {
        self.queries.push(query);
        return self;
    }

    pub fn execute(self, client: Arc<Mutex<Client>>) {
        log::info!("Executing publish queries");
        let specs_interval: u64 = std::env::var("SPECS_INTERVAL").unwrap().parse().unwrap();
        for query in &self.once_queries {
            query.publish_query(client.clone(), self.device_id.clone());
        }
        log::info!("Starting loop queries thread");
        thread::spawn(move || loop {
            for query in &self.queries {
                query.publish_query(client.clone(), self.device_id.clone());
            }

            thread::sleep(Duration::from_millis(specs_interval));
        });
    }
}
