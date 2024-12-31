use std::ops::Not;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn str_to_baseball_num(input: &str) -> Vec<u32> {
    let vec: Vec<u32> = str::chars(input).map(|c| c.to_digit(10))
        .map(|option_c| option_c.expect("문자열이 포함됐습니다"))
        .filter(|n| (1..=9).contains(n))
        .collect();

    vec.len()
        .eq(&3)
        .not()
        .then(|| {panic!("3자리가 아닙니다")});

    vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 학습_테스트1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn 개별_문자열을_숫자로_바꾸기(){
        let msg = "a23";

        let vec: Vec<u32>=  str::chars(msg)
            .map(|c| c.to_digit(10))
            .map(|option_c| option_c.expect("에러"))
            .collect();


        println!("{:?}", vec);
    }

    #[test]
    #[should_panic(expected = "3자리가 아닙니다")]
    fn 문자열은_3자리이며_1에서_9사이의_숫자임(){
        let input = "2124";

        str_to_baseball_num(input);
    }
}
