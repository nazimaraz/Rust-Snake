use piston_window::*;
use piston_window::types::Color;
use crate::game::Direction;
use crate::app::Component;

struct Position {
    x: f64, y: f64,
}

struct Size {
    x: f64, y: f64,
}

pub struct Snake {
    pub direction: Direction,
    position: Position,
    size: Size,
    color: Color,
}

pub trait SnakeComponent: Component {
    fn new() -> Snake;
}

impl Component for Snake {
    fn update(&mut self, _delta_time: f64) {}
    
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(self.color, [self.position.x, self.position.y, self.size.x, self.size.y], context.transform, g2d);
    }
}

impl SnakeComponent for Snake {
    fn new() -> Snake {
        Snake {
            direction: Direction::Right,
            position: Position { x: 10.0, y: 5.0 },
            size: Size { x: 20.0, y: 20.0 },
            color: [0.5, 0.5, 0.5, 1.0],
        }
    }
}
