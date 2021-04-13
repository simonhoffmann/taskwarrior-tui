use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use async_std::channel::unbounded;
use async_std::sync::Arc;
use async_std::task;
use futures::prelude::*;
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct EventConfig {
    pub tick_rate: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    pub rx: async_std::channel::Receiver<Event<event::KeyEvent>>,
}

impl Events {
    pub fn with_config(config: EventConfig) -> Events {
        use crossterm::event::{KeyCode::*, KeyModifiers};
        let tick_rate = config.tick_rate;
        let (tx, rx) = unbounded::<Event<event::KeyEvent>>();
        task::spawn_local(async move {
            let mut reader = EventStream::new();

            loop {
                let mut delay = Delay::new(tick_rate).fuse();
                let mut event = reader.next().fuse();

                select! {
                    _ = delay => {
                        tx.send(Event::Tick).await.ok();
                    },
                    maybe_event = event => {
                        if let Some(Ok(event::Event::Key(e))) = maybe_event {
                            tx.send(Event::Input(e)).await.unwrap();
                            task::sleep(Duration::from_millis(1)).await;
                            task::yield_now().await;
                        };
                    }
                }
            }
        });
        Events { rx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub async fn next(&self) -> Result<Event<event::KeyEvent>, async_std::channel::RecvError> {
        self.rx.recv().await
    }

    pub fn leave_tui_mode(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
        terminal.show_cursor().unwrap();
    }

    pub fn enter_tui_mode(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
        enable_raw_mode().unwrap();
        terminal.resize(terminal.size().unwrap()).unwrap();
    }
}
