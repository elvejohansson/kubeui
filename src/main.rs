use std::io::Result;

pub use app::*;

mod app;
mod ui;

fn main() -> Result<()> {
    return App::run();
}
