use crate::services::internal::log_send;
use crate::utils::phone::normalise_phone;

#[derive(Debug, Clone)]
pub struct SMSMessage {
    pub to: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct SmsClient {
    sender_id: String,
}

impl SmsClient {
    pub fn new(sender_id: &str) -> Self {
        return SmsClient {
            sender_id: sender_id.into(),
        };
    }

    pub fn send(&self, msg: &SMSMessage) -> () {
        let to = normalise_phone(&msg.to);
        log_send("SMS", &to);
        println!(
            "[SMS] From: {} | To: {} | Text: {}",
            self.sender_id, to, msg.text
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn normalizes_phone() {
        let c = SmsClient::new("MyApp".into());
        let m = SMSMessage {
            to: "+44 7700 900123".into(),
            text: "Ping".into(),
        };
        c.send(&m);
    }
}
