use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};
// use termion::event::Key;
// use termion::input::TermRead;

/// A config used to keep track of the tick rate and the exit key
#[derive(Clone, Copy)]
pub struct Config {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: KeyCode::Char('q'),
            tick_rate: Duration::from_millis(1000),
        }
    }
}

/// Event for termion input and tick events
pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    #[allow(dead_code)]
    input_handle: thread::JoinHandle<()>,
    #[allow(dead_code)]
    ignore_exit_key: Arc<AtomicBool>,
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    if event::poll(config.tick_rate - last_tick.elapsed()).unwrap() {
                        if let CEvent::Key(key) = event::read().unwrap() {
                            tx.send(Event::Input(key)).unwrap();
                            if !ignore_exit_key.load(Ordering::Relaxed)
                                && key.code == config.exit_key
                            {
                                return;
                            }
                        }
                    }
                    if last_tick.elapsed() >= config.tick_rate {
                        tx.send(Event::Tick).unwrap();
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Events {
            rx,
            ignore_exit_key,
            input_handle,
        }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }

    // pub fn disable_exit_key(&mut self) {
    //     self.ignore_exit_key.store(true, Ordering::Relaxed);
    // }

    // pub fn enable_exit_key(&mut self) {
    //     self.ignore_exit_key.store(false, Ordering::Relaxed);
    // }
}
