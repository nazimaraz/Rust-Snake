use piston_window::*;
use crate::snake::Snake;
use crate::board::Board;

pub enum Direction {
    Up, Down, Left, Right,
}

pub struct Game {
    snake: Snake,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            board: Board::new(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.snake.direction = match key {
            Key::Up | Key::W => Direction::Up,
            Key::Down | Key::S => Direction::Down,
            Key::Left | Key::A => Direction::Left,
            Key::Right | Key::D => Direction::Right,
            _ => Direction::Right
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.snake.update(delta_time);
        self.board.update(delta_time);
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        self.snake.draw(context, g2d);
        self.board.draw(context, g2d);
    }
}
