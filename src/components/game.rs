use async_trait::async_trait;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::crossterm::event::KeyCode::Char;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Text;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::canvas::{Canvas, Circle, Line, Rectangle};
use ratatui::widgets::Paragraph;
use crate::action::Action;
use crate::components::Component;
use crate::components::game::Square::Unoccupied;

pub struct Game {
    board: [[Square; 3]; 3],
    selected: (f64, f64),
    turn: Square,
    winner: (Square, f64, f64, f64, f64)
}

#[derive(PartialEq, Eq)]
enum Square {
    Circle,
    X,
    Unoccupied,
}

#[async_trait]
impl Component for Game {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<Action> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(Action::None);
        }

        match key_event.code {
            Char('q') => return Ok(Action::Quit),
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
            KeyCode::Enter => {
                if !matches!(self.board[self.selected.0 as usize][self.selected.1 as usize], Unoccupied) {
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
                        if &self.turn == self.board.get(ax as usize).unwrap().get(ay as usize).unwrap() {
                            let mut ax2 = ax+x;
                            let mut ay2 = ay+y;
                            if ax2 <= 2.0 && ax2 >= 0.0 && ay2 <= 2.0 && ay2 >= 0.0 {
                                if &self.turn == self.board.get(ax2 as usize).unwrap().get(ay2 as usize).unwrap() {
                                    self.winner = (if matches!(&self.turn, Square::X) { Square::X } else { Square::Circle },
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
                                    if &self.turn == self.board.get(ax2 as usize).unwrap().get(ay2 as usize).unwrap() {
                                        self.winner = (if matches!(&self.turn, Square::X) { Square::X } else { Square::Circle },
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
            }
            _ => ()
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
                ctx.draw(&Rectangle {
                    x: -95.0 + 66.0 * self.selected.0,
                    y: 37.0 - 66.0 * self.selected.1,
                    color: Color::Green,
                    height: 58.0,
                    width: 58.0,
                });
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
                if !matches!(self.winner.0, Unoccupied) {
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
        if matches!(self.winner.0, Unoccupied) {
            let mut player1 = Span::from("Player1");
            let mut player2 = Span::from("Player2");
            if matches!(self.turn, Square::X) {
                player1 = player1.style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                player2 = player2.style(Style::new().add_modifier(Modifier::REVERSED));
            }
            text = Text::from(ratatui::prelude::Line::from(vec![player1, Span::from(" | "), player2]));
        } else {
            if matches!(self.winner.0, Square::X) {
                text = Text::from("Player1 wins!").style(Style::new().add_modifier(Modifier::REVERSED));
            } else {
                text = Text::from("Player2 wins!").style(Style::new().add_modifier(Modifier::REVERSED));
            }
        }
        frame.render_widget(Paragraph::new(text).centered(), layout[1]);
        frame.render_widget(canvas, layout[3]);
    }
}

impl Game {
    pub fn new() -> Self {
        Game { winner: (Square::Unoccupied, 0.0, 0.0, 0.0, 0.0), turn: Square::X, selected: (1.0, 1.0), board: [[Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied], [Unoccupied, Unoccupied, Unoccupied]] }
    }
}