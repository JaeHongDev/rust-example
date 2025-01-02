use std::io::{stdin, stdout, Write};

pub trait Input {
    fn read_number(option: Option<&str>) -> i64;
    fn read_line(option: Option<&str>) -> String;
}

pub struct ConsoleInputView;

impl Input for ConsoleInputView {

    fn read_number(option: Option<&str>) -> i64 {

        print(option);
        read_line().parse().expect("숫자 이외의 값이 입력됐습니다.")
    }

    fn read_line(option: Option<&str>) -> String {
        print(option);
        read_line()
    }
}

fn read_line() -> String {
    stdin().lines().next().unwrap().unwrap()
}

fn print(option: Option<&str>) {
    match option {
        None => {}
        Some(msg) => {
            print!("{}", msg);
            stdout().flush().unwrap();
        }
    }
}
