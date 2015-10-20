extern crate goblin;

use goblin::window::xcb::Window; 

fn main() {
    let win = Window::new(640, 480).unwrap();
    println!("{:?}",win);
    std::thread::sleep_ms(5000);
    //win.update();
    //win.close();
}
