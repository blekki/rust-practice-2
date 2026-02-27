mod hackerrank;

// have used to keep the func's names without a prefix "_".
fn solve_func_is_never_used() {
    if false {
        hackerrank::task0::simple_array_sum(&[]);
        hackerrank::task03::staircase(5);
        hackerrank::task04::gradingStudents(&[]);
        hackerrank::task05::countApplesAndOranges(0, 0, 0, 0, &[], &[]);
    }
}

fn main() {
    solve_func_is_never_used(); // solve the same named warnings
    println!("We learn Rust!");
}
