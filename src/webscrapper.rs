use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport}; 

fn main() {
    send_mail();
}

fn send_mail() {
    let smtp_key: &str = "xsmtpsib-57c2e14769476eb421eaebaa2ecbd50bb06a7846963cc485206dd3e2dd4243a0-E8fvIYU26WPAH1k4";
    let from_email: &str = "Nicolas_1224@hotmail.com";
    let host: &str = "smtp-relay.brevo.com";
    let to_email: &str = "nicolas_1224@hotmail.com";

    let email = Message::builder() 
    .from(from_email.parse().unwrap())
    .to(to_email.parse().unwrap())
    .subject("test email")
    .body("Hello World".to_string())
    .unwrap();

    let mailer: SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(Credentials::new(
            from_email.to_string(),
            smtp_key.to_string(),
        ))
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}