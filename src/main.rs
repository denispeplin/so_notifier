use core::time;

pub mod api_client;
pub mod decoder;
pub mod notifications;
pub mod questions;

// the loop timeout must be no shorter than 1 minute
// shorter timeouts would be considered an abuse by API
const LOOP_TIMEOUT_MINS: u64 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // it's a trick so on the first run none of
    // the questions would be considered new
    let mut latest_question_id = u32::MAX;

    loop {
        let text_resp = api_client::get_text_response()?;
        let root = decoder::decode_questions(text_resp);
        let questions = root.items;

        let new_questions = questions::list_new(&questions, latest_question_id);

        latest_question_id = questions::latest_id(&questions);

        for q in new_questions {
            println!("{}\n{}\n{}\n", q.title, q.link, q.id);
            notifications::desktop::send(&q.title, &q.link);
        }

        std::thread::sleep(time::Duration::from_secs(LOOP_TIMEOUT_MINS * 60));
    }
}
