// https://www.hackerrank.com/challenges/migratory-birds/problem

use std::collections::HashMap; // include HashMap header

#[allow(non_snake_case)]
pub fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new(); // HashMap<bird_id, count>
    
    // check the all birds in the array
    for bird in arr {
        // check is the current bird new one
        if map.get(bird).is_none() {
            map.insert(*bird, 1); // add this bird
        }
        else {
            *map.get_mut(bird).unwrap() += 1; // grow up a count
        }
    }

    // find a lower bird_id of the most common
    let mut min_of_max: (i32, i32) = (0, 0); // pair (bird_id, count)
    for (bird, count) in map.iter() {
        // found a more common bird
        if min_of_max.1 < *count {
            min_of_max.0 = *bird;
            min_of_max.1 = *count;
            continue;
        }

        // the same common but with a lower id
        if min_of_max.0 > *bird && min_of_max.1 == *count {
            min_of_max.0 = *bird;
        }
    }

    // return a bird type
    return min_of_max.0;
}

#[test]
fn test_migration_birds() {
    assert_eq!(migratoryBirds(&[1, 4, 4, 4, 5, 3]), 4);
    assert_eq!(migratoryBirds(&[7, 3, 2, 5, 6, 6, 5]), 5);
    assert_eq!(migratoryBirds(&[7, 7, 8, 8, 6, 6]), 6);
    assert_eq!(migratoryBirds(&[1]), 1);
}