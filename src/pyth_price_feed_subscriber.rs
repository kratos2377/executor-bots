use std::collections::HashMap;

use reqwest_eventsource::EventSource;



pub struct PythPriceFeedSubscriber {
    pub latest_pyth_vaas: HashMap<String , String>,
    pub endpoint: String,
    pub connection: EventSource
    // redis connection
}

impl PythPriceFeedSubscriber {
    fn new(endpoint: String) -> Self {
        Self {
            latest_pyth_vaas: HashMap::new(),
            endpoint: endpoint.clone(),
            connection: EventSource::get(endpoint)
        }
    }

   async fn subscribe(&self, feedsIds: Vec<String>) {
        todo!()
    }


    
}