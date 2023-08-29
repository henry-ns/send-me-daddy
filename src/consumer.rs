use serde::Deserialize;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

#[derive(Deserialize, Debug)]
struct Message<'a> {
    message: &'a str
}

pub fn send_email_consumer() {
    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic("send_email".to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    tokio::spawn(async move {
        loop {
            for ms in consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    let msg = std::str::from_utf8(m.value);

                    match msg {
                        Err(_) => println!("can't pass the value"),
                        Ok(value) => {
                            let json: Message = serde_json::from_str(value).unwrap();
                            println!("CONSUMER - {:?}", json.message);
                        }
                    }

                }

                consumer.consume_messageset(ms).unwrap();
            }

            consumer.commit_consumed().unwrap();
        }
    });
}
