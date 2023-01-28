use app::{App, StatefulList, AppState};
use components::tab::Tab;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Show, Hide},
};
use std::{error::Error, io, process::Command};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

mod components;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    setup_terminal()?;
    
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(AppState {
        context: get_current_context(),
        pods: StatefulList::with_items(get_current_pods()),
        nodes: StatefulList::with_items(get_current_nodes()),
    });

    loop {
        terminal.draw(|f| {
            app.draw(f).expect("Failed to draw app");
        })?;

        if let Event::Key(key) = event::read()? {
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
                _ => {}
            }
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
