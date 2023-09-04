extern crate dotenv;

mod queue;

use crate::queue::Queue;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Clone, Copy)]
struct Hello<'a> {
    message: &'a str,
}

#[get("/")]
async fn hello(queue: web::Data<Queue>) -> impl Responder {
    let msg = Hello {
        message: "Hello word",
    };

    queue.send_to("email", msg).await.unwrap();

    HttpResponse::Ok().json(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let kafka_host = env::var("KAFKA_HOST").ok().unwrap();
    let http_host = env::var("HTTP_HOST").ok().unwrap();

    let queue = Queue::new(kafka_host);

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
        .bind(http_host)?
        .run()
        .await
}
