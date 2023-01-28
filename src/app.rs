use tui::{widgets::ListState, backend::Backend, Frame, layout::{Direction, Constraint, Layout}};

use crate::{
    components::{
        HeaderComponent,
    }
};

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

pub struct App {
    pub header: HeaderComponent,
}

impl App {
    pub fn new(ctx: String) -> Self {
        Self {
            header: HeaderComponent::new(ctx),
        }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
    ) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Percentage(100)
            ].as_ref())
            .split(f.size());
    
        self.header.draw(f, main_chunks[0])
    }
    
}
