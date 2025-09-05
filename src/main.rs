use crate::app::App;

mod app;
mod args;
mod blur;
mod models;

fn main() {
    App::new().run();
}
