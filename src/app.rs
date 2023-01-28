use std::error::Error;

use tui::{widgets::ListState, backend::Backend, Frame, layout::{Direction, Constraint, Layout}};

use crate::{
    components::{
        HeaderComponent,
        TabComponent, tab::Tab
    }
};

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

pub struct App {
    pub header: HeaderComponent,
    pub tab: TabComponent,
}

impl App {
    pub fn new(ctx: String) -> Self {
        Self {
            header: HeaderComponent::new(ctx),
            tab: TabComponent::new(),
        }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
    ) -> Result<(), Box<dyn Error>> {
        let wrapper = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Percentage(100)
            ].as_ref())
            .split(f.size());

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0)
            ].as_ref())
            .split(wrapper[1]);
    
        self.header.draw(f, wrapper[0]);

        self.tab.draw(f, main_chunks[0]);

        match self.tab.selected_tab {
            Tab::Pods => {
                self.header.draw(f, main_chunks[1])
            }
            Tab::Nodes => {
                self.header.draw(f, main_chunks[1])
            }
        }

        Ok(())
    }
    
}
