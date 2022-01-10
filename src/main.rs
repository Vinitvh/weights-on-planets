use std::io;

fn main() {
    println!("Enter you weight in (kg)..");

    let mut input = String::new();
    io::stdin().read_line(& mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let (mercury, venus, earth, mars, jupiter, saturn, uranus, neptune) = calculate_weights(weight);
    // Round off to two decimal places
    println!("Your weight is {:.2}kg on Mercury!", mercury); 
    println!("Your weight is {:.2}kg on Venus!", venus); 
    println!("Your weight is {:.2}kg on Earth!", earth); 
    println!("Your weight is {:.2}kg on Mars!", mars); 
    println!("Your weight is {:.2}kg on Jupiter!", jupiter); 
    println!("Your weight is {:.2}kg on Saturn!", saturn); 
    println!("Your weight is {:.2}kg on Uranus!", uranus); 
    println!("Your weight is {:.2}kg on Neptune!", neptune); 
}

fn calculate_weights(weight: f32) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let mercury = (weight) * 0.38;
    let venus = (weight) * 0.91;
    let earth = (weight) * 1.0;
    let mars = (weight) * 0.3711;
    let jupiter = (weight) * 2.34;
    let saturn = (weight) * 1.06;
    let uranus = (weight) * 0.92;
    let neptune = (weight) * 1.19;

 (mercury, venus, earth, mars, jupiter, saturn, uranus, neptune)
}    
