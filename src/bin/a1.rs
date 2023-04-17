// Display first and last name
// Write a function to display first name
// Write a function to display last name
// Use println!() to output message to user

fn main() {
    let first_name = String::from("Steve");
    let last_name = "Naples";

    display_first_name(first_name);
    display_last_name(last_name);
    
}

fn display_first_name(first_name: String) {
    println!("{first_name}");
}

fn display_last_name(last_name: &str) {
    println!("{last_name}");
}