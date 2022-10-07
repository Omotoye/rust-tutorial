use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input: String = String::new();

    // unwrap() would terminate the program if the result is an Error
    // if Success, it would yield the content of the result.
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32  = input.trim().parse().unwrap(); // remove the white spaces from the string
    // parse would parse the string to the type that the variable on the lhs is expecting.

    println!("Input: {}", input);
    let mars_weight: f32 = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
