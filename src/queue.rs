use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};

use serde::Serialize;
use tokio::sync::Mutex;

pub struct Queue {
    producer: Mutex<Producer>,
}

impl Queue {
    pub fn new(host: String) -> Queue {
        let hosts = vec![host.to_owned()];

        let producer = Producer::from_hosts(hosts.to_owned())
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Queue {
            producer: Mutex::new(producer),
        }
    }

    pub async fn send_to(&self, topic: &str, data: impl Serialize) -> Result<(), kafka::Error> {
        let value = serde_json::json!(data).to_string();
        let rec = Record::from_value(topic, value.as_bytes());

        let mut producer = self.producer.lock().await;

        producer.send(&rec)
    }
}
