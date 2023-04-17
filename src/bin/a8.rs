// using struct
// requirements: Print the flavor of a drink and it's fluid ounces

// use an enum to create different flavors of drinks
// use a struct to store flavor and fluid ounce of drink
// use a function to print out the flavor and ounces
// use match to print the drink flavor

enum Flavor {
    Orange,
    Cola,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f32,
}

fn main() {
    let coke = Drink {
        flavor: Flavor::Cola,
        fluid_oz: 18.6,
    };
    let crush: Drink = Drink {
        flavor: Flavor::Orange,
        fluid_oz: 10.2,
    };
    print_drink(coke);
    print_drink(crush);
}

fn print_drink(my_drink: Drink) {
    let mut fl;
    match my_drink.flavor {
        Flavor::Cola => fl = "cola",
        Flavor::Orange => fl = "orange",
    }
    println!("flavor: {:?}", fl);
    println!("oz: {:?}", my_drink.fluid_oz);
}
