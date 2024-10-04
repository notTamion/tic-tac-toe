use ratatui::widgets::canvas::{Painter, Rectangle, Shape};
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
    pub board: Vec<Vec<Square>>,
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
            Char('k') | KeyCode::Up => self.selected.1 = (self.selected.1 + 1.0).min(self.board.len() as f64 - 1.0),
            Char('j') | KeyCode::Down => self.selected.1 = (self.selected.1 - 1.0).max(0.0),
            Char('l') | KeyCode::Right => self.selected.0 = (self.selected.0 + 1.0).min(self.board.len() as f64 - 1.0),
            Char('h') | KeyCode::Left => self.selected.0 = (self.selected.0 - 1.0).max(0.0),
            KeyCode::Enter => self.hit(),
            _ => ()
        }
        Ok(Action::None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let canvas = Canvas::default()
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(|ctx| {
                let shift = 100.0/self.board.len() as f64;
                for i in 1..self.board.len() {
                    let shifted = shift * i as f64;
                    ctx.draw(&Line {
                        x1: shifted,
                        y1: 0.0,
                        x2: shifted,
                        y2: 100.0,
                        color: self.line_color,
                    });
                    ctx.draw(&Line {
                        x1: 0.0,
                        y1: shifted,
                        x2: 100.0,
                        y2: shifted,
                        color: self.line_color,
                    });
                }

                if self.show_selector == true {
                    ctx.draw(&Rectangle {
                        x: 0.1*shift + shift * self.selected.0,
                        y: 0.1*shift + shift * self.selected.1,
                        color: Color::Green,
                        height: shift*0.8,
                        width: shift*0.8,
                    });
                }
                for x in 0..self.board.len() {
                    for y in 0..self.board.len() {
                        let x = x as f64;
                        let y = y as f64;
                        match self.board.get(x as usize).unwrap().get(y as usize).unwrap() {
                            Square::Circle => {
                                ctx.draw(&Circle {
                                    x: shift * (0.5 + x),
                                    y: shift * (0.5 + y),
                                    radius: shift * 0.3,
                                    color: Color::Yellow,
                                })
                            }
                            X => {
                                ctx.draw(&Cross {
                                    x: shift * (0.5 + x),
                                    y: shift * (0.5 + y),
                                    radius: shift * 0.3,
                                    color: Color::Cyan,
                                });
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
        Game{ selected: (1.0, 1.0), board: vec![vec![Square::None; 8]; 8], scores: (0, 0), winner: (Square::None, 0.0, 0.0, 0.0, 0.0), turn: X, line_color: Color::White, show_selector: true }
    }

    pub fn hit(&mut self) {
        let board_size = self.board.len() as f64;
        let selected_x = self.selected.0;
        let selected_y = self.selected.1;
        if self.board[selected_x as usize][selected_y as usize] != Square::None || self.winner.0 != Square::None {
            return;
        }

        for relative_x in if selected_x == 0.0 { 0 } else { -1 }..if selected_x == board_size-1.0 { 1 } else { 2 } {
            for relative_y in if selected_y == 0.0 { 0 } else { -1 }..if selected_y == board_size-1.0 { 1 } else { 2 } {
                let relative_x = relative_x as f64;
                let relative_y = relative_y as f64;
                if relative_x == 0.0 && relative_y == 0.0 {
                    continue;
                }
                let mut base_x = selected_x;
                let mut base_y = selected_y;

                let mut top_x = selected_x;
                let mut top_y = selected_y;

                let mut fail = false;
                while base_x-relative_x < board_size && base_x-relative_x >= 0.0 && base_y-relative_y < board_size && base_y-relative_y >= 0.0 {
                    if self.board[(base_x-relative_x) as usize][(base_y-relative_y) as usize] == self.turn {
                        base_x -= relative_x;
                        base_y -= relative_y;
                    } else {
                        fail = true;
                        break;
                    }
                }
                if fail {continue}

                while top_x+relative_x < board_size && top_x+relative_x >= 0.0 && top_y+relative_y < board_size && top_y+relative_y >= 0.0 {
                    if self.board[(top_x+relative_x) as usize][(top_y+relative_y) as usize] == self.turn {
                        top_x += relative_x;
                        top_y += relative_y;
                    } else {
                        fail = true;
                        break;
                    }
                }
                if fail {continue}
                if self.turn == X {
                    self.scores.0 += 1;
                } else {
                    self.scores.1 += 1;
                }
                let shift = 100.0/self.board.len() as f64;
                self.winner = (self.turn,
                               (base_x+0.5) * shift - relative_x * shift * 0.4,
                               (base_y+0.5) * shift - relative_y * shift * 0.4,
                               (top_x+0.5) * shift + relative_x * shift * 0.4,
                               (top_y+0.5) * shift + relative_y * shift * 0.4,
                );
                self.show_selector = false;
                break;
            }
        }

        match self.turn {
            X => {
                self.board[selected_x as usize][selected_y as usize] = X;
                self.turn = Square::Circle;
            }
            Square::Circle => {
                self.board[selected_x as usize][selected_y as usize] = Square::Circle;
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
        self.board = vec![vec![Square::None; 3]; 3];
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

pub struct Cross {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Cross {
    fn draw(&self, painter: &mut Painter) {
        Line {
            x1: self.x-self.radius,
            y1: self.y-self.radius,
            x2: self.x+self.radius,
            y2: self.y+self.radius,
            color: self.color,
        }.draw(painter);
        Line {
            x1: self.x-self.radius,
            y1: self.y+self.radius,
            x2: self.x+self.radius,
            y2: self.y-self.radius,
            color: self.color,
        }.draw(painter);
    }
}