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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_attempt() {
        Attempt {
            is_negative: true,
            lesson_title: String::from("Делаю тестовый урок"),
            lesson_url: String::from("Какой-то урл")
        };
    }

    #[test]
    fn cat_create_review() {
        Review {
            status: String::from("timeout"),
            new_attempts: Option::from(Vec::from([])),
            last_attempt_timestamp: Option::from(1.1),
            timestamp_to_request: Option::from(1.1)
        };
    }

    #[test]
    #[should_panic]
    fn one_of_timestamp_shoud_be() {
        let review = Review {
            status: String::from("timeout"),
            new_attempts: Option::from(None),
            last_attempt_timestamp: Option::from(None),
            timestamp_to_request: Option::from(None)
        };
        review.get_timestamp();
    }

    #[test]
    fn can_get_timestamp() {
        let review = Review {
            status: String::from("timeout"),
            new_attempts: Option::from(None),
            last_attempt_timestamp: Option::from(1.1),
            timestamp_to_request: Option::from(None)
        };
        review.get_timestamp();
    }
}