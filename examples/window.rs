extern crate goblin;

use goblin::window::Window; 

fn main() {
    let mut win = Window::new(640, 480).unwrap();
    win.update();
    win.close();
}
