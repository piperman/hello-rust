// topic: decision making with match
// requirements: display "it's true" or "it's false" based on value of 
// boolean variable
// notes:
// use a variable set to true or false
// use match to determine which message to display

fn main() {
    let var = false;
    match var {
        true => println!("it's true"),
        false => println!("it's false")
    }
}