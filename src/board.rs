use piston_window::*;
use crate::app::Component;

pub struct Board {
    window_size: (f64, f64),
}

pub trait BoardComponent: Component {
    fn new(window_size: (f64, f64)) -> Board;
}

impl Component for Board {
    fn update(&mut self, _delta_time: f64) {}
    
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle([1.0, 1.0, 1.0, 1.0], [0.0, 0.0, self.window_size.0, self.window_size.1], context.transform, g2d);
    }
}

impl BoardComponent for Board {
    fn new(window_size: (f64, f64),) -> Board {
        Board {
            window_size
        }
    }
}
