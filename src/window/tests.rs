use super::Window;

#[test]
fn test_window_creation() {
    let win = Window::new(640, 480).unwrap();
    assert_eq!(win.width, 640);
    assert_eq!(win.height, 480);
}
