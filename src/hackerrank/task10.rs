// https://www.hackerrank.com/challenges/sock-merchant/problem

use std::collections::HashMap; // include HashMap header

#[allow(non_snake_case)]
pub fn sockMerchant(_n: i32, ar: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();    // HashMap<socks_by_colors, count>

    // sort the socks by colors
    for sock in ar {
        if map.get(sock).is_none() {
            map.insert(*sock, 1); // add the sock
        }
        else {
            *map.get_mut(sock).unwrap() += 1; // increment count
        }
    }
    
    // get the all socks pairs
    let mut pair_count = 0;
    for (_color, count) in map.iter() {
        pair_count += (count / 2) as i32;
    }
    
    return pair_count;
}

#[test]
fn test_socks_pairs() {
    assert_eq!(sockMerchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
    assert_eq!(sockMerchant(5, &[1, 1, 1, 1, 1]), 2);
    assert_eq!(sockMerchant(7, &[-8, -8, 4, 2, 3, 4, -8]), 2);
    assert_eq!(sockMerchant(2, &[9999999, 9999999]), 1);
    assert_eq!(sockMerchant(1, &[0]), 0);
}