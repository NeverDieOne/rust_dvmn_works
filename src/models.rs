use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attempt {
    pub is_negative: bool,
    pub lesson_title: String,
    pub lesson_url: String
}

impl Attempt {
    pub fn get_message(&self) -> String {
        let result_text = if self.is_negative {"Не принята"} else {"Принята"};
        return String::from(format!(
            "Работа {} \nСсылка на урок: {}\nУспешность: {}",
            self.lesson_title, self.lesson_url, result_text
        ))
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub status: String,
    pub new_attempts: Option<Vec<Attempt>>,
    pub last_attempt_timestamp: Option<f32>,
    pub timestamp_to_request: Option<f32>,
}


impl Review {
    pub fn get_timestamp(&self) -> f32 {
        self.last_attempt_timestamp.unwrap_or_else(
            ||self.timestamp_to_request.unwrap()
        )
    }
}
