use piston_window::*;
use crate:: {
    snake::Snake,
    board::Board,
    fruit::Fruit,
    app::Component,
    snake::SnakeComponent,
    board::BoardComponent,
    fruit::FruitComponent,
};
pub const BLOCK_SIZE: f64 = 25.0;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub enum Direction {
    Up, Down, Left, Right,
}

pub struct Game {
    is_game_over: bool,
    snake: Snake,
    board: Board,
    fruit: Fruit,
}

pub trait GameComponent: Component {
    fn new(window_size: (f64, f64)) -> Game;
    fn key_pressed(&mut self, new_direction: Key);
    fn handle_game(&mut self);
}

impl Component for Game {
    fn update(&mut self, delta_time: f64) {
        if !self.is_game_over {
            self.handle_game();
            self.snake.update(delta_time);
            self.board.update(delta_time);
        }
    }

    fn draw(&self, context: &Context, g2d: &mut G2d) {
        if !self.is_game_over {
            self.board.draw(context, g2d);
            self.snake.draw(context, g2d);
            self.fruit.draw(context, g2d);
        }
    }
}

impl GameComponent for Game {
    fn new(window_size: (f64, f64)) -> Game {
        Game {
            is_game_over: false,
            snake: Snake::new(window_size),
            board: Board::new(window_size),
            fruit: Fruit::new(window_size),
        }
    }

    fn key_pressed(&mut self, new_direction: Key) {
        match (&self.snake.direction_input, new_direction) {
            (Direction::Right | Direction::Left, Key::Up | Key::W) => self.snake.direction_input = Direction::Up, 
            (Direction::Right | Direction::Left, Key::Down | Key::S) => self.snake.direction_input = Direction::Down,
            (Direction::Up | Direction::Down, Key::Left | Key::A) => self.snake.direction_input = Direction::Left,
            (Direction::Up | Direction::Down, Key::Right | Key::D) => self.snake.direction_input = Direction::Right,
            (_, _) => {},
        }
    }

    fn handle_game(&mut self) {
        self.is_game_over = self.snake.is_collide();
        if self.fruit.is_eaten(&self.snake.snake_body[0]) {
            self.fruit.position = self.fruit.get_new_fruit_position();
            'test: loop {
                for body_part in &self.snake.snake_body {
                    if (self.fruit.position.x == body_part.x) & (self.fruit.position.y == body_part.y) {
                        self.fruit.position = self.fruit.get_new_fruit_position();
                        break;
                    }
                    break 'test;
                }
            }
            let new_body_part = self.snake.snake_body[self.snake.snake_body.len() - 1];
            self.snake.snake_body.push(new_body_part);
        }
    }
}
