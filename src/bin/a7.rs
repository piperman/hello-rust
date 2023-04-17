// Use enums
// requirement: Print the name of a color to the console.

// notes:
// use an enum with color names as variants
// use a function to print the color name
// the function must use the enum as a parameter
// use a match expression to determine which color name to print.

enum Colors {
    Green,
    Yellow,
    Orange,
    Red
}

fn main() {
    print_my_color(Colors::Yellow);
}

fn print_my_color(my_color: Colors) {
    match my_color {
        Colors::Green => println!("green"),
        Colors::Orange => println!("orange"),
        Colors::Red => println!("red"),
        Colors::Yellow => println!("yellow")
    }
}