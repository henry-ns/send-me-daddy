mod queue;

use crate::queue::Queue;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use kafka::client::KafkaClient;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy)]
struct Hello<'a> {
    message: &'a str,
}

#[get("/")]
async fn hello(queue: web::Data<Queue<'_>>) -> impl Responder {
    let msg = Hello {
        message: "Hello word",
    };

    queue.send_to("email", msg).await.unwrap();

    HttpResponse::Ok().json(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "localhost:9092";
    let mut client = KafkaClient::new(vec![host.to_owned()]);
    client.load_metadata_all().unwrap();

    println!("Kafka Topics:");
    for topic in client.topics() {
        for partition in topic.partitions() {
            println!("  - {} #{}", topic.name(), partition.id(),);
        }
    }

    let queue = Queue::new(host);

    queue.subscribe_to("email", |m| {
        let msg = std::str::from_utf8(m.value);

        match msg {
            Err(_) => println!("can't pass the value"),
            Ok(value) => {
                let json: Hello = serde_json::from_str(value).unwrap();
                println!("CONSUMER - {:?}", json.message);
            }
        }
    });

    let app_data = web::Data::new(queue);

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
