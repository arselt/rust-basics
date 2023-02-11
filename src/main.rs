fn main() {
    let age: u8 = 20;
    let name: &str = "Arath";
    let mut year: u16 = 2020;
    
    year = year + 3;

    println!("Hello I'm {} and i'm learning Rust in my {}s! on {}", name, age, year);
    
    let max_temp: u8 = 27; // unsigned type, negative numbers not assignable
    let min_temp: i8 = -10; // rust uses snake case for declaration
    
    println!("In my city the max temp was {max_temp}c and the min for today {min_temp}c");

    let mut others_name: String = String::new();
    let mut others_age: String = String::new();

    println!("...");
    println!("...");
    println!("What's your name?");
    
    std::io::stdin().read_line(&mut others_name).unwrap();
    
    println!("...");
    println!("What's your age?");

    std::io::stdin().read_line(&mut others_age).unwrap();

    let o_age_int : u8 = others_age.trim().parse().unwrap(); // parse, to number
    let o_name_trim : &str = others_name.trim(); // trim, removes whitespace

    println!("Welcome, {o_name_trim}... you are {o_age_int}");
}
