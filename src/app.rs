use std::error::Error;

use tui::{widgets::ListState, backend::Backend, Frame, layout::{Direction, Constraint, Layout}};

use crate::{
    components::{
        tab::Tab,
        HeaderComponent,
        NodesComponent,
        PodsComponent,
        TabComponent,
    }
};

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }
}

pub struct AppComponents {
    pub header: HeaderComponent,
    pub nodes: NodesComponent,
    pub pods: PodsComponent,
    pub tab: TabComponent,
}

pub struct AppState {
    pub context: String,
    pub pods: StatefulList<String>,
    pub nodes: StatefulList<String>,
}


pub struct App {
    pub components: AppComponents,
    pub state: AppState
}

impl App {
    pub fn new(state: AppState) -> Self {
        Self {
            components: AppComponents {
                header: HeaderComponent::new(state.context.clone()),
                nodes: NodesComponent::new(state.nodes),
                pods: PodsComponent::new(state.pods),
                tab: TabComponent::new(),
            },
            state: AppState {
                context: state.context,
                pods: StatefulList {
                    state: ListState::default(),
                    items: vec![]
                },
                nodes: StatefulList {
                    state: ListState::default(),
                    items: vec![]
                }
            }
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
    
        self.components.header.draw(f, wrapper[0]);

        self.components.tab.draw(f, main_chunks[0]);

        match self.components.tab.selected_tab {
            Tab::Pods => {
                self.components.pods.draw(f, main_chunks[1])
            }
            Tab::Nodes => {
                self.components.nodes.draw(f, main_chunks[1])
            }
        }

        Ok(())
    }
    
}
