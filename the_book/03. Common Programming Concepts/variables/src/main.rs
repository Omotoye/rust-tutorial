fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    let spaces = "    ";
    let spaces = spaces.len();

    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("The length of spaces: {spaces}");

    }
