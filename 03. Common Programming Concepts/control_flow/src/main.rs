fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // the two possible result must be of thesame type
    // let number = if condition { 5 } else { "six" }; // this would generate and error

    println!("The value of number is: {number}");
}
