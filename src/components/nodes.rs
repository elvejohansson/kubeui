use tui::{backend::Backend, Frame, layout::{Rect}, text::{Span}, widgets::{Borders, Block, List, ListItem}};

use crate::app::StatefulList;

pub struct NodesComponent {
    pub nodes: StatefulList<String>,
}

impl NodesComponent {
    pub fn new(nodes: StatefulList<String>) -> Self {
        Self {
            nodes,
        }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect
    ) {
        let items: Vec<ListItem> = self
            .nodes
            .items
            .iter().map(|i| ListItem::new(Span::raw(i))).collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL));

        f.render_stateful_widget(items, area, &mut self.nodes.state);


    }
}
