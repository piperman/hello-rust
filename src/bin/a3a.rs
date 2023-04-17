// flow control using if..else

// display a message based on boolean variable
// when variable is true display "hello"
// when variable is false display "goodbye"

// notes:
// use a variable set to true or false
// use an if..else block to determine which message to display
// use the println macro to display messages to the terminal

fn main() {
    let greeting = false;

    if greeting == true {
        println!("hello");
    } else {
        println!("goodbye");
    }
}