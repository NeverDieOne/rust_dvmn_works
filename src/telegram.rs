use reqwest::blocking::Client;
use log::warn;


pub trait Messanger {
    fn send_message(&self, chat_id: &str, text: &str);
}

pub struct Telegram {
    client: Client,
    url: String
}


impl Telegram {
    pub fn new(token: &str) -> Self {
        return Telegram {
            client: Client::new(),
            url: String::from(format!("https://api.telegram.org/bot{}", token))
        }
    }
}


impl Messanger for Telegram {
    fn send_message(&self, chat_id: &str, text: &str) {
        let send_message_url = format!("{}/sendMessage", self.url);
        match self.client
            .get(&send_message_url)
            .query(&[("chat_id", &chat_id), ("text", &text)])
            .send()
            .expect("Не удалось отправить запрос")
            .error_for_status() {
                Ok(_) => (),
                Err(err) => warn!(
                    "Не удалось отправить сообщение: {}",
                    err.status().expect("Не удалось получить статус-код")
                )
            }
    }
}
