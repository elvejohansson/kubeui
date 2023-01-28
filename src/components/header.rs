use tui::{backend::Backend, Frame, layout::{Rect, Alignment}, text::{Span, Spans}, widgets::{Paragraph, Borders, Block}, style::{Style, Modifier}};

pub struct HeaderComponent {
    pub context: String,
}

impl HeaderComponent {
    pub fn new(ctx: String) -> Self {
        Self {
            context: ctx,
        }
    }

    pub fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect
    ) {
        let header_block = Block::default()
            .borders(Borders::ALL)
            .title(format!("kubetui - {}", env!("CARGO_PKG_VERSION")))
            .style(Style::default().add_modifier(Modifier::BOLD))
            .title_alignment(Alignment::Center);
        let header_text = vec![
            Spans::from(vec![
                Span::styled("Context: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&self.context, Style::default().remove_modifier(Modifier::BOLD)),
            ]),
        ];

        let header = Paragraph::new(header_text)
            .block(header_block)
            .alignment(Alignment::Left);
        
        f.render_widget(header, area)
    }
}
