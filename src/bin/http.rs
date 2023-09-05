extern crate dotenv;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use send_me_daddy::{email::Email, queue::Queue};
use std::env;

#[post("/")]
pub async fn send_email(queue: web::Data<Queue>, payload: web::Json<Email>) -> impl Responder {
    queue.send_to("email", payload.clone()).await.unwrap();

    HttpResponse::Ok().json(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let queue_host = env::var("KAFKA_HOST").ok().unwrap();
    let http_port: u16 = env::var("HTTP_PORT").ok().unwrap().parse().unwrap();

    let queue = Queue::new(queue_host);

    let app_data = web::Data::new(queue);

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(send_email))
        .bind(("127.0.0.1", http_port))?
        .run()
        .await
}
