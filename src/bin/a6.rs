// topic: loop using while
// count down from 5 to 1, display countdown in terminal.

fn main() {
    let mut x = 5;
    while x >= 1 {
        println!("{:?}", x);
        x = x - 1;
    }
    println!("done!");
}