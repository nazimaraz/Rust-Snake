use piston_window::*;
use piston_window::types::Color;
use crate::game::Direction;
use crate::app::Component;

const BLOCK_SIZE: f64 = 10.0;

struct Position {
    x: f64, y: f64,
}

struct Size {
    x: f64, y: f64,
}

pub struct Snake {
    window_size: (f64, f64),
    pub direction: Direction,
    position: Position,
    size: Size,
    color: Color,
    movement_duration: f64,
    movement_timer: f64
}

pub trait SnakeComponent: Component {
    fn new(window_size: (f64, f64)) -> Self;
    fn handle_movement(&mut self);
    fn is_ready_for_move(&mut self) -> bool;
    fn movement(&mut self);
}

impl Component for Snake {
    fn update(&mut self, delta_time: f64) {
        self.movement_timer += delta_time;
        self.handle_movement();
    }
    
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(self.color, [self.position.x, self.position.y, self.size.x, self.size.y], context.transform, g2d);
    }
}

impl SnakeComponent for Snake {
    fn new(window_size: (f64, f64)) -> Self {
        Self {
            window_size: window_size,
            direction: Direction::Right,
            position: Position { x: 10.0, y: 5.0 },
            size: Size { x: BLOCK_SIZE, y: BLOCK_SIZE },
            color: [0.5, 0.5, 0.5, 1.0],
            movement_duration: 0.1,
            movement_timer: 0.0,
        }
    }

    fn handle_movement(&mut self) {
        if self.is_ready_for_move() {
            self.movement_timer = 0.0;
            self.movement();
        }
    }

    fn movement(&mut self) {
        match self.direction {
            Direction::Up => self.position.y -= BLOCK_SIZE,
            Direction::Down => self.position.y += BLOCK_SIZE,
            Direction::Right => self.position.x += BLOCK_SIZE,
            Direction::Left => self.position.x -= BLOCK_SIZE,
        }

        self.position.x = if self.position.x > self.window_size.0 - BLOCK_SIZE { 0.0 }
            else if self.position.x < 0.0 { self.window_size.0 - BLOCK_SIZE }
            else { self.position.x };

        self.position.y = if self.position.y > self.window_size.1 - BLOCK_SIZE { 0.0 }
            else if self.position.y < 0.0 { self.window_size.1 - BLOCK_SIZE }
            else { self.position.y };
    }

    fn is_ready_for_move(&mut self) -> bool {
        self.movement_timer > self.movement_duration
    }
}
