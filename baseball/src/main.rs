use std::io::{stdin, stdout, Write};

fn main() {


    println!("숫자 야구 게임을 시작합니다.");

    let input = read_line("숫지를 입력해주세요 : ");
    println!("{}", input);


    let words: Vec<i64>= input.chars()
        .map(|c| c as i64)
        .collect();


    println!("{:?}", words);



}


fn read_line(msg: &str) -> String {

    print!("{}", msg);

    stdout()
        .flush()
        .unwrap();

    stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
}

