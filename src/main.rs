use serde::Deserialize;

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
    let resp = reqwest::blocking::get(SO_URL.to_owned() + TAG)?.text()?;

    let questions = serde_json::from_str::<Root>(&resp).unwrap();
    for q in questions.items {
        println!("{}\n{}\n{}\n", q.title, q.link, q.question_id);
    }

    Ok(())
}
