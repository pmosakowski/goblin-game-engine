extern crate glium;

struct Window {
    width: i32,
    height: i32,
}

impl Window {
    pub fn new(x: i32, y: i32) -> Window {
        Window {
            width: x,
            height: y,
        }
    }
}

mod tests;
