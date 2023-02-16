use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Question {
    pub title: String,
    pub link: String,
    #[serde(rename = "question_id")]
    pub id: u32,
}

pub fn list_new(questions: &Vec<Question>, latest_question_id: u32) -> Vec<&Question> {
    questions
        .iter()
        .filter(|q| q.id > latest_question_id)
        .map(|q| q.clone())
        .collect::<Vec<&Question>>()
}

pub fn latest_id(questions: &Vec<Question>) -> u32 {
    // questions are coming from API reverse sorted by ID
    questions[0].id
}
