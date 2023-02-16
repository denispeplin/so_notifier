use notify_rust::{Notification, Urgency};
use std::thread;

pub fn send(summary: &str, link: &str) {
    let (handle_tx, rx) = std::sync::mpsc::channel();

    let handle = Notification::new()
        .summary(summary)
        .body(link)
        .icon("hint")
        .urgency(Urgency::Critical)
        .show()
        .expect("Notification must be sent");

    thread::spawn(move || handle.on_close(|_action| handle_tx.send(()).unwrap_or(())));

    rx.recv().unwrap();
}
