use std::io;

fn main() {
    println!("Enter you weight in (kg)..");

    let mut input = String::new();
    io::stdin().read_line(& mut input).unwrap();
    let weight = input.trim().parse();
    calculate_weights(weight);
    println!("{}kg", input.trim()); 
}

fn calculate_weights(weight: f32) -> f32 {
println!("{}kg", weight)
}