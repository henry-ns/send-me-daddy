extern crate dotenv;

pub mod mail {
    tonic::include_proto!("mail");
}

use dotenv::dotenv;
use send_me_daddy::{email::Email, queue::Queue};
use std::env;
use tonic::{transport::Server, Code, Request, Response, Status};

use crate::mail::{
    mail_server::{Mail, MailServer},
    MailRequest, MailResponse,
};

pub struct MyMail {
    queue: Queue,
}

impl MyMail {
    fn new(host: String) -> MyMail {
        MyMail {
            queue: Queue::new(host),
        }
    }
}

#[tonic::async_trait]
impl Mail for MyMail {
    async fn send(&self, request: Request<MailRequest>) -> Result<Response<MailResponse>, Status> {
        println!("Requested by {:?}", request.get_ref().name);

        let topic = env::var("KAFKA_TOPIC").unwrap();
        
        let result = self
            .queue
            .send_to(
                &topic,
                Email {
                    host: "test".to_owned(),
                    body: "body".to_owned(),
                },
            )
            .await;

        match result {
            Ok(_) => Ok(Response::new(MailResponse {
                message: format!("Success {}", request.get_ref().name),
            })),
            Err(e) => Err(Status::new(Code::Internal, format!("Error {}", e))),
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let queue_host = env::var("KAFKA_HOST").unwrap();
    let addr = env::var("GRPC_HOST").unwrap().parse().unwrap();
    let mail = MyMail::new(queue_host);

    println!("Init gRPC server on {:?}", addr);

    Server::builder()
        .add_service(MailServer::new(mail))
        .serve(addr)
        .await?;

    Ok(())
}
