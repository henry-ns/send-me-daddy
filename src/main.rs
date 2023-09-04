extern crate dotenv;

mod email;
mod http;
mod queue;

use crate::email::Email;
use crate::queue::Queue;

use actix_web::{web, HttpServer, App};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let kafka_host = env::var("KAFKA_HOST").ok().unwrap();
    let http_port: u16 = env::var("HTTP_PORT").ok().unwrap().parse().unwrap();

    let queue = Queue::new(kafka_host);

    queue.subscribe_to("email", |m| {
        let msg = std::str::from_utf8(m.value);

        match msg {
            Err(_) => println!("can't pass the value"),
            Ok(value) => {
                let data: Email = serde_json::from_str(value).unwrap();
                println!("CONSUMER - {:?}", data);
            }
        }
    });

    let app_data = web::Data::new(queue);

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(http::send_email))
        .bind(("127.0.0.1", http_port))?
        .run()
        .await
}
