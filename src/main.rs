mod consumer;
mod provider;

use std::vec;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use kafka::client::KafkaClient;
use serde::Serialize;

#[derive(Serialize)]
struct Hello<'a> {
    message: &'a str,
}

#[get("/")]
async fn hello() -> impl Responder {
    provider::send_email_producer();

    HttpResponse::Ok().json(Hello {
        message: "Hello word",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = KafkaClient::new(vec!["localhost:9092".to_owned()]);
    client.load_metadata_all().unwrap();

    println!("Kafka Topics:");
    for topic in client.topics() {
        for partition in topic.partitions() {
            println!("  - {} #{}", topic.name(), partition.id(),);
        }
    }

    consumer::send_email_consumer();

    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
