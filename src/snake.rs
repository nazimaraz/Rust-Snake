use piston_window::*;
use piston_window::types::Color;
use crate::game::Direction;

struct Position {
    x: f64, y: f64,
}

pub struct Snake {
    pub direction: Direction,
    position: Position,
    color: Color,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            direction: Direction::Right,
            position: Position { x: 10.0, y: 5.0 },
            color: [0.5, 0.5, 0.5, 1.0],
        }
    }

    pub fn update(&mut self, _delta_time: f64) {

    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(self.color, [self.position.x, self.position.y, 20.0, 20.0], context.transform, g2d);
    }
}
