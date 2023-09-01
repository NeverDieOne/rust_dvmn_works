use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use std::{thread, time::{Duration, SystemTime, UNIX_EPOCH}};
use log::{info, warn, error};
use env_logger;
use dotenv::dotenv;
use std::env;

mod models;

fn main() {
    env_logger::init();

    dotenv().ok();
    let devman_token = env::var("DEVMAN_TOKEN")
        .expect("Не удалось найти Devman токен в переменных");
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("Не удалось найти TG токен в переменных");
    let chat_id = env::var("TG_CHAT_ID")
        .expect("Не найден chat_id в переменных");

    let mut timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap().as_secs_f32();

    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage", telegram_bot_token);
    let url = "https://dvmn.org/api/long_polling/";
    let client = Client::new();

    loop {
        let devman_response = match client
            .get(url)
            .header(AUTHORIZATION, format!("Token {}", devman_token))
            .query(&[("timestamp", timestamp)])
            .timeout(Duration::from_secs(60))
            .send()
            .expect("Не удалось отправить запрос")
            .error_for_status() {
                Ok(res) => res,
                Err(err) => {
                    if err.is_timeout() {
                        info!("Произошёл таймаут");
                        continue
                    } else if err.is_connect() {
                        warn!("Ошибка подключения, ожидание 1 мин");
                        thread::sleep(Duration::from_secs(60));
                        continue
                    } else {
                        error!("{err}");
                        break
                    }
                }
            }
            .text()
            .expect("Не удалось получить ответ");
    
        let review: models::Review = serde_json::from_str(&devman_response)
            .expect("Ошибка парсинга json");
        
        match review.status.as_str() {
            "found" => {
                timestamp = review.last_attempt_timestamp
                    .expect("Не удалось найти timestamp");
                for attempt in review.new_attempts {
                    match client
                        .get(&telegram_url)
                        .query(&[("chat_id", &chat_id), ("text", &attempt.get_message())])
                        .send()
                        .expect("Не удалось сделать запрос")
                        .error_for_status() {
                            Ok(_) => info!("Сообщение отправлено"),
                            Err(err) => warn!(
                                "Не удалось отправить сообщение: {}",
                                err.status().expect("Не удалось получить статус-код")
                            )
                        }
                }
            },
            "timeout" => {
                timestamp = review.timestamp_to_request
                    .expect("Не удалось найти timestamp")
            },
            &_ => continue
        };

        thread::sleep(Duration::from_secs(60));
    };
    
}
