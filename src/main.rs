use core::time;
use notify_rust::{Notification, Urgency};
use serde::Deserialize;

use questions::Question;

#[derive(Deserialize, Debug)]
struct Root {
    items: Vec<Question>,
    quota_max: u32,
    quota_remaining: u32,
}

fn decode_questions(text_resp: String) -> Root {
    serde_json::from_str::<Root>(&text_resp).expect(&text_resp)
}

pub mod api_client;
pub mod questions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // it's a trick so on the first run none of
    // the questions would be considered new
    let mut latest_question_id = u32::MAX;

    loop {
        let text_resp = api_client::get_text_response()?;
        let root = decode_questions(text_resp);
        let questions = root.items;

        println!(
            "Quota max: {}, quota remaining: {}",
            root.quota_max, root.quota_remaining
        );

        let new_questions = questions::list_new(&questions, latest_question_id);

        latest_question_id = questions::latest_id(&questions);

        for q in new_questions {
            println!("{}\n{}\n{}\n", q.title, q.link, q.id);
            desktop_notification(&q.title, &q.link);
        }

        std::thread::sleep(time::Duration::from_millis(60_000));
    }
}

fn desktop_notification(summary: &str, link: &str) {
    let handle = Notification::new()
        .summary(summary)
        .body(link)
        .icon("hint")
        .urgency(Urgency::Critical)
        .show()
        .expect("Notification must be sent");

    // won't show the next notification until the previous one is clicked on
    handle.wait_for_action(|_action| ());
}
