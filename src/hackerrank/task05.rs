// https://www.hackerrank.com/challenges/apple-and-orange/problem

#[allow(non_snake_case)]
pub fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> Option<(i32, i32)> {
    let mut apples_in_area: i32 = 0;
    let mut oranges_in_area: i32 = 0;

    // find all apples in area
    for k in apples {
        let fall_pos = a + k;
        if (s..=t).contains(&fall_pos) {
            apples_in_area += 1;
        }
    }

    // find all oranges in area
    for k in oranges {
        let fall_pos = b + k;
        if (s..=t).contains(&fall_pos) {
            oranges_in_area += 1;
        }
    }

    // result
    println!("{}", apples_in_area);
    println!("{}", oranges_in_area);
    
    return Some((apples_in_area, oranges_in_area)); // need only for unit tests
}

#[test]
fn test_apples_and_oranges() {
    let s: i32 = 7;
    let t: i32 = 11;
    let a: i32 = 5;
    let b: i32 =  15;
    let apples: [i32; 3] = [-2, 2, 1];
    let oranges: [i32; 2] = [5, -6];
    assert_eq!(
        countApplesAndOranges(s, t, a, b, &apples, &oranges),
        Some((1, 1))
    )
}