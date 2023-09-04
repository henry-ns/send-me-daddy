use actix_web::{post, web, HttpResponse, Responder};

use crate::{email::Email, queue::Queue};

#[post("/")]
pub async fn send_email(queue: web::Data<Queue>, payload: web::Json<Email>) -> impl Responder {
    queue.send_to("email", payload.clone()).await.unwrap();

    HttpResponse::Ok().json(payload)
}
