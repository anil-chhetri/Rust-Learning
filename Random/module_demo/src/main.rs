mod services;
mod utils;


use crate::services::{EmailClient, EmailMessage, SmsClient, SMSMessage};

fn main() {
   let email_client = EmailClient::new("noreply@example.com".to_string());
    let sms_client = SmsClient::new("MyApp");

    let welcome = EmailMessage {
        to: "alice@example.com".into(),
        subject: "Welcome!".into(),
        body: "Hi Alice, glad you're here.".into(),
    };
    email_client.send(&welcome);

    let otp = SMSMessage {
        to: "+44 7700 900123".into(),
        text: "Your code is 123456".into(),
    };
    sms_client.send(&otp);

    let recipients = vec![
        "bob@example.com".to_string(),
        "carol@foo.co".to_string(),
        "not-an-email".to_string(),
    ];
    services::broadcast_email(&email_client, &recipients, "News", "Hello from Module Demo!");
}
