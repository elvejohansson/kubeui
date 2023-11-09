use std::{
    io::{stdout, Result, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::ui::Root;

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    should_quit: bool,
    context: AppState,
}

// This is super messy, but super useful
pub static TABS: &'static [(i32, &str)] = &[(1, "Nodes"), (2, "Pods")];

#[derive(Clone, Copy)]
pub struct AppState {
    pub tab_index: i32,
}

impl App {
    pub fn run() -> Result<()> {
        let mut app = Self::new()?;
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        while !app.should_quit {
            app.draw_ui()?;
            app.handle_events()?;
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;

        return Ok(());
    }

    fn new() -> Result<Self> {
        let term = Terminal::new(CrosstermBackend::new(stdout()))?;

        return Ok(Self {
            terminal: term,
            should_quit: false,
            context: AppState {
                tab_index: 1, // TODO: Load this from save file
            },
        });
    }

    fn draw_ui(&mut self) -> Result<()> {
        self.terminal
            .draw(|frame| return frame.render_widget(Root::new(self.context), frame.size()))?;

        return Ok(());
    }

    fn handle_events(&mut self) -> Result<()> {
        return match self.next_event(Duration::from_millis(16))? {
            Some(Event::Key(key)) => self.handle_key_event(key),
            _ => Ok(()),
        };
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // windows send "release" event as well, lets ignore that
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        // TODO: Clean up tab matching, use static TABS instead

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('1') => {
                self.context.tab_index = 1;
            }
            KeyCode::Char('2') => {
                self.context.tab_index = 2;
            }
            _ => {}
        };

        return Ok(());
    }

    fn next_event(&mut self, timeout: Duration) -> Result<Option<Event>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }
        let event = event::read()?;
        Ok(Some(event))
    }
}
