use app::{App, StatefulList, AppState};
use components::tab::Tab;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Show, Hide},
};
use std::{error::Error, io, process::Command, sync::mpsc, thread, time::{Instant, Duration}};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

mod components;
mod app;

enum UEvent<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_terminal()?;
    
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(AppState {
        context: get_current_context(),
        pods: StatefulList::with_items(get_current_pods()),
        nodes: StatefulList::with_items(get_current_nodes()),
    });

    let (sender, receiver) = mpsc::channel();
    let tick_rate = std::time::Duration::from_millis(100);

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    sender.send(UEvent::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                sender.send(UEvent::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    loop {
        terminal.draw(|f| {
            app.draw(f).expect("Failed to draw app");
        })?;

        match receiver.recv()? {
            UEvent::Input(key) => {
                match key.code {
                    KeyCode::Char('q') => {
                        shutdown_terminal()?;
                        break;
                    }
                    KeyCode::Char('p') => {
                        app.components.tab.selected_tab = Tab::Pods;
                    }
                    KeyCode::Char('n') => {
                        app.components.tab.selected_tab = Tab::Nodes;
                    }
                    KeyCode::Char('c') => {
                        app.state.context = get_current_context();
                    }
                    _ => {}
                }
            }
            UEvent::Tick => {}
        }
    }

    Ok(())
}

fn get_current_context() -> String {
    let output = Command::new("kubectl")
        .args(&["config", "current-context"])
        .output()
        .expect("Failed to get current K8 context");
    let context = String::from_utf8_lossy(&output.stdout);
    context.trim().to_string()
}

fn get_current_pods() -> Vec<String> {
    let output = Command::new("kubectl")
        .args(&["get", "pods", "-A"])
        .output()
        .expect("Failed to get current K8 pods");
    let pods = String::from_utf8_lossy(&output.stdout);
    pods.lines().map(|s| s.to_string()).collect()
}

fn get_current_nodes() -> Vec<String> {
    let output = Command::new("kubectl")
        .args(&["get", "nodes", "-o", "name"])
        .output()
        .expect("Failed to get current K8 nodes");
    let nodes = String::from_utf8_lossy(&output.stdout);
    nodes.lines().map(|s| s.to_string()).collect()
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Hide
    )?;
    Ok(())
}

fn shutdown_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;
    Ok(())
}
