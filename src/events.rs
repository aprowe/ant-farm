use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use termion::event::Key;
use termion::input::TermRead;

pub enum TermEvent<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct TermEventHandler {
    pub tick_rate: Duration,
    rx: mpsc::Receiver<TermEvent<Key>>,
    input_handle: thread::JoinHandle<()>,
    last_update: SystemTime,
}

impl TermEventHandler {
    pub fn new(tick_rate: Duration) -> TermEventHandler {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(TermEvent::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };
        TermEventHandler {
            last_update: SystemTime::now(),
            rx,
            input_handle,
            tick_rate,
        }
    }

    // Get the next input event, or a tick event if enough time has passed
    pub fn next(&mut self) -> TermEvent<Key> {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.last_update).unwrap();

        // Wait up from 0 to tick rate
        let duration = self.tick_rate.saturating_sub(elapsed);

        // Wait up to duration
        if let Ok(evt) = self.rx.recv_timeout(duration) {
            return evt;
        } else {
            // Update last tick time
            self.last_update = now;

            return TermEvent::Tick;
        }
    }
}
