use piston_window::*;
use piston_window::types::Color;
use crate::game::Direction;
use crate::app::Component;
use crate::game::Position;
use crate::game::BLOCK_SIZE;

struct Size {
    x: f64, y: f64,
}

pub struct Snake {
    pub snake_body: Vec<Position>,
    window_size: (f64, f64),
    pub direction: Direction,
    size: Size,
    color: Color,
    movement_duration: f64,
    movement_timer: f64
}

pub trait SnakeComponent: Component {
    fn new(window_size: (f64, f64)) -> Self;
    fn handle_movement(&mut self);
    fn is_collide(&mut self) -> bool;
    fn is_ready_for_move(&mut self) -> bool;
    fn movement(&mut self);
}

impl Component for Snake {
    fn update(&mut self, delta_time: f64) {
        self.movement_timer += delta_time;
        self.handle_movement();
    }
    
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        for body_part in &self.snake_body {
            rectangle([0.0, 0.0, 0.0, 1.0], [body_part.x-0.5, body_part.y-0.5, self.size.x+1., self.size.y+1.], context.transform, g2d);
            rectangle(self.color, [body_part.x, body_part.y, self.size.x, self.size.y], context.transform, g2d);
        }
    }
}

impl SnakeComponent for Snake {
    fn new(window_size: (f64, f64)) -> Self {
        Self {
            snake_body: vec![
                Position { x: 125.0, y: 25.0 },
                Position { x: 100.0, y: 25.0 },
                Position { x: 75.0, y: 25.0 },
                Position { x: 50.0, y: 25.0 },
                Position { x: 25.0, y: 25.0 },
            ],
            window_size,
            direction: Direction::Right,
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
        for i in (1..self.snake_body.len()).rev() {
            self.snake_body[i].x = self.snake_body[i - 1].x;
            self.snake_body[i].y = self.snake_body[i - 1].y;
        }

        let head: &mut Position = &mut self.snake_body[0];
        match self.direction {
            Direction::Up => head.y -= BLOCK_SIZE,
            Direction::Down => head.y += BLOCK_SIZE,
            Direction::Right => head.x += BLOCK_SIZE,
            Direction::Left => head.x -= BLOCK_SIZE,
        }

        head.x = if head.x > self.window_size.0 - BLOCK_SIZE { 0.0 }
            else if head.x < 0.0 { self.window_size.0 - BLOCK_SIZE }
            else { head.x };

        head.y = if head.y > self.window_size.1 - BLOCK_SIZE { 0.0 }
            else if head.y < 0.0 { self.window_size.1 - BLOCK_SIZE }
            else { head.y };
    }

    fn is_collide(&mut self) -> bool {
        let head: &Position = &self.snake_body[0];
        for i in 1..self.snake_body.len() {
            let body_part: &Position = &self.snake_body[i];
            if self.snake_body.len() > 4 && head.x == body_part.x && head.y == body_part.y {
                return true
            }
        }
        false
    }

    fn is_ready_for_move(&mut self) -> bool {
        self.movement_timer > self.movement_duration
    }
}
