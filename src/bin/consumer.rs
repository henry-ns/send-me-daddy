extern crate dotenv;

use std::env;

use dotenv::dotenv;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use send_me_daddy::email::Email;

#[path = "../sender.rs"]
mod sender;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let topic = env::var("KAFKA_TOPIC").unwrap();
    let host = env::var("KAFKA_HOST").unwrap();

    let mut consumer = Consumer::from_hosts(vec![host])
        .with_topic(topic)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("email-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            ms.messages().iter().for_each(|m| {
                let msg = std::str::from_utf8(m.value);

                match msg {
                    Err(_) => println!("can't pass the value"),
                    Ok(value) => {
                        let data: Email = serde_json::from_str(value).unwrap();
                        sender::send_email(data.receiver.clone(), data.subject.clone(), data.body.clone());

                        println!("CONSUMER - {:?}", data);
                    }
                }
            });

             consumer.consume_messageset(ms).unwrap();
        }
       consumer.commit_consumed().unwrap();
    }
}
