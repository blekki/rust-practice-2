// https://www.hackerrank.com/challenges/between-two-sets/problem

fn lcm(array: &[i32]) -> i32 {
    // find a highest value to use it as a checking step
    let mut highest: i32 = array[0];
    for a in array {
        if highest < *a { highest = *a; }
    }

    let mut factor: i32 = 0; // future lcm
    let mut found: bool = false;
    while found == false {
        factor += highest;
        for a in array {
            found = true;
            if factor % *a != 0 {
                found = false;
                break;
            }
        }
    }
    return factor;
}

fn is_common_divisor(divisor: i32, array: &[i32]) -> bool {
    let mut result: bool = true;
    for num in array {
        let condition1: bool = *num < divisor;
        let condition2: bool = *num % divisor != 0;
        
        if condition1 || condition2 {
            result = false;
            break;
        }
    }
    return result;
}

#[allow(non_snake_case)]
pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    // check is that possible find a lcm-gcd to the current task
    let mut highest_a = a[0];
    let mut lowest_b  = b[0];

    for num in a {
        if highest_a < *num { highest_a = *num; }
    }

    for num in b {
        if lowest_b > *num { lowest_b = *num; }
    }

    if highest_a > lowest_b { return 0; }

    // find results count
    let mut count: i32 = 0;
    let lcm = lcm(a);
    if lowest_b < lcm { return 0; }
    
    let mut divisor = lcm;
    while divisor <= lowest_b {
        if is_common_divisor(divisor, b) {
            count += 1;
        }
        divisor += lcm;
    }
    
    return count;
}


#[test]
fn test_lcm() {
    assert_eq!(lcm(&[1, 2, 4, 12]), 12);
    assert_eq!(lcm(&[3, 7]),        21);
    assert_eq!(lcm(&[2, 3, 8, 9]),  72);
    assert_eq!(lcm(&[1]),            1);
    assert_eq!(lcm(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 2520);
}

#[test]
fn test_common_divisor() {
    assert_eq!(
        is_common_divisor(10, &[20, 30, 40, 50]),
        true
    );
    assert_eq!(
        is_common_divisor(10, &[20, 31]),
        false
    );
    assert_eq!(
        is_common_divisor(21, &[21, 6]),
        false
    );
}

#[test]
fn test_total_x() {
    assert_eq!(
        getTotalX(&[2, 4], &[16, 32, 96]),
        3
    );
    assert_eq!(
        getTotalX(&[100, 99, 98, 97, 96, 95, 94, 93, 92, 91], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        0
    );
}