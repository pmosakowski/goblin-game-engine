extern crate glium;

#[cfg(test)]
mod tests;

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
