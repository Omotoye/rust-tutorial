use std::io;

fn main() {
    // /* Numerical Operations */
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");

    let floored = 2 / 3; // Results in 0
    println!("2 / 3 = {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    /* The Boolean type */

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t is {t} and f is {f}");

    /* The Character type */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /* Compound types: tuples and array */

    /* Tuple */

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // using patter matching to destrcture a tuple
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // we can also access a tuple element by using . followed by the
    // index of the value we want to access

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    /* Array */

    // every element in an array must be of thesame type
    // array are fixed in size
    // array should be used when you know the number of elements
    // will not need to change

    // Declaring an array 

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // you can specify the type and length of the array 
    let a: [i32; 5] = [1, 2, 3, 4, 5]; 

    // you can initialize an array that contains the same value 
    // for each element by specifying the initial value and then 
    // the length of the array 
    let a = [3; 5]; // this is thesame as let a = [3, 3, 3, 3, 3];

    // accessing and array element using indexing 
    let a = [1, 2, 3, 4, 5];

    let first = a[0]; 
    let second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
