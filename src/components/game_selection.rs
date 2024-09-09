use async_trait::async_trait;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use crate::action::Action;
use crate::components::Component;
use color_eyre::Result;
use ratatui::crossterm::event::KeyCode::Char;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{List, ListState, Paragraph};
use crate::components::local_game::LocalGame;

pub struct GameSelection {
    list_state: ListState,
}

#[async_trait]
impl Component for GameSelection {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<Action> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(Action::None)
        }
        match key_event.code {
            Char('q') => return Ok(Action::Quit),
            Char('j') | KeyCode::Down => self.list_state.select_next(),
            Char('k') | KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Enter => {
                    match self.list_state.selected().unwrap() {
                        0 => return Ok(Action::ChangeComponent(Box::new(LocalGame::new()))),
                        1 => return Ok(Action::Quit),
                        _ => ()
                    }
            }
            _ => ()
        }
        Ok(Action::None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let game_modes = ["Local", "Quit"];
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(2),
                Constraint::Length(game_modes.len() as u16),
                Constraint::Fill(1)]
            )
            .split(Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(11),
                Constraint::Fill(1)]
            ).split(area)[1]
        );
        frame.render_widget(Paragraph::new("Tic Tac Toe"), layout[1]);
        frame.render_stateful_widget(List::new(game_modes).highlight_style(Style::new().add_modifier(Modifier::REVERSED)), layout[2], &mut self.list_state);
    }
}

impl GameSelection {
    pub fn new() -> Self {
        let list_state = ListState::default().with_selected(Some(0));
        GameSelection { list_state }
    }
}