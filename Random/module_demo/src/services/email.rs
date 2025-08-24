use crate::services::internal;
use crate::utils::email::is_valid_email;

/// Email message.
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EmailClient {
    from: String,
}

impl EmailClient {
    pub fn new(from: String) -> Self {
        Self { from }
    }

    /// send email message to client.
    pub fn send(&self, msg: &EmailMessage) -> () {
        if !is_valid_email(&msg.to) {
            eprintln!("[email] Invalid recipient: {}", msg.to);
            return;
        }
        internal::log_send("email", &msg.to);
        println!(
            "[Email] from {} | To: {} | Subject: {}\n{}",
            self.from, msg.to, msg.subject, msg.body
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_recipents() {
        let client = EmailClient::new("noreply@example.com".to_string());
        let ok = EmailMessage {
            to: "a@b.com".into(),
            body: "hello".into(),
            subject: "hi".into(),
        };

        client.send(&ok);

        let bad = EmailMessage {
            to: "not-an-email".into(),
            subject: "Oops".into(),
            body: "Nope".into(),
        };
        client.send(&bad); // should warn about invalid
    }
}
