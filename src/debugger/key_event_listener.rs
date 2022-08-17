use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use crossterm::event::{Event, KeyEvent, read};

pub fn create_key_event_receiver() -> Receiver<KeyEvent> {
    let (tx, rx) = mpsc::channel::<KeyEvent>();

    thread::spawn(move || {
        loop {
            let event = read().unwrap();

            if let Event::Key(e) = event {
                tx.send(e).unwrap();
            }
        }
    });

    rx
}
