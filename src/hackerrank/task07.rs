// https://www.hackerrank.com/challenges/between-two-sets/problem

fn lcm(array: &[i32]) -> i128 {
    // find a highest value to use it as a checking step
    let mut highest_num: i32 = array[0];
    for a in array {
        if highest_num < *a {
            highest_num = *a;
        }
    }

    let mut factor: i128 = 0; // future lcm
    let mut found: bool = false;
    while found == false {
        factor += highest_num as i128;
        for a in array {
            found = true;
            if factor % (*a as i128) != 0 {
                found = false;
                break;
            }
        }
    }
    return factor;
}

fn is_common_divisor(divisor: i128, array: &[i32]) -> bool {
    let mut result: bool = true;
    for num in array {
        let condition1: bool = (*num as i128) < divisor;
        let condition2: bool = (*num as i128) % divisor != 0;
        
        if condition1 || condition2 {
            result = false;
            break;
        }
    }
    return result;
}

#[allow(non_snake_case)]
pub fn getTotalX(a: &[i32], b: &[i32]) -> i32 {

    let mut lowest_b: i128 = b[0] as i128;
    for num in b {
        if lowest_b > (*num as i128) {
            lowest_b = *num as i128;
        }
    }

    let mut highest_a: i128 = a[0] as i128;
    for num in a {
        if highest_a < (*num as i128) {
            highest_a = *num as i128;
        }
    }

    if highest_a > lowest_b { return 0; }



    let mut matches: i32 = 0;
    let lcm: i128 = lcm(a);
    let mut divisor: i128 = lcm;

    if lowest_b < lcm {
        return matches;
    }

    while divisor <= lowest_b {
        if is_common_divisor(divisor, b) == true {
            matches += 1;
        }
        divisor += lcm;
    }
    
    return matches;
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