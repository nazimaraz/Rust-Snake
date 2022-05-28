use rand::prelude::*;
use piston_window::*;
use crate::app::Component;
use crate::game::Position;
use crate::game::BLOCK_SIZE;

pub struct Fruit {
    pub position: Position,
    window_size: (f64, f64),
}

pub trait FruitComponent: Component {
    fn new(window_size: (f64, f64)) -> Fruit;
    fn get_new_fruit_position_with_parameter(window_size: (f64, f64)) -> Position;
    fn get_new_fruit_position(&self) -> Position;
    fn is_eaten(&self, head: &Position) -> bool;
}

impl Component for Fruit {
    fn update(&mut self, _delta_time: f64) {}
    
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        ellipse([1.0, 0.0, 0.0, 1.0], [self.position.x, self.position.y, BLOCK_SIZE, BLOCK_SIZE], context.transform, g2d);
    }
}

impl FruitComponent for Fruit {
    fn new(window_size: (f64, f64)) -> Fruit {
        Fruit {
            window_size,
            position: Fruit::get_new_fruit_position_with_parameter(window_size),
        }
    }

    fn get_new_fruit_position_with_parameter(window_size: (f64, f64)) -> Position {
        let mut rng = rand::thread_rng();
        Position {
            x: (rng.gen::<f64>() * window_size.0 / BLOCK_SIZE).floor() * BLOCK_SIZE,
            y: (rng.gen::<f64>() * window_size.1 / BLOCK_SIZE).floor() * BLOCK_SIZE
        }
    }

    fn get_new_fruit_position(&self) -> Position {
        Fruit::get_new_fruit_position_with_parameter(self.window_size)
    }

    fn is_eaten(&self, head: &Position) -> bool {
        self.position.x == head.x && self.position.y == head.y
    }
}
