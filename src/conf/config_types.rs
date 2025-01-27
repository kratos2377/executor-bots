use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct BrokerProperties {
    pub urls: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConsumerConfiguration {
    pub id: String,
    pub topic: Vec<String>,
    pub client_id: String,
    pub group_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SchemaRegistryProperties {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct TopicConfiguration {
    pub mappings: Vec<TopicProperties>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct TopicProperties {
    pub id: String,
    pub topic_name: String,
    pub partitions: i32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConfiguration {
    pub broker: BrokerProperties,
    pub consumer: Vec<ConsumerConfiguration>,
    pub producer: ProducerProperties,
    pub topic: TopicConfiguration,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ProducerProperties {
    pub client_id: String,
    pub transactional_id: String,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfiguration {
    pub port: u16,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ExecutorsConfiguration {
    pub number_of_executors: usize,
}


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct EventQueueConfig {
    pub queue_size: usize,
}









// Impl Methods for structs
impl TopicConfiguration {
    pub fn get_mapping(&self, id: &str) -> TopicProperties {
        let mapping: Vec<TopicProperties> = self
            .mappings
            .clone()
            .into_iter()
            .filter(|t| t.id == id)
            .collect();

        mapping
            .first()
            .unwrap_or_else(|| panic!("{} topic configuration not found", id))
            .clone()
    }
}