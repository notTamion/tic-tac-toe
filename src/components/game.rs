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
use crate::components::game::Square::{Draw, Unoccupied};
use crate::components::game_selection::GameSelection;

pub struct Game {
    board: [[Square; 3]; 3],
    selected: (f64, f64),
    turn: Square,
    winner: (Square, f64, f64, f64, f64),
    menu_state: ListState,
    has_menu_open: bool,
    scores: (u8, u8),
}

#[derive(PartialEq, Eq)]
enum Square {
    Circle,
    X,
    Unoccupied,
    Draw,
}

#[async_trait]
impl Component for Game {
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
                            self.board = [[Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied]];
                            self.winner = (Unoccupied, 0.0, 0.0, 0.0, 0.0);
                            self.turn = Square::X;
                            self.selected = (1.0, 1.0);
                            self.menu_state = ListState::default().with_selected(Some(0));
                            self.has_menu_open = false;
                        }
                        2 => {
                            self.board = [[Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied]];
                            self.winner = (Unoccupied, 0.0, 0.0, 0.0, 0.0);
                            self.turn = Square::X;
                            self.selected = (1.0, 1.0);
                            self.menu_state = ListState::default().with_selected(Some(0));
                            self.has_menu_open = false;
                            self.scores = (0, 0);
                        }
                        3 => return Ok(Action::ChangeComponent(Box::new(GameSelection::new()))),
                        4 => return Ok(Action::Quit),

                        _ => {}
                    }
                }
                _ => {}
            }

            return Ok(Action::None);
        } else if self.winner.0 == Unoccupied {
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
                    if self.board[self.selected.0 as usize][self.selected.1 as usize] != Unoccupied {
                        return Ok(Action::None);
                    }

                    for x in if self.selected.0 == 0.0 { 0 } else { -1 }..if self.selected.0 == 2.0 { 1 } else { 2 } {
                        for y in if self.selected.1 == 0.0 { 0 } else { -1 }..if self.selected.1 == 2.0 { 1 } else { 2 } {
                            let x = x as f64;
                            let y = y as f64;
                            if x == 0.0 && y == 0.0 {
                                continue;
                            }
                            let ax = self.selected.0 + x;
                            let ay = self.selected.1 + y;
                            if self.turn == self.board[ax as usize][ay as usize] {
                                let mut ax2 = ax+x;
                                let mut ay2 = ay+y;
                                if ax2 <= 2.0 && ax2 >= 0.0 && ay2 <= 2.0 && ay2 >= 0.0 {
                                    if self.turn == self.board[ax2 as usize][ay2 as usize] {
                                        if self.turn == Square::X {
                                            self.scores.0 += 1;
                                        } else {
                                            self.scores.1 += 1;
                                        }
                                        self.has_menu_open = true;
                                        self.winner = (if self.turn == Square::X { Square::X } else { Square::Circle },
                                                       -30.0*x-66.0 + 66.0 * self.selected.0,
                                                       30.0*y+66.0 - 66.0 * self.selected.1,
                                                       30.0*x-66.0 + 66.0 * ax2,
                                                       -30.0*y+66.0 - 66.0 * ay2,
                                        );
                                        break;
                                    }
                                } else {
                                    ax2 = self.selected.0 - x;
                                    ay2 = self.selected.1 - y;
                                    if ax2 <= 2.0 && ax2 >= 0.0 && ay2 <= 2.0 && ay2 >= 0.0 {
                                        if self.turn == self.board[ax2 as usize][ay2 as usize] {
                                            if self.turn == Square::X {
                                                self.scores.0 += 1;
                                            } else {
                                                self.scores.1 += 1;
                                            }
                                            self.has_menu_open = true;
                                            self.winner = (if self.turn == Square::X { Square::X } else { Square::Circle },
                                                           -30.0*x-66.0 + 66.0 * ax2,
                                                           30.0*y+66.0 - 66.0 * ay2,
                                                           30.0*x-66.0 + 66.0 * ax,
                                                           -30.0*y+66.0 - 66.0 * ay,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    match self.turn {
                        Square::X => {
                            self.board[self.selected.0 as usize][self.selected.1 as usize] = Square::X;
                            self.turn = Square::Circle;
                        }
                        Square::Circle => {
                            self.board[self.selected.0 as usize][self.selected.1 as usize] = Square::Circle;
                            self.turn = Square::X
                        }
                        _ => ()
                    }
                    self.selected = (1.0, 1.0);
                    if self.winner.0 == Unoccupied {
                        let mut cancel = true;
                        self.board.iter().for_each(|s| s.iter().for_each(|slot| {if slot == &Unoccupied {cancel = false}}));
                        if cancel {
                            self.winner.0 = Draw;
                            self.has_menu_open = true;
                        }
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
                if self.winner.0 == Unoccupied || self.winner.0 == Draw {
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
                        match self.board.get(x as usize).unwrap().get(y as usize).unwrap() {
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
                if self.winner.0 != Unoccupied && self.winner.0 != Draw {
                    ctx.draw(&Line {
                        x1: self.winner.1,
                        y1: self.winner.2,
                        x2: self.winner.3,
                        y2: self.winner.4,
                        color: Color::Red,
                    })
                }
            });
        let text;
        if self.winner.0 == Unoccupied {
            let mut player1 = Span::from(format!("{} Player1", self.scores.0));
            let mut player2 = Span::from(format!("Player2 {}", self.scores.1));
            if self.turn == Square::X {
                player1 = player1.style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                player2 = player2.style(Style::new().add_modifier(Modifier::REVERSED));
            }
            text = Text::from(ratatui::prelude::Line::from(vec![player1, Span::from(" | "), player2]));
        } else {
            if self.winner.0 == Draw {
                text = Text::from("Draw!").style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                text = Text::from(if self.winner.0 == Square::X { "Player1 wins!" } else { "Player2 wins!" }).style(Style::new().add_modifier(Modifier::REVERSED));
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

impl Game {
    pub fn new() -> Self {
        Game { scores: (0, 0), has_menu_open: false, menu_state: ListState::default().with_selected(Some(0)), winner: (Unoccupied, 0.0, 0.0, 0.0, 0.0), turn: Square::X, selected: (1.0, 1.0), board: [[Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied]] }
    }
}