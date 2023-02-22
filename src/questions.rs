use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Question {
    pub title: String,
    pub link: String,
    #[serde(rename = "question_id")]
    pub id: u32,
}

pub fn list_new(questions: &[Question], latest_question_id: u32) -> Vec<&Question> {
    questions
        .iter()
        .filter(|q| q.id > latest_question_id)
        .collect::<Vec<&Question>>()
}

pub fn latest_id(questions: &[Question]) -> u32 {
    // questions are coming from API reverse sorted by ID
    questions[0].id
}

pub fn initial_id() -> u32 {
    u32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_new() {
        let questions = build_questions();
        let new_questions = list_new(&questions, 2);

        assert_eq!(
            new_questions,
            vec![&Question {
                id: 3,
                ..Default::default()
            }]
        );
    }

    #[test]
    fn test_with_initial_id() {
        let questions = build_questions();
        let new_questions = list_new(&questions, initial_id());

        assert!(new_questions.is_empty());
    }

    #[test]
    fn test_latest_id() {
        let questions = build_questions();

        let latest_id = latest_id(&questions);

        assert_eq!(latest_id, 3);
    }

    impl Default for Question {
        fn default() -> Self {
            Self {
                link: String::default(),
                title: String::default(),
                id: 0,
            }
        }
    }

    fn build_questions() -> Vec<Question> {
        vec![
            Question {
                id: 3,
                ..Default::default()
            },
            Question {
                id: 2,
                ..Default::default()
            },
        ]
    }
}
