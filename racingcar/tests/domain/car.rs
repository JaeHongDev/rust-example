#[cfg(test)]

mod car_test {

    use racingcar::domain::car::Car;


    #[test]
    #[should_panic]
    fn 자동차의_이름은_5글자_미만입니다() {
        (5..=10)
            .map(|len| "x".repeat(len))
            .for_each(|name| { Car::new(name); });
    }

    #[test]
    #[should_panic]
    fn 자동차의_이름은_최소_1글자_이상입니다(){
        ["", " ", " "]
            .iter()
            .for_each(|name| {
                Car::new(name.parse().unwrap());
            });
    }
}


#[test]
fn 자동차의_이름이_유효하지_않은_경우_오류가_발생합니다(){

}