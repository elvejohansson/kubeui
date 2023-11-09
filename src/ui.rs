use std::rc::Rc;

use itertools::Itertools;
use ratatui::{
    prelude::{Alignment, Buffer, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Widget},
};

use crate::{AppState, TABS};

pub struct Root {
    context: AppState,
}

impl Widget for Root {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().render(area, buf);
        let area = layout(area, Direction::Vertical, vec![3, 0, 1]);
        self.render_tabs_bar(area[0], buf);
        self.render_selected_tab(area[1], buf);
        self.render_bottom_bar(area[2], buf);
    }
}

impl Root {
    pub fn new(context: AppState) -> Self {
        return Root { context };
    }

    fn render_tabs_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Vertical, vec![1]);

        let titles = TABS
            .iter()
            .map(|(idx, label)| {
                let selected_index = self.context.tab_index == idx.clone();

                let label_style = if selected_index {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                return text::Line::from(Span::styled(format!(" {}: {}", idx, label), label_style));
            })
            .collect();

        Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Kubeui"))
            .render(area[0], buf);
    }

    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        let index = self.context.tab_index;
        match index {
            1 => Paragraph::new("nodes")
                .block(Block::default().borders(Borders::ALL))
                .render(area, buf),
            2 => Paragraph::new("pods")
                .block(Block::default().borders(Borders::ALL))
                .render(area, buf),
            _ => unreachable!(),
        };
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0]);

        let block = Block::new().style(Style::default().bg(Color::DarkGray));

        let keys = [("?/h", "Help"), ("q/Esc", "Quit")];

        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(
                    format!(" {} ", key),
                    Style::default().bg(Color::Yellow).fg(Color::Black),
                );
                let desc = Span::styled(format!(" {} ", desc), Style::default());
                return [key, desc];
            })
            .collect_vec();

        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .block(block)
            .render(area[0], buf);
    }
}

/// Helper method for splitting an area into mutliple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect_vec();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}
