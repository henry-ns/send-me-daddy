extern crate dotenv;

pub mod mail {
    tonic::include_proto!("mail");
}

use dotenv::dotenv;
use std::env;
use tonic::{transport::Server, Request, Response, Status};

use crate::mail::{
    mail_server::{Mail, MailServer},
    MailRequest, MailResponse,
};

#[derive(Default)]
pub struct MyMail {}

#[tonic::async_trait]
impl Mail for MyMail {
    async fn send(&self, request: Request<MailRequest>) -> Result<Response<MailResponse>, Status> {
        println!("Requested by {:?}", request.get_ref().name);

        Ok(Response::new(MailResponse {
            message: format!("hello {}", request.get_ref().name),
        }))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("GRPC_HOST").unwrap().parse().unwrap();
    let mail = MyMail::default();

    println!("Init gRPC server on {:?}", addr);

    Server::builder()
        .add_service(MailServer::new(mail))
        .serve(addr)
        .await?;

    Ok(())
}
