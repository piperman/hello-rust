// topic: program flow using if..else if..else

// display ">5", "<5" or "=5" based on value of variable
// notes:
// use an integer variable set to any integer value
// use an if..else if..else block to determine which message to display
// use println macro to display message to terminal

fn main() {
    let my_int: i32 = 43;

    if my_int > 5 {
        println!(">5");
    } else if my_int < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}