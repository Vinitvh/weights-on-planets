use std::io;

fn main() {
    println!("Enter you weight in (kg)..");

    let mut input = String::new();
    io::stdin().read_line(& mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars = calculate_weights(weight);
    // Round off to two decimal places
    println!("Your weight is {:.2}kg on Mars!", mars); 
}

fn calculate_weights(weight: f32) -> f32 {
 (weight / 9.81) * 3.711
}   