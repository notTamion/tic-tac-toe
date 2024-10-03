use ratatui::widgets::canvas::Rectangle;
use async_trait::async_trait;
use ratatui::crossterm::event::KeyCode::Char;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Circle, Line};
use crate::action::Action;
use crate::components::Component;
use crate::components::game::Square::{Draw, X};

pub struct Game {
    pub selected: (f64, f64),
    pub board: [[Square; 3]; 3],
    pub scores: (u8, u8),
    pub winner: (Square, f64, f64, f64, f64),
    pub turn: Square,
    pub line_color: Color,
    pub show_selector: bool,
}

#[async_trait]
impl Component for Game {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<Action> {
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
            KeyCode::Enter => {
                self.hit();
            }
            _ => ()
        }
        Ok(Action::None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let canvas = Canvas::default()
            .x_bounds([-99.0, 99.0])
            .y_bounds([-99.0, 99.0])
            .paint(|ctx| {
                ctx.draw(&Line {
                    x1: -33.0,
                    y1: 99.0,
                    x2: -33.0,
                    y2: -99.0,
                    color: self.line_color,
                });
                ctx.draw(&Line {
                    x1: 33.0,
                    y1: 99.0,
                    x2: 33.0,
                    y2: -99.0,
                    color: self.line_color,
                });
                ctx.draw(&Line {
                    x1: -99.0,
                    y1: 33.0,
                    x2: 99.0,
                    y2: 33.0,
                    color: self.line_color,
                });
                ctx.draw(&Line {
                    x1: -99.0,
                    y1: -33.0,
                    x2: 99.0,
                    y2: -33.0,
                    color: self.line_color,
                });
                if self.show_selector == true {
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
                if self.winner.0 != Square::None && self.winner.0 != Draw {
                    ctx.draw(&Line {
                        x1: self.winner.1,
                        y1: self.winner.2,
                        x2: self.winner.3,
                        y2: self.winner.4,
                        color: Color::Red,
                    })
                }
            });
        frame.render_widget(canvas, area);
    }
}

impl Game {
    pub fn new() -> Self {
        Game{ selected: (1.0, 1.0), board: [[Square::None, Square::None, Square::None], [Square::None, Square::None, Square::None], [Square::None, Square::None, Square::None]], scores: (0, 0), winner: (Square::None, 0.0, 0.0, 0.0, 0.0), turn: X, line_color: Color::White, show_selector: true }
    }

    pub fn hit(&mut self) {
        let sx = self.selected.0;
        let sy = self.selected.1;
        if self.board[sx as usize][sy as usize] != Square::None || self.winner.0 != Square::None {
            return;
        }

        for x in if sx == 0.0 { 0 } else { -1 }..if sx == 2.0 { 1 } else { 2 } {
            for y in if sy == 0.0 { 0 } else { -1 }..if sy == 2.0 { 1 } else { 2 } {
                let x = x as f64;
                let y = y as f64;
                if x == 0.0 && y == 0.0 {
                    continue;
                }
                let ax = sx + x;
                let ay = sy + y;
                if self.turn == self.board[ax as usize][ay as usize] {
                    let mut ax2 = ax+x;
                    let mut ay2 = ay+y;
                    if ax2 <= 2.0 && ax2 >= 0.0 && ay2 <= 2.0 && ay2 >= 0.0 {
                        if self.turn == self.board[ax2 as usize][ay2 as usize] {
                            if self.turn == X {
                                self.scores.0 += 1;
                            } else {
                                self.scores.1 += 1;
                            }
                            self.winner = (self.turn,
                                           -30.0*x-66.0 + 66.0 * sx,
                                           30.0*y+66.0 - 66.0 * sy,
                                           30.0*x-66.0 + 66.0 * ax2,
                                           -30.0*y+66.0 - 66.0 * ay2,
                            );
                            self.show_selector = false;
                            break;
                        }
                    } else {
                        ax2 = sx - x;
                        ay2 = sy - y;
                        if ax2 <= 2.0 && ax2 >= 0.0 && ay2 <= 2.0 && ay2 >= 0.0 {
                            if self.turn == self.board[ax2 as usize][ay2 as usize] {
                                if self.turn == X {
                                    self.scores.0 += 1;
                                } else {
                                    self.scores.1 += 1;
                                }
                                self.winner = (self.turn,
                                               -30.0*x-66.0 + 66.0 * ax2,
                                               30.0*y+66.0 - 66.0 * ay2,
                                               30.0*x-66.0 + 66.0 * ax,
                                               -30.0*y+66.0 - 66.0 * ay,
                                );
                                self.show_selector = true;
                            }
                        }
                    }
                }
            }
        }

        match self.turn {
            X => {
                self.board[sx as usize][sy as usize] = X;
                self.turn = Square::Circle;
            }
            Square::Circle => {
                self.board[sx as usize][sy as usize] = Square::Circle;
                self.turn = X
            }
            _ => ()
        }
        self.selected = (1.0, 1.0);

        if self.winner.0 == Square::None {
            let mut cancel = true;
            self.board.iter().for_each(|s| s.iter().for_each(|slot| {if slot == &Square::None {cancel = false}}));
            if cancel {
                self.winner.0 = Draw;
            }
        }
    }

    pub fn restart(&mut self) {
        self.rematch();
        self.scores = (0, 0);
    }

    pub fn rematch(&mut self) {
        self.board = [[Square::None, Square::None, Square::None], [Square::None, Square::None, Square::None], [Square::None, Square::None, Square::None]];
        self.winner = (Square::None, 0.0, 0.0, 0.0, 0.0);
        self.turn = X;
        self.show_selector = true;
        self.selected = (1.0, 1.0);
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Square {
    Circle,
    X,
    None,
    Draw,
}
