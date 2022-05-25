use piston_window::*;
use crate::snake::Snake;
use crate::board::Board;
use crate::app::Component;
use crate::snake::SnakeComponent;
use crate::board::BoardComponent;

pub enum Direction {
    Up, Down, Left, Right,
}

pub struct Game {
    snake: Snake,
    board: Board,
}

pub trait GameComponent: Component {
    fn new() -> Game;
    fn key_pressed(&mut self, key: Key);
}

impl Component for Game {
    fn update(&mut self, delta_time: f64) {
        self.snake.update(delta_time);
        self.board.update(delta_time);
    }

    fn draw(&self, context: &Context, g2d: &mut G2d) {
        self.snake.draw(context, g2d);
        self.board.draw(context, g2d);
    }
}

impl GameComponent for Game {
    fn new() -> Game {
        Game {
            snake: Snake::new(),
            board: Board::new(),
        }
    }

    fn key_pressed(&mut self, key: Key) {
        self.snake.direction = match key {
            Key::Up | Key::W => Direction::Up,
            Key::Down | Key::S => Direction::Down,
            Key::Left | Key::A => Direction::Left,
            Key::Right | Key::D => Direction::Right,
            _ => Direction::Right
        }
    }
}
