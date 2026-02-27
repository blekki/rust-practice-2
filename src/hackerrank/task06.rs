// https://www.hackerrank.com/challenges/kangaroo/problem

const NEGATIVE_RESULT: &str = "NO";
const POSITIVE_RESULT: &str = "YES";

#[allow(non_snake_case)]
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if x1 == x2 { return String::from(POSITIVE_RESULT); }
    if x1 > x2  { return String::from(NEGATIVE_RESULT); }                // kangoroo_№1 has overtook kangoroo_№2
    if x1 < x2 && v1 < v2 { return String::from(NEGATIVE_RESULT); }     // kangoroo_№1 has never catches up kangoroo_№2

    return kangaroo(x1 + v1, v1, x2 + v2, v2);
}

#[test]
fn test_kangaroo_positive() {
    assert_eq!(
        kangaroo(0, 3, 4, 2),
        String::from(POSITIVE_RESULT)
    );
}

#[test]
fn test_kangaroo_negative() {
    assert_eq!(
        kangaroo(0, 2, 5, 3),
        String::from(NEGATIVE_RESULT)
    );
}