mod app;
mod components;
mod action;

use color_eyre::Result;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    App::start().await
}
