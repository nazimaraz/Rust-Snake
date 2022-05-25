use piston_window::*;
use crate::app::Component;

pub struct Board {}

pub trait BoardComponent: Component {
    fn new() -> Board;
}

impl Component for Board {
    fn update(&mut self, _delta_time: f64) {}
    
    fn draw(&self, _context: &Context, _g2d: &mut G2d) {}
}

impl BoardComponent for Board {
    fn new() -> Board {
        Board {}
    }
}
