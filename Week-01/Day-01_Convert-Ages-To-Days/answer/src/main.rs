use std::io;

fn main() {

    // Get current age
    let mut input = String::new();

    println!("What is your current age?");

    // Call input output standard library
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert String to u32 integer
    let age: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a positive number.");
            return;
        }
    };


    // Print current age
    println!("Your current age is: {}", age);

    // Convert age in years to age in days (sort of)
    let age_in_days: u32 = age * 365;
    println!("Your current age in days is: {}", age_in_days);
}
