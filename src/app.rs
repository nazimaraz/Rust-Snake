use piston_window::*;
use piston_window::types::Color;
use crate::game::Game;

pub struct App {
    window: PistonWindow,
    window_name: String,
    window_size: (f64, f64),
    background_color: Color,
}

impl App {
    pub fn new(window_name: &str, (window_size_x, window_size_y): (f64, f64)) -> App {
        App {
            window: WindowSettings::new("", [0.0, 0.0]).build().unwrap(),
            window_name: window_name.to_string(),
            window_size: (window_size_x, window_size_y),
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn create_window(&mut self) -> &mut Self {
        let window_settings = WindowSettings::new(&self.window_name, [self.window_size.0, self.window_size.1])
            .exit_on_esc(true);
        self.window = window_settings.build().unwrap();
        return self;
    }

    pub fn background_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.background_color = color;
        return self;
    }

    pub fn run(&mut self) {        
        // Create the game
        let mut game = Game::new();

        while let Some(event) = self.window.next() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                game.key_pressed(key);
            }

            event.update(|arguments| {
                game.update(arguments.dt);
            });

            self.window.draw_2d(&event, |context, g2d, _| {
                clear(self.background_color, g2d);
                game.draw(&context, g2d);
            });
        }
    }
}
