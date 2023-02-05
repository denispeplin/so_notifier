use core::time;
use notify_rust::Notification;
use serde::Deserialize;
use std::collections::HashSet;

const SO_URL: &str = "https://api.stackexchange.com/2.3/questions?page=1&pagesize=10&order=desc&sort=creation&site=stackoverflow&tagged=";
// the tag is hardcoded for now to `rust`
const TAG: &str = "rust";

#[derive(Deserialize, Debug)]
struct Root {
    items: Vec<Question>,
}

#[derive(Deserialize, Debug)]
struct Question {
    title: String,
    link: String,
    question_id: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut question_ids = HashSet::new();

    loop {
        let resp = reqwest::blocking::get(SO_URL.to_owned() + TAG)?.text()?;

        let questions = serde_json::from_str::<Root>(&resp).unwrap();

        if !question_ids.is_empty() {
            for q in &questions.items {
                if !question_ids.contains(&q.question_id) {
                    println!("{}\n{}\n{}\n", q.title, q.link, q.question_id);
                    desktop_notification(&q.title, &q.link);
                }
            }
        }

        question_ids.clear();
        for q in questions.items {
            question_ids.insert(q.question_id);
        }

        std::thread::sleep(time::Duration::from_millis(60_000));
    }
}

fn desktop_notification(summary: &str, link: &str) {
    Notification::new()
        .summary(summary)
        .body(link)
        .icon("hint")
        .show()
        .expect("Notification must be sent");
}
