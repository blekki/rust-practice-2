// https://www.hackerrank.com/challenges/staircase/problem
pub fn staircase(n: i32) {
    let space: &str = " ";
    let stair: &str = "#";
    
    for i in 1..=n {
        print!("{}", space.repeat((n - i) as usize));
        print!("{}", stair.repeat(i as usize));
        println!();
    }
}

#[test]
fn test0() {
    assert_eq!(0, 0);
}