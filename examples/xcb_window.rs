extern crate goblin;

use goblin::window::xcb::Window; 

fn main() {
    let mut win = Window::new(640, 480).unwrap();
    win.update();
    println!("{}", win.to_str());
    std::thread::sleep_ms(5000);
    win.close();
}
