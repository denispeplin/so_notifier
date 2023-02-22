use serde::Deserialize;

use super::logger::log;
use super::questions::Question;

#[derive(Deserialize, Debug)]
pub struct Root {
    pub items: Vec<Question>,
    quota_max: u32,
    quota_remaining: u32,
}

pub fn decode_questions(text_resp: String) -> Root {
    let root = serde_json::from_str::<Root>(&text_resp).expect(&text_resp);
    print_quota(&root);
    root
}

fn print_quota(root: &Root) {
    log(format!(
        "Quota max: {}, quota remaining: {}",
        root.quota_max, root.quota_remaining
    ));
}
