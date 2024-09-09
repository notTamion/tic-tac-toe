use std::time::Duration;
use color_eyre::Result;
use async_trait::async_trait;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::action::Action;

pub mod game_selection;
mod local_game;

#[async_trait]
pub trait Component {

    async fn handle_event(&mut self) -> Result<Action> {
        if event::poll(Duration::from_millis(16))? == false {
            return Ok(Action::None)
        }
        match event::read()? {
            Event::Key(event) => self.handle_key_event(event).await,
            Event::Paste(pasted_string)  => self.handle_paste(pasted_string),
            _ => Ok(Action::None)
        }
    }

    async fn handle_key_event(&mut self, _key_event: KeyEvent) -> Result<Action> {
        Ok(Action::None)
    }

    fn handle_paste(&mut self, _pasted_string: String) -> Result<Action> {
        Ok(Action::None)
    }

    async fn update(&mut self) -> Result<Action> {
        Ok(Action::None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect);
}