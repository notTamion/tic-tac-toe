use std::io::{Stdout, stdout};
use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{ExecutableCommand};
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{Terminal};
use crate::action::Action;
use crate::components::Component;
use crate::components::main_menu::MainMenu;

type Tui = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
    tui: Tui,
    component: Box<dyn Component + Send>,
}

impl App {
    pub async fn start() -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Self::set_panic_hook();
        let mut app = App { tui: Terminal::new(CrosstermBackend::new(stdout()))?, component: Box::new(MainMenu::new()) };
        app.run().await
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            let update_action = self.component.update().await.wrap_err("Failed to update component")?;
            if !self.handle_action(update_action) {
                break;
            }

            self.tui.draw(|frame| {
                self.component.render(frame, frame.size());
            })?;

            let event_action = self.component.handle_event().await.wrap_err("Failed to handle events")?;
            if !self.handle_action(event_action) {
                break;
            }
        }
        Self::stop();
        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => false,
            Action::ChangeComponent(new_component) => {
                self.component = new_component;
                true
            }
            _ => true
        }

    }

    fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Self::stop();
            hook(panic_info);
            eprintln!("Please provide errors to https://github.com/notTamion/github-tui/issues");
        }));
    }

    fn stop() {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
