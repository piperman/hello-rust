// looping using loop statement
// display 1 through 4 in the terminal

fn main() {
    let mut i = 1;
    loop {
        println!("{:?}", i);
        if i == 4 {
            break;
        }
        i = i + 1;
    }
}