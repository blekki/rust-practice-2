mod hackerrank;

// have used to keep the func's names without a prefix "_".
fn solve_func_is_never_used() {
    if false {
        hackerrank::task0::simple_array_sum(&[]);
    }
}

fn main() {
    println!("We learn Rust!");
    
    solve_func_is_never_used(); // solve the same named warning
}
