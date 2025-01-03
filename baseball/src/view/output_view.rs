pub trait Output {
    fn println(&self, msg: &str);
}

pub struct ConsoleOutputView {}

impl ConsoleOutputView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Output for ConsoleOutputView {
    fn println(&self, msg: &str) {
        println!("{}", msg);
    }
}
