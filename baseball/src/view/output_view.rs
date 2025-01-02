
pub trait Output {
    
    fn println(msg: &str);
}


pub struct ConsoleOutputView {}

impl Output for ConsoleOutputView { 

    fn println(msg: &str) {
        println!("{}", msg);
    }
    
}

pub fn from() -> impl Output {
    ConsoleOutputView {}
}



