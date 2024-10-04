use async_trait::async_trait;
use ratatui::crossterm::event::KeyCode::Char;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Color;
use crate::action::Action;
use crate::components::Component;
use crate::components::game::{Game, Square};

pub struct SuperGame {
    pub managing_game: Game,
    pub games: [[Game; 3]; 3],
    pub selecting_game: bool,
}

#[async_trait]
impl Component for SuperGame {
    async fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<Action> {
        if self.selecting_game {
            match key_event.code {
                Char('k') | KeyCode::Up => self.managing_game.selected.1 = (self.managing_game.selected.1 + 1.0).min(2.0),
                Char('j') | KeyCode::Down => self.managing_game.selected.1 = (self.managing_game.selected.1 - 1.0).max(0.0),
                Char('l') | KeyCode::Right => self.managing_game.selected.0 = (self.managing_game.selected.0 + 1.0).min(2.0),
                Char('h') | KeyCode::Left => self.managing_game.selected.0 = (self.managing_game.selected.0 - 1.0).max(0.0),
                KeyCode::Enter => {
                    self.selecting_game = false;
                    self.managing_game.show_selector = false;
                    let game = &mut self.games[self.managing_game.selected.0 as usize][self.managing_game.selected.1 as usize];
                    game.show_selector = true;
                    game.turn = self.managing_game.turn;
                }
                _ => ()
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
        let games_columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .spacing(2)
            .split(area);
        let games_rows1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .spacing(2)
            .split(games_columns[0]);
        let games_rows2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .spacing(2)
            .split(games_columns[1]);
        let games_rows3 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .spacing(2)
            .split(games_columns[2]);
        let game_areas:[[Rect; 3]; 3] = [[games_rows1[0], games_rows2[0], games_rows3[0]], [games_rows1[1], games_rows2[1], games_rows3[1]], [games_rows1[2], games_rows2[2], games_rows3[2]]];
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
        let mut games = [[Game::new(), Game::new(), Game::new()], [Game::new(), Game::new(), Game::new()], [Game::new(), Game::new(), Game::new()]];
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
        let mut games = [[Game::new(), Game::new(), Game::new()], [Game::new(), Game::new(), Game::new()], [Game::new(), Game::new(), Game::new()]];
        for x in 0..games.len() {
            for y in 0..games.len() {
                games[x][y].show_selector = false;
            }
        }
        self.games = games;
        self.managing_game.rematch();
        self.selecting_game = true;
    }
}