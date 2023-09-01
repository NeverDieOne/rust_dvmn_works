use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attempt {
    pub is_negative: bool,
    pub lesson_title: String,
    pub lesson_url: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub status: String,
    pub new_attempts: Vec<Attempt>,
    pub last_attempt_timestamp: Option<f32>,
    pub timestamp_to_request: Option<f32>,
}

