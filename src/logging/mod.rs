#[cfg(test)]
mod tests;

pub struct Logger<'a> {
    handler: Box<LogHandler + 'a>,
}

impl<'a> Logger<'a> {
    pub fn new(handler: Box<LogHandler + 'a>) -> Logger<'a> {
        Logger {
            handler: handler,
        }
    }

    pub fn debug(&mut self, msg: &String) {
        self.handler.output(msg);
    }
}

pub trait LogHandler {
    fn output(&mut self, msg: &String);
}

pub struct FileLogHandler {
    buffer: Vec<String>,
}

impl LogHandler for FileLogHandler {
    fn output(&mut self, msg: &String) {
        self.buffer.push(msg.clone());
    }
}

pub struct StderrLogHandler {
    buffer: Vec<String>,
    // needs stdout fd and something implementing io::Write
}

impl LogHandler for StderrLogHandler {
    fn output(&mut self, msg: &String) {
        self.buffer.push(msg.clone());
    }
}
