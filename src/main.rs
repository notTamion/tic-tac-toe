mod app;
mod components;
mod action;
mod game;

use color_eyre::Result;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    App::start().await
}
