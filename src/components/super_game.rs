use async_trait::async_trait;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use crate::action::Action;
use crate::components::Component;
use crate::components::game::{Game, Square};

pub struct SuperGame {
    pub managing_game: Game,
    pub games: Vec<Vec<Game>>,
    pub selecting_game: bool,
}

#[async_trait]
impl Component for SuperGame {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<Action> {
        if self.selecting_game {
            if key_event.code == KeyCode::Enter {
                let game = &mut self.games[self.managing_game.selected.0 as usize][self.managing_game.selected.1 as usize];
                if game.winner.0 == Square::None {
                    self.selecting_game = false;
                    self.managing_game.show_selector = false;
                    game.show_selector = true;
                    game.turn = self.managing_game.turn;
                }
            } else {
                self.managing_game.handle_key_event(key_event).await?;
            }
        } else {
            let game = &mut self.games[self.managing_game.selected.0 as usize][self.managing_game.selected.1 as usize];
            let selected = game.selected;
            game.handle_key_event(key_event).await?;
            if game.turn != self.managing_game.turn {
                if game.winner.0 != Square::None {
                    self.managing_game.hit();
                }
                self.managing_game.selected = selected;
                self.managing_game.turn = game.turn;
                game.show_selector = false;
                let new_game = &mut self.games[selected.0 as usize][selected.1 as usize];
                if new_game.winner.0 != Square::None {
                    self.selecting_game = true;
                    self.managing_game.selected = (1.0, 1.0);
                    self.managing_game.show_selector = true;
                } else {
                    new_game.show_selector = true;
                    new_game.turn = self.managing_game.turn;
                }
            }
        }
        Ok(Action::None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut game_areas: Vec<Vec<Rect>> = Vec::with_capacity(self.games.len());
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1); self.games.len()])
            .spacing(2)
            .split(area);
        for column in 0..columns.len() {
            let mut rows =Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1); self.games.len()])
                .spacing(2)
                .split(columns[column]).to_vec();
            rows.reverse();
            game_areas.push(rows);
        }
        for x in 0..self.games.len() {
            for y in 0..self.games.len() {
                self.games[x][y].render(frame, game_areas[x][y]);
            }
        }
        self.managing_game.render(frame, area);
    }
}

impl SuperGame {
    pub fn new() -> Self {
        let mut managing_game = Game::new();
        managing_game.line_color = Color::Yellow;
        let mut games = vec![vec![Game::new(); 3]; 3];
        for x in 0..games.len() {
            for y in 0..games.len() {
                games[x][y].show_selector = false;
            }
        }
        SuperGame { managing_game, games, selecting_game: true }
    }

    pub fn restart(&mut self) {
        self.rematch();
        self.managing_game.scores = (0, 0);
    }

    pub fn rematch(&mut self) {
        let mut games = vec![vec![Game::new(); self.games.len()]; self.games.len()];
        for x in 0..games.len() {
            for y in 0..games.len() {
                games[x][y].show_selector = false;
                games[x][y].set_size(self.games.len());
            }
        }
        self.games = games;
        self.managing_game.rematch();
        self.selecting_game = true;
    }

    pub fn set_size(&mut self, num: usize) {
        if num > 2 {
            self.managing_game.selected = ((num / 2) as f64, (num / 2) as f64);
            self.managing_game.board = vec![vec![Square::None; num]; num];
            let mut games = vec![vec![Game::new(); num]; num];
            for x in 0..games.len() {
                for y in 0..games.len() {
                    games[x][y].show_selector = false;
                    games[x][y].set_size(num);
                }
            }
            self.games = games;
        }
    }
}