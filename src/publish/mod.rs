use std::{sync::{Arc, Mutex}, thread, time::Duration};
use rumqttc::Client;
pub(crate) mod publish_specs;


type PublishFn = fn(client: Arc<Mutex<Client>>);

pub struct PublishQueries{
    queries: Vec<PublishFn>
}

impl PublishQueries {
    pub fn new() -> Self{
        PublishQueries{
            queries: vec![]
        }
    }

    pub fn add_query(mut self, _fn: PublishFn) -> Self{
        self.queries.push(_fn);
        return self;
    }

    pub fn execute(self, client: Arc<Mutex<Client>>){
        let specs_interval: u64 = std::env::var("SPECS_INTERVAL").unwrap().parse().unwrap();
        thread::spawn(move || loop {

            for function in &self.queries {
                function(client.clone())
            }
        
            thread::sleep(Duration::from_millis(specs_interval));
        });
    }
}
