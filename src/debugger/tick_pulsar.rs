use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub fn create_tick_receiver() -> Receiver<()> {
    let (tx, rx) = mpsc::channel::<()>();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(20));
            tx.send(()).unwrap();
        }
    });

    rx
}
