pub mod email;
pub mod sms;

// internal helper shared across service, not part of public API
pub(crate) mod internal;

//re-export
pub use email::{EmailClient, EmailMessage};
pub use sms::{SMSMessage, SmsClient};

pub fn broadcast_email(client: &EmailClient, recipents: &[String], subject: &str, body: &str) {
    for to in recipents {
        let msg = EmailMessage {
            to: to.clone(),
            body: body.to_string(),
            subject: subject.to_string(),
        };
        client.send(&msg);
    }
}
