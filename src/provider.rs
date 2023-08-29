use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};

pub fn send_email_producer() {
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let json = serde_json::json!({
        "message": "hello world"
    });

    println!("PRODUCER");
    println!("{}", json);

    producer
        .send(&Record::from_value(
            "send_email",
            json.to_string().as_bytes(),
        ))
        .unwrap();
}
