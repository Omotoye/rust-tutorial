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

    //**** start from if expressions. */
}
