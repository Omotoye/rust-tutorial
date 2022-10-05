#![allow(unused)] // this would allow us declare unused variables **NOT RECOMMENDED

use std::io; // library for standard input/output
             // use std::io::* // to bring in all public library in std::io into this programs scope
use rand::Rng; // for generating random number
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // println!("What is your name?"); // println!() is a macro because of the ! and not a function

    // variables are immutable by default; to make them mutable, a mut keyword has to come before
    // the variable name.
    // let mut name: String = String::new(); // String::new() is a function that's going to return an empty string
    // let greeting: &str = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't Receive Input");
    // println!("Hello, {}! {}", name.trim_end(), greeting)

    /* Variables */
    // const ONE_MIL: u32 = 1_000_000; // constant are written in uppercase letters
    // const PI: f32 = 3.141592;
    // let age: &str = "47";
    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    /****** Data Types *********/

    /* Integer Data Types */

    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize
    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max u128 : {}", f64::MAX);

    /* Boolean type */
    // let is_true: bool = true;
    // let is_false: bool = false;

    /* Character */
    // let my_grade: char = 'A';

    /******** Math  *********/
    // let num_1: f32 = 1.11111111111111111;
    // println!("f32 : {}", num_1 + 0.1111111111111111111);
    // let num_2: f64 = 1.11111111111111111;
    // println!("f32 : {}", num_2 + 0.1111111111111111111);

    // let mut num_3: u32 = 5;
    // let num_4: u32 = 4;
    // println!("5 + 4 = {}", num_3 + num_4);
    // println!("5 - 4 = {}", num_3 - num_4);
    // println!("5 * 4 = {}", num_3 * num_4);
    // println!("5 / 4 = {}", num_3 / num_4);
    // println!("5 % 4 = {}", num_3 % num_4);
    // num_3 += 1;

    /* Generating Random Values */
    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random : {}", random_num);

    /**** start from if expressions. */
    // let age = 8;
    // if (age >= 1) && (age <= 18) {
    //     println!("Important Birthday");
    // } else if (age == 21) || (age == 50) {
    //     println!("Important Birthday level 2");
    // } else if age >= 65 {
    //     println!("Important Birthday level 3");
    // } else {
    //     println!("Not an Important Birthday")
    // }

    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 { true } else { false };
    // println!("Can Vote : {}", can_vote);

    /* Match Conditional */
    // let age2 = 20;
    // match age2 {
    //     // if age2 match num in range 1 - 19
    //     1..=18 => println!("Important Birthday"),

    //     // if age2 match 21 or 50
    //     21 | 50 => println!("Important Birthday level 2"), 

    //     // if age2 match num in range 65 - MAX of i32
    //     65..=i32::MAX => println!("Important Birthday level 3"),

    //     // else
    //     _ => println!("Not an important Birthday"),

    // };

    // println!("Just Checking");

    // let my_age: i8 = 18;
    // let voting_age: i8 = 18; 

    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Can't vote"), 
    //     Ordering::Greater => println!("Can Vote"), 
    //     Ordering::Equal => println!("You gained the right to vote"),
    // };

    // println!("{}", voting_age);

    /*** Arrays ****/
//     let arr_1 = [1, 2, 3, 4,];
//     println!("1st: {}", arr_1[0]);
//     println!("Length: {}", arr_1.len());

//     // looping through arrays 
//     let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let mut loop_idx = 0;

//     // using loop
//     loop {
//         if arr_2[loop_idx] % 2 == 0 {
//             loop_idx += 1;
//             continue;
//         }
//          if arr_2[loop_idx]  == 9 {
//            break;
//         }
//         println!("Val: {}", arr_2[loop_idx]);
//         loop_idx  += 1;
        
//     }
//    // using while loops 
//    let mut loop_idx = 0;

//    println!();
//    while loop_idx < arr_2.len() {
//     println!("Arr: {}", arr_2[loop_idx]);
//     loop_idx += 1;
//    } 

//    // using for loops 
//    println!();
//    for val in arr_2.iter() {
//     println!("Val: {}", val);
//    }

   /***** Tuples *******/
//    let my_tuple: (u8, String, f64) = (47, "Omotoye".to_string(), 50_000.00);

//    println!("Name: {}", my_tuple.1);

//    // tuple unpacking 
//    let (v1, v2, v3) = my_tuple;
//    println!("Age: {}", v1);

    /********* Strings *********/
    // let mut st1: String = String::new();
    // st1.push('A');
    // st1.push_str(" word");
    
    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }
    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    // let st3 = String::from("x r t b k k a m c");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }

    // // Creating a string by getting a reference to a string
    // let st4: &str = "Random String";
    // let mut st5: String = st4.to_string();
    // println!("{}", st5);
    // let byte_arr1 = st5.as_bytes();

    // let st6 = &st5[3..9];
    // println!("{}", st6);
    // println!("String length: {}", st6.len());
    // st5.clear();

    // // combining strings 
    // let st6: String = String::from("Just some");
    // let st7: String = String::from(" words");
    // let st8: String = st6 + &st7;
    // for char in st8.bytes() {
    //     println!("{}", char);
    // }

   /********* Casting ********/
//    let int_u8: u8 = 5;
//    let int2_u8: u8 = 4; 
//    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    /******** Enums *************/
    enum Day {
        Monday, 
        Tuesday,
        Wednesday,
        Thursday, 
        Friday,
        Saturday,
        Sunday,
    };

    // Defining Methods for an Enum type
    impl Day {
        fn is_weekend($self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true, 
                _ => false
            };
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"), 
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    };

    println!("Is today the weekend {}", today.is_weekend());
}
