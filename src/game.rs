use crate::game::Square::X;

pub struct Game {
    pub(crate) board: [[Square; 3]; 3],
    pub(crate) scores: (u8, u8),
    pub(crate) winner: (Square, f64, f64, f64, f64),
    pub(crate) turn: Square,
}

impl Game {
    pub fn new() -> Self {
        Game { board: [[Square::None, Square::None, Square::None], [Square::None, Square::None, Square::None], [Square::None, Square::None, Square::None]], scores: (0, 0), winner: (Square::None, 0.0, 0.0, 0.0, 0.0), turn: X }
    }

    pub fn hit(&mut self, sx: f64, sy: f64) {
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
                            break;
                        }
                    } else {
                        ax2 = sx - x;
                        ay2 = sy - y;
                        if ax2 <= 2.0 && ax2 >= 0.0 && ay2 <= 2.0 && ay2 >= 0.0 {
                            if self.turn == self.board[ax2 as usize][ay2 as usize] {
                                if self.turn == Square::X {
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
                            }
                        }
                    }
                }
            }
        }

        match self.turn {
            Square::X => {
                self.board[sx as usize][sy as usize] = Square::X;
                self.turn = Square::Circle;
            }
            Square::Circle => {
                self.board[sx as usize][sy as usize] = Square::Circle;
                self.turn = Square::X
            }
            _ => ()
        }

        if self.winner.0 == Square::None {
            let mut cancel = true;
            self.board.iter().for_each(|s| s.iter().for_each(|slot| {if slot == &Square::None {cancel = false}}));
            if cancel {
                self.winner.0 = Square::Draw;
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
    }
}
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Square {
    Circle,
    X,
    None,
    Draw,
}
