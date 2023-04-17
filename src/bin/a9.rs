// Topic: Data management using tuples
//
// Requirements: Print whether the cartesian y-value of a coordinate is
// < 5, > 5, = 5;
//
// Use a function that returns a tuple
// destructure the return value into two variables
// Use an if..else if..else block to determine what to print.

fn main() {
    let (_x, y) = coord();
    if y < 5 {
        println!("{:?} < 5", y);
    } else if y > 5 {
        println!("{:?} > 5", y);
    } else {
        println!("{:?} = 5", y);
    }
}

fn coord() -> (i32, i32) {
    (2, 3)
}