mod app;
mod game;
mod snake;
mod board;

use app::App;

fn main() {
    App::new("Rust Snake")
        .create_window((500.0, 500.0))
        .background_color([1.0, 1.0, 1.0, 1.0])
        .run();
}
