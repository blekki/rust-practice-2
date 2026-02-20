// https://www.hackerrank.com/challenges/grading/problem

#[allow(non_snake_case)]
pub fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    const LOWER_LIMIT: i32 = 38;
    const DIFFERENCE_LIMIT: i32 = 3;

    let mut new_grades: Vec<i32> = Vec::new();
    
    // check is grades list empty
    if grades.len() == 0 { return new_grades; }

    // other logic
    let students_count: i32 = grades[0];
    for a in 1..=students_count {
        let rate = grades[a as usize];

        // if less then limit --> student hasn't possibility change grade
        if rate < LOWER_LIMIT {
            new_grades.push(rate);
            continue;
        }

        // round condition
        let difference: i32 = rate % 5;
        if difference >= DIFFERENCE_LIMIT {
            new_grades.push(rate - difference + 5);
            continue;
        }

        // default
        new_grades.push(rate);
    }

    return new_grades;
}

#[test]
fn test_empty_containers() {
        assert_eq!(gradingStudents(&[]), vec![]);
        assert_eq!(gradingStudents(&[0]), vec![]);
}

#[test]
fn test_full_containers() {
    {
        let grades: [i32; 8] = [7, 43, 10, 88, 51, 90, 37, 100];
        let expect: Vec<i32> = vec![45, 10, 90, 51, 90, 37, 100];
        assert_eq!(gradingStudents(&grades), expect);
    }

    {
        let grades: [i32; 5] = [4, 73, 67, 38, 33];
        let expected: Vec<i32> = vec![75, 67, 40, 33];
        assert_eq!(gradingStudents(&grades), expected);
    }

    {
        let grades: [i32; 6] = [5, 55, 56, 57, 58, 59];
        let expected: Vec<i32> = vec![55, 56, 57, 60, 60];
        assert_eq!(gradingStudents(&grades), expected);
    }
}