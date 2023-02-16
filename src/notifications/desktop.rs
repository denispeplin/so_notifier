use notify_rust::{Notification, Urgency};

pub fn send(summary: &str, link: &str) {
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
