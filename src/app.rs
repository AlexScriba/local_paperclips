use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::
        Widget
    ,
    Frame,
};

use crate::game::{game_renderer::GameRenderer, paperclips::Game};
use crate::{interval::IntervalTimer, tui::Tui};

#[derive(Debug, Default)]
pub struct App {
    game: Game,
    renderer: GameRenderer,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        let mut interval = IntervalTimer::new(100);

        while !self.exit {
            if interval.elapsed_and_clear() {
                self.game.tick();
            }

            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('m') => self.game.make_paperclip(),
            KeyCode::Char('b') => self.game.buy_clipper(),
            KeyCode::Char('w') => self.game.buy_wire(),
            KeyCode::Up => self.game.change_sell_price(0.01),
            KeyCode::Down => self.game.change_sell_price(-0.01),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.renderer.render_game(&self.game, area, buf);
    }
}
