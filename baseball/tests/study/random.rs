use rand::prelude::SliceRandom;

#[test]
fn 숫자_야구_게임에서_랜덤한_자리_수_가져오기() {
    let mut nums: Vec<i64> = (1..=9).collect();

    nums.shuffle(&mut rand::rng());

    let n = rand::random_range(1..=9);

    std::iter::repeat(rand::random_range(1..=9));
    println!("{:?}", nums);

    let a: Vec<i64> = nums.iter().take(3).map(|&x| x).collect();
    println!("{:?}", a);
    println!("{:?}", n);
}
