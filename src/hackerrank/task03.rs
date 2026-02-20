// https://www.hackerrank.com/challenges/staircase/problem
pub fn staircase(n: i32) -> String{
    let space_symbol: &str = " ";
    let stair_symbol: &str = "#";
    
    let mut result: String = String::from("");
    for i in 1..=n {
        result += &space_symbol.repeat((n - i) as usize);
        result += &stair_symbol.repeat(i as usize);
        result += &"\n";
    }

    print!("{}", result);           // need to pass the hackerrank unit_tests
    return String::from(result);    // need to pass the own unit_tests
}

#[test]
fn test0() {
    assert_eq!(
        staircase(0), 
        ""
    );

    assert_eq!(
        staircase(5),
        "    #\n".to_owned() +
        "   ##\n" +
        "  ###\n" +
        " ####\n" +
        "#####\n"
    );

    assert_eq!(
        staircase(2),
        " #\n".to_owned() +
        "##\n"
    );
}