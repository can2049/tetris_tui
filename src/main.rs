mod app;
mod game;
mod input;
mod ui;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new().run()?;
    Ok(())
}
