#![allow(unused)] // this would allow us declare unused variables **NOT RECOMMENDED

use std::io; // library for standard input/output
             // use std::io::* // to bring in all public library in std::io into this programs scope
use rand::Rng; // for generating random number
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add; // allow us to perform addition with our generics
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    const PI: f32 = 3.141592;

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
    // enum Day{
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday,
    // };

    // // Defining Methods for an Enum type
    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Day::Saturday | Day::Sunday => true,
    //             _ => false
    //         }
    //     }
    // }

    // let today: Day = Day::Monday;
    // match today {
    //     Day::Monday => println!("Everyone hates Monday"),
    //     Day::Tuesday => println!("Donut day"),
    //     Day::Wednesday => println!("Hump day"),
    //     Day::Thursday => println!("Pay day"),
    //     Day::Friday => println!("Almost Weekend"),
    //     Day::Saturday => println!("Weekend"),
    //     Day::Sunday => println!("Weekend"),
    // };

    // println!("Is today the weekend {}", today.is_weekend());

    /****** Vectors  *********/
    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2 = vec![1, 2, 3, 4];
    // vec2.push(5);
    // let mut vec3: Vec<i32> = Vec::new();
    // vec3 = [1, 3, 4, 5, 7, 8].to_vec();
    // println!("1st: {}", vec2[0]);
    // let second: &i32 = &vec2[1];
    // match vec2.get(1){
    //     Some(second) => println!("2nd: {}", second),
    //     None => println!("No 2nd value"),
    // }
    // for i in &mut vec2 {
    //     *i *= 2;
    // }
    // for i in &vec2 {
    //     println!("{}", i);
    // }
    // println!("Vec Length {}", vec2.len());
    // println!("Pop: {:?}", vec2.pop());

    // get_sum(5, 4);
    // println!("{}", get_sum_2(5,4));
    // println!("{}", get_sum_3(5,4));

    // let (val_1, val_2) = get_2(3);
    // println!("Nums: {} {}", val_1, val_2);

    // let num_list = vec![1, 2, 3, 4, 5];

    // println!("Sum of list = {}", sum_list(&num_list));

    // println!("5 + 4 = {}", get_sum_gen(5, 4));
    // println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));

    /******* Ownership ********/
    // let str1 = String::from("world");
    //
    // This would only occur with str, array, vect any derived type but not
    // with int, float
    // let str2 = str1;
    // println!("Hello {}", str1); // Error
    // let str2 = str1.clone();
    // println!("Hello {}", str1); // No Error

    // let str_1 = str1.clone();
    // print_str(str1);
    // let str3 = print_return_str(str_1);
    // println!("str3 = {}", str3);

    // let mut str_3 = String::from("Omotoye");
    // change_string(&mut str_3);

    // let mut heroes = HashMap::new();
    // heroes.insert("Superman", "Clark kent");
    // heroes.insert("Batman", "Bruce Wayne");
    // heroes.insert("The Flash", "Barry Allen");

    // for (k, v) in heroes.iter() {
    //     println!("{} = {}", k, v);
    // }

    // println!("Length of Dictionary: {}", heroes.len());

    // if heroes.contains_key(&"Batman") {
    //     let the_batman = heroes.get(&"Batman");
    //     match the_batman {
    //         Some(x) => println!("Batman is a hero"),
    //         None => println!("Batman is not a hero"),
    //     }
    // }

    /******* Struct ********* */
    // struct Customer {
    //     name: String,
    //     address: String,
    //     balance: f32,
    // }

    // let mut bob = Customer{
    //     name: String::from("Bob Smith"),
    //     address: String::from("555 Main St"),
    //     balance: 234.50
    // };
    // bob.address = String::from("505 Main St");

    // struct Rectangle {
    //     length: T,
    //     height: U,
    // }
    // let rec = Rectangle{length: 4, height: 10.5};

    /****** Trait ******* */
    // trait Shape {
    //     fn new(length: f32, width: f32) -> Self;
    //     fn area(&self) -> f32;
    // }

    // struct Circle {length: f32, width: f32};
    // struct Rectangle{length: f32, width: f32};

    // // implementing the Shape trait for Rectangle and Cirle
    // impl Shape for Rectangle {
    //     fn new(length: f32, width: f32) -> Rectangle {
    //         return Rectangle{length, width};
    //     }
    //     fn area(&self) -> f32 {
    //         return self.length * self.width;
    //     }
    // }

    // impl Shape for Circle {
    //     fn new(length: f32, width: f32) -> Circle {
    //         return Circle{length, width};
    //     }
    //     fn area(&self) -> f32 {
    //         return (self.length / 2.0).powf(2.0) * PI;
    //     }
    // }

    // let rec: Rectangle = Shape::new( 10.0, 10.0);
    // let circ: Circle= Shape::new( 10.0,  10.0);

    // println!("Rec Area: {}", rec.area());
    // println!("Circ Area: {}", circ.area());

    /***** Calling function from Modules *******/
    // order_food();

    /******** Error Handling ******* */
    // panic!("A Terrible Error has just happened!");
    //
    // let lil_arr: [i32, 2] = [1, 2];
    // println!("{}", lil_arr[10]); // Error
    // let path = "lines.txt";
    // let output = File::create(path);
    // let mut output = match output {
    // Ok(file) => file,
    // Err(error) => {
    // panic!("Problem creating file: {:?}", error);
    // }
    // };

    // write!(output, "Just some\nRandom words").expect("Failed to write to file");

    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // for line in buffered.lines() {
    // println!("{}", line.unwrap());
    // }

    // let output2 = File::create("rand.txt");
    // let output2 = match output2 {
    // Ok(file) => file,
    // Err(error) => match error.kind() {
    // ErrorKind::NotFound => match File::create("rand.txt") {
    // Ok(fc) => fc,
    // Err(e) => panic!("Can't create file: {:?}", error),
    // },
    // _other_error => panic!("Problem opening file: {:?}", error),
    // },
    // };

    /********Restart from Iterators..... */

    /******* Iterators ******** */
    // let mut arr_it = [1, 2, 3, 4];
    // for val in arr_it.iter() {
    // println!("{}", val);
    // }
    // let mut iter1 = arr_it.iter();
    // println!("1st: {:?}", iter1.next());

    /************ Closures ****************** */
    // a closure is a function without a name

    // How to create closure
    // let var_name = |parameters| -> return_type
    // {BODY}

    // let can_vote = |age: i32| {
    // age >= 18
    // };
    // println!("Can vote: {}", can_vote(8));
    // closure can assess variables outside of its body
    // let mut samp1 = 5;
    // let print_var = || println!("samp1 = {}", samp1);
    // print_var();
    // samp1 = 10;
    // let mut change_var = || samp1 += 1;
    // change_var();
    // println!("samp1 = {}", samp1);
    // samp1 = 10;
    // println!("samp1 = {}", samp1);

    // fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
    // func(a, b)
    // }
    // let sum = |a, b| a + b;
    // let prod = |a, b| a * b;
    // println!("5 + 4 = {}", use_func(5, 4, sum));
    // println!("5 * 4 = {}", use_func(5, 4, prod));

    /****** Smart Pointers **********/
    // let b_int1 = Box::new(10);
    // println!("b_int1 = {}", b_int1);

    // struct TreeNode<T> {
    // pub left: Option<Box<TreeNode<T>>>,
    // pub right: Option<Box<TreeNode<T>>>,
    // pub key: T,
    // }

    // impl<T> TreeNode<T> {
    // pub fn new(key: T) -> Self {
    // TreeNode {
    // left: None,
    // right: None,
    // key,
    // }
    // }
    // pub fn left(mut self, node: TreeNode<T>) -> Self {
    // self.left = Some(Box::new(node));
    // self
    // }
    // pub fn right(mut self, node: TreeNode<T>) -> Self {
    // self.right = Some(Box::new(node));
    // self
    // }
    // }

    // let node1 = TreeNode::new(1)
    // .left(TreeNode::new(2))
    // .right(TreeNode::new(3));

    /************ Concurrency ********** */
    // command problems with parallel programming
    // 1. threads are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution
    // let thread1 = thread::spawn(|| {
    // for i in 1..25 {
    // println!("Spawned thread: {}", i);
    // thread::sleep(Duration::from_millis(1));
    // }
    // });

    // for i in 1..20 {
    // println!("Main thread: {}", i);
    // thread::sleep(Duration::from_millis(1));
    // }
    // thread1.join().unwrap();

    // pub struct Bank {
    // balance: f32,
    // }

    // fn withdraw(the_bank: Arc<Mutex<Bank>>, amt: f32){
    // let mut bank_ref = the_bank.lock().unwrap();
    // if bank_ref.balance < 5.00 {
    // println!("Current Balance: {} Withdrawal a smaller amount", bank_ref.balance);
    // } else {
    // bank_ref.balance -= amt;
    // println!("Customer withdrew {} Current Balance {}", amt, bank_ref.balance);
    // }
    //
    // }

    // fn customer(the_bank: Arc<Mutex<Bank>>) {
    // withdraw(&the_bank, 5.00);
    // }
    // let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));
    // let handles = (0..10).map(|_| {
    // let bank_ref = bank.clone();
    // thread::spawn(|| {
    // customer(bank_ref)
    // })
    // });
    // for handle in handles {
    // handle.join().unwrap();
    // }
    // println!("Total {}", bank.lock().unwrap().balance);
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

/************ Generics  **********/
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

/*********** Functions **************/
fn say_hello() {
    println!("Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}
