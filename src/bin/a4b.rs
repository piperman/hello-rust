// decision making with match
// requirements: display "one", "two", "three" or "other" based on value
// of integer variable.
// notes:
// set a variable to an integer value
// use match to determine what message to display
// use _ to match all other cases

fn main() {
    let n = 9;

    match n {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
}