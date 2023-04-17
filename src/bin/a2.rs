// basic math
// display the result of the sum of two numbers

// use function to sum two numbers
// use function to display the sum
// use {:?} in println macro

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_sum(result:i32) {
    println!("The sum is {:?}", result);
}

fn main() {
    let result = sum(5, 6);
    display_sum(result);
}