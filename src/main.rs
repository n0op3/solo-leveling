use std::io;

use crate::ui::app::App;

pub mod config;
pub mod level;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
