use async_trait::async_trait;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::crossterm::event::KeyCode::Char;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Clear, List, ListState, Paragraph};
use crate::action::Action;
use crate::components::Component;
use crate::components::game::Square;
use crate::components::game::Square::Draw;
use crate::components::main_menu::MainMenu;
use crate::components::super_game::SuperGame;

pub struct SuperLocalGame {
    game: SuperGame,
    menu_state: ListState,
    has_menu_open: bool,
    in_setup: bool,
}

#[async_trait]
impl Component for SuperLocalGame {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<Action> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(Action::None);
        }

        if self.in_setup {
            match key_event.code {
                KeyCode::Enter => {
                    self.in_setup = false;
                    self.game.managing_game.show_selector = true;
                }
                Char('j') | KeyCode::Down => self.game.set_size(self.game.managing_game.board.len() - 1),
                Char('k') | KeyCode::Up => self.game.set_size(self.game.managing_game.board.len() + 1),
                _ => {}
            }
        } else if self.has_menu_open {
            match key_event.code {
                KeyCode::Esc => {
                    self.has_menu_open = false;
                    self.menu_state = ListState::default().with_selected(Some(0));
                }
                Char('j') | KeyCode::Down => self.menu_state.select_next(),
                Char('k') | KeyCode::Up => self.menu_state.select_previous(),
                KeyCode::Enter => {
                    match self.menu_state.selected().unwrap() {
                        0 => {
                            self.has_menu_open = false;
                            self.menu_state = ListState::default().with_selected(Some(0));
                        },
                        1 => {
                            self.game.rematch();
                            self.menu_state = ListState::default().with_selected(Some(0));
                            self.has_menu_open = false;
                        }
                        2 => {
                            self.game.restart();
                            self.menu_state = ListState::default().with_selected(Some(0));
                            self.has_menu_open = false;
                        }
                        3 => return Ok(Action::ChangeComponent(Box::new(MainMenu::new()))),
                        4 => return Ok(Action::Quit),

                        _ => {}
                    }
                }
                _ => {}
            }

            return Ok(Action::None);
        } else if self.game.managing_game.winner.0 == Square::None {
            if key_event.code == KeyCode::Esc {
                self.has_menu_open = true;
            } else {
                self.game.handle_key_event(key_event).await?;
                if self.game.managing_game.winner.0 != Square::None {
                    self.has_menu_open = true;
                }
            }
        } else {
            self.has_menu_open = true;
        }

        Ok(Action::None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Percentage(75),
                Constraint::Fill(1)]
            )
            .split(Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1)]
                ).split(area)[1]
            );
        let text;
        if self.game.managing_game.winner.0 == Square::None {
            let mut player1 = Span::from(format!("{} Player1", self.game.managing_game.scores.0));
            let mut player2 = Span::from(format!("Player2 {}", self.game.managing_game.scores.1));
            if self.game.managing_game.turn == Square::X {
                player1 = player1.style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                player2 = player2.style(Style::new().add_modifier(Modifier::REVERSED));
            }
            text = Text::from(ratatui::prelude::Line::from(vec![player1, Span::from(" | "), player2]));
        } else {
            if self.game.managing_game.winner.0 == Draw {
                text = Text::from("Draw!").style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                text = Text::from(if self.game.managing_game.winner.0 == Square::X { "Player1 wins!" } else { "Player2 wins!" }).style(Style::new().add_modifier(Modifier::REVERSED));
            }
        }
        frame.render_widget(Paragraph::new(text).centered(), layout[1]);
        self.game.render(frame, layout[3]);

        if self.has_menu_open {
            let menu_layout = Layout::default().direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(7),
                    Constraint::Fill(1)]
                ).split(Layout::default().direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(5),
                    Constraint::Fill(1)]).split(layout[3])[1]);
            frame.render_widget(Clear::default(), menu_layout[1]);
            frame.render_stateful_widget(List::new(["Resume", "Rematch", "Restart", "Menu", "Quit"]).highlight_style(Style::new().add_modifier(Modifier::REVERSED)), menu_layout[1], &mut self.menu_state);
        } else if self.in_setup {
            let layout = Layout::default().direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(16),
                    Constraint::Fill(1)]
                ).split(Layout::default().direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(3),
                    Constraint::Fill(1)]).split(layout[3])[1]);
            frame.render_widget(Clear::default(), layout[1]);
            frame.render_widget(Paragraph::new("Select Game Size\nusing arrow keys\nand hit Enter").centered(), layout[1]);
        }
    }
}

impl SuperLocalGame {
    pub fn new() -> Self {
        let mut game = SuperGame::new();
        game.managing_game.show_selector = false;
        SuperLocalGame { game, has_menu_open: false, menu_state: ListState::default().with_selected(Some(0)), in_setup: true }
    }
}
