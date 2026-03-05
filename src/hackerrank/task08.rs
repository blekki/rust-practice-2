// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

#[allow(non_snake_case)]
pub fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut lower: i32 = scores[0];
    let mut highter: i32 = scores[0];
    
    let mut h_count = 0;
    let mut l_count = 0;

    for rate in scores {
        if *rate < lower { // find the worst records
            lower = *rate;
            l_count += 1;
            continue;
        }

        if *rate > highter { // find the best records
            highter = *rate;
            h_count += 1;
        }
    }

    return vec![h_count, l_count];
}

#[test]
fn test__() {
    assert_eq!(breakingRecords(&[10, 5, 20, 20, 4, 5, 2, 25, 1]), vec![2, 4]);
    assert_eq!(breakingRecords(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42]), vec![4, 0]);
    assert_eq!(breakingRecords(&[10, 9, 8, 7, 6, 5]), vec![0, 5]);
}