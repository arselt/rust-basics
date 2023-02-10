fn main() {
    let age: u8 = 20;
    let name: &str = "Arath";
    let mut year: u16 = 2020;
    
    year = year + 3;

    println!("Hello I'm {} and i'm learning Rust in my {}s! on {}", name, age, year);
}
