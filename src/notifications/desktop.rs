use notify_rust::{Notification, Urgency};
use std::{thread, time};

const TIMEOUT: time::Duration = time::Duration::from_secs(5 * 60);

pub fn send(summary: &str, link: &str) {
    let (handle_tx, rx) = std::sync::mpsc::channel();
    let timeout_tx = handle_tx.clone();

    let handle = Notification::new()
        .summary(summary)
        .body(link)
        .icon("hint")
        .urgency(Urgency::Critical)
        .show()
        .expect("Notification must be sent");

    thread::spawn(move || handle.on_close(|_action| handle_tx.send(()).unwrap_or(())));

    thread::spawn(move || {
        thread::sleep(TIMEOUT);
        timeout_tx.send(()).ok();
    });

    rx.recv().unwrap();
}
