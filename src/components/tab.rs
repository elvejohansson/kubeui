use tui::{backend::Backend, Frame, layout::{Rect}, text::{Spans}, widgets::{Borders, Block, Tabs}, style::{Style, Modifier, Color}};

#[derive(Copy, Clone)]
pub enum Tab {
    Pods,
    Nodes,
}

pub struct TabComponent {
    pub selected_tab: Tab
}

impl TabComponent {
    pub fn new() -> Self {
        Self {
            selected_tab: Tab::Pods,
        }
    }

    fn names(&self) -> Vec<String> {
        vec![
            String::from(format!("[{}] Pods", "p")),
            String::from(format!("[{}] Nodes", "n"))
        ]
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect
    ) {
        let titles = self.names().iter().cloned().map(Spans::from).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.selected_tab as usize)
            .style(Style::default())
            .highlight_style(
                Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD)
            )
            .divider("");

        f.render_widget(tabs, area)        
    }
}
