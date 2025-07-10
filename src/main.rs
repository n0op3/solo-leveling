use std::io;

use crate::ui::app::App;

pub mod category;
pub mod config;
pub mod data;
pub mod level;
pub mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
