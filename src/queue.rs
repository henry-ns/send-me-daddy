use std::time::Duration;

use kafka::{
    consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message},
    producer::{Producer, Record, RequiredAcks},
};

use serde::Serialize;
use tokio::sync::Mutex;

pub struct Queue<'a> {
    producer: Mutex<Producer>,
    host: &'a str,
}

impl Queue<'_> {
    pub fn new(host: &str) -> Queue {
        let producer = Producer::from_hosts(vec![host.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Queue {
            host,
            producer: Mutex::new(producer),
        }
    }

    pub async fn send_to(
        &self,
        topic: &str,
        data: impl Serialize,
    ) -> Result<(), kafka::Error> {
        let value = serde_json::json!(data).to_string();
        let rec = Record::from_value(topic, value.as_bytes());

        let mut producer = self.producer.lock().await;

        producer.send(&rec)
    }

    pub fn subscribe_to(&self, topic: &str, cb: fn(m: &Message<'_>) -> ()) {
        let mut consumer = Consumer::from_hosts(vec![self.host.to_owned()])
            .with_topic(topic.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();

        tokio::spawn(async move {
            loop {
                for ms in consumer.poll().unwrap().iter() {
                    ms.messages().iter().for_each(cb);
                    consumer.consume_messageset(ms).unwrap();
                }
                consumer.commit_consumed().unwrap();
            }
        });
    }
}
