use async_trait::async_trait;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::crossterm::event::KeyCode::Char;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::canvas::{Canvas, Circle, Line, Rectangle};
use ratatui::widgets::{Clear, List, ListState, Paragraph};
use crate::action::Action;
use crate::components::Component;
use crate::components::local_game::Square::Draw;
use crate::components::game_selection::GameSelection;
use crate::game::{Game, Square};

pub struct LocalGame {
    game: Game,
    selected: (f64, f64),
    menu_state: ListState,
    has_menu_open: bool,
}

#[async_trait]
impl Component for LocalGame {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<Action> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(Action::None);
        }

        if self.has_menu_open {
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
                            self.selected = (1.0, 1.0);
                            self.menu_state = ListState::default().with_selected(Some(0));
                            self.has_menu_open = false;
                        }
                        2 => {
                            self.game.restart();
                            self.selected = (1.0, 1.0);
                            self.menu_state = ListState::default().with_selected(Some(0));
                            self.has_menu_open = false;
                        }
                        3 => return Ok(Action::ChangeComponent(Box::new(GameSelection::new()))),
                        4 => return Ok(Action::Quit),

                        _ => {}
                    }
                }
                _ => {}
            }

            return Ok(Action::None);
        } else if self.game.winner.0 == Square::None {
            match key_event.code {
                Char('k') | KeyCode::Up => {
                    if self.selected.1 > 0.0 {
                        self.selected.1 -= 1.0;
                    }
                }
                Char('j') | KeyCode::Down => {
                    if self.selected.1 < 2.0 {
                        self.selected.1 += 1.0;
                    }
                }
                Char('l') | KeyCode::Right => {
                    if self.selected.0 < 2.0 {
                        self.selected.0 += 1.0;
                    }
                }
                Char('h') | KeyCode::Left => {
                    if self.selected.0 > 0.0 {
                        self.selected.0 -= 1.0;
                    }
                }
                KeyCode::Esc => self.has_menu_open = true,
                KeyCode::Enter => {
                    self.game.hit(self.selected.0, self.selected.1);
                    if self.game.winner.0 != Square::None {
                        self.has_menu_open = true;
                    }
                }
                _ => ()
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
        let canvas = Canvas::default()
            .x_bounds([-99.0, 99.0])
            .y_bounds([-99.0, 99.0])
            .paint(|ctx| {
                ctx.draw(&Line {
                    x1: -33.0,
                    y1: 99.0,
                    x2: -33.0,
                    y2: -99.0,
                    color: Color::White,
                });
                ctx.draw(&Line {
                    x1: 33.0,
                    y1: 99.0,
                    x2: 33.0,
                    y2: -99.0,
                    color: Color::White,
                });
                ctx.draw(&Line {
                    x1: -99.0,
                    y1: 33.0,
                    x2: 99.0,
                    y2: 33.0,
                    color: Color::White,
                });
                ctx.draw(&Line {
                    x1: -99.0,
                    y1: -33.0,
                    x2: 99.0,
                    y2: -33.0,
                    color: Color::White,
                });
                if self.game.winner.0 == Square::None || self.game.winner.0 == Draw {
                    ctx.draw(&Rectangle {
                        x: -95.0 + 66.0 * self.selected.0,
                        y: 37.0 - 66.0 * self.selected.1,
                        color: Color::Green,
                        height: 58.0,
                        width: 58.0,
                    });
                }
                for x in 0..3 {
                    for y in 0..3 {
                        let x = x as f64;
                        let y = y as f64;
                        match self.game.board.get(x as usize).unwrap().get(y as usize).unwrap() {
                            Square::Circle => {
                                ctx.draw(&Circle {
                                    x: -66.0 + 66.0 * x,
                                    y: 66.0 - 66.0 * y,
                                    radius: 25.0,
                                    color: Color::Yellow,
                                })
                            }
                            Square::X => {
                                ctx.draw(&Line {
                                    x1: -91.0 + 66.0 * x,
                                    y1: 91.0 - 66.0 * y,
                                    x2: -41.0 + 66.0 * x,
                                    y2: 41.0 - 66.0 * y,
                                    color: Color::Cyan,
                                });
                                ctx.draw(&Line {
                                    x1: -41.0 + 66.0 * x,
                                    y1: 91.0 - 66.0 * y,
                                    x2: -91.0 + 66.0 * x,
                                    y2: 41.0 - 66.0 * y,
                                    color: Color::Cyan,
                                })
                            }
                            _ => {}
                        }
                    }
                }
                if self.game.winner.0 != Square::None && self.game.winner.0 != Draw {
                    ctx.draw(&Line {
                        x1: self.game.winner.1,
                        y1: self.game.winner.2,
                        x2: self.game.winner.3,
                        y2: self.game.winner.4,
                        color: Color::Red,
                    })
                }
            });
        let text;
        if self.game.winner.0 == Square::None {
            let mut player1 = Span::from(format!("{} Player1", self.game.scores.0));
            let mut player2 = Span::from(format!("Player2 {}", self.game.scores.1));
            if self.game.turn == Square::X {
                player1 = player1.style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                player2 = player2.style(Style::new().add_modifier(Modifier::REVERSED));
            }
            text = Text::from(ratatui::prelude::Line::from(vec![player1, Span::from(" | "), player2]));
        } else {
            if self.game.winner.0 == Draw {
                text = Text::from("Draw!").style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                text = Text::from(if self.game.winner.0 == Square::X { "Player1 wins!" } else { "Player2 wins!" }).style(Style::new().add_modifier(Modifier::REVERSED));
            }
        }
        frame.render_widget(Paragraph::new(text).centered(), layout[1]);
        frame.render_widget(canvas, layout[3]);

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
        }
    }
}

impl LocalGame {
    pub fn new() -> Self {
        LocalGame { game: Game::new(), has_menu_open: false, menu_state: ListState::default().with_selected(Some(0)), selected: (1.0, 1.0) }
    }
}