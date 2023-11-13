extern crate dotenv;

use dotenv::dotenv;
use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


pub fn send_email(receiver: String, subject: String, body: String) {
  dotenv().ok();

  let smtp_user = env::var("SMTP_USER").ok().unwrap();
  let smtp_password = env::var("SMTP_PASSWORD").ok().unwrap();


  let sender: String = format!("Sender <{}>", smtp_user);
  let to: String = format!("Receiver <{}>", receiver);

  let email = Message::builder()
      .from(sender.parse().unwrap())
      .to(to.parse().unwrap())
      .subject(subject)
      .body(body)
      .unwrap();

  let creds = Credentials::new(smtp_user, smtp_password);
  
  let mailer = SmtpTransport::relay("smtp.gmail.com").unwrap().credentials(creds).build();

  match mailer.send(&email) {
      Ok(_) => println!("Email sent to {:?}", receiver),
      Err(e) => panic!("Error sending email to {:?}: {:?}", receiver, e),
  }
}
