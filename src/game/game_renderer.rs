
use super::paperclips::Game;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};

#[derive(Debug, Default)]
pub struct GameRenderer{
}

impl GameRenderer {
    pub fn render_game(&self, game: &Game, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Local Paperclips ".bold());

        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec![
                "Paper Clips: ".into(),
                game.num_paperclips().to_string().yellow(),
            ]),
            Line::from(vec![
                "Money: ".into(),
                format!("{:.2}", game.money()).to_string().yellow(),
            ]),
            Line::from(vec![
                "Demand: ".into(),
                format!("{:.0} %", game.public_demand() * 100.0).to_string().yellow(),
            ]),
            Line::from(vec![
                "Sell Price: ".into(),
                format!("4 {:.2}", game.sell_price()).to_string().yellow(),
            ]),
            Line::from(vec![
                "Wire: ".into(),
                game.wire_manager().amnt_wire().to_string().yellow(),
                format!(" (Cost: {:.2})", game.wire_manager().buy_cost()).into()
            ]),
            Line::from(vec![
                "Clippers: ".into(),
                game.num_clippers().to_string().yellow(),
                format!(" (Cost: {:.2})", game.clippers().buy_price()).into()
            ]),
        ]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
