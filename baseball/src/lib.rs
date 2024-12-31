pub fn add(left: usize, right: usize) -> usize {
    left + right
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

}
