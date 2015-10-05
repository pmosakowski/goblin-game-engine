use super::*;
use std::cell::RefCell;
use std::rc::Rc;

struct MockLogHandler {
    buffer: Rc<RefCell<Vec<String>>>,
}

impl MockLogHandler {
    pub fn new() -> MockLogHandler {
        MockLogHandler {
            buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get_buffer(&self) -> Rc<RefCell<Vec<String>>> {
        self.buffer.clone()
    }
}

impl LogHandler for MockLogHandler {
    fn output(&mut self, msg: &String) {
        self.buffer.borrow_mut().push(msg.clone());
    }
}

#[test]
fn log_creation() {
    let log_handler = Box::new(MockLogHandler::new());
    let buffer = log_handler.get_buffer();
    let mut log = Logger::new(log_handler);

    let message = String::from("Test message");
    log.debug(&message);

    assert_eq!(buffer.borrow_mut().pop().unwrap(), message);
}
