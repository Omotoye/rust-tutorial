// use std::collections::HashMap;
// use std::fs::File;
// use std::io::ErrorKind;
// use std::iter::Sum;

// // Write a function that takes a string and returns the first word
// // it finds in the string
// fn first_word_not_optimal(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     fn add(a: u32, b: u32) -> u32 {
//         a + b
//     }
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// // Defining Enums
// enum IpAddrKind {
//     V4,
//     V6,
// }

// // The data in the struct can be representated inside the enum in a more concise way
// enum IpAddrKind2 {
//     V4(String),
//     V6(String),
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    // /***** VARIABLES ****** */
    // let x = 5;
    // const MAX_POINT: u32 = 100_000;
    // println!("The value of x is: {}", x);
    // // x = 6; // Compiler Error: x is an immuatable variable
    // let x = 6; // this owrks because of shadowing

    // // when shadowing we can change the type of the variable
    // let mut spaces = " ";
    // // spaces = spaces.len(); // compiler error spaces.len() is of int type
    // let spaces = spaces.len();

    // // mut cannot be used for the previous example.
    // let mut new_spaces = " ";
    // // new_spaces = new_spaces.len(); // Compiler error, since they are not of thesame type

    // /******** DATA TYPE ****************` */
    // // TUPLE
    // let tup: (i32, f32, u8) = (500, 6.4, 1);

    // // tuple unpacking with pattern matching
    // let (x, y, z) = tup;
    // println!("The value of y is: {}", y);

    // // accesing tuple with .index
    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let one = tup.2;
    // println!("The value of one is: {}", one);

    // // ARRAYS
    // let a: [i32; 5] = [1, 2, 3, 4, 6];

    // let months: [&str; 12] = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // let arr: [i32; 5] = [0; 5];

    // let last_month = months[months.len() - 1];

    // let condition: bool = true;

    // // let number = if condition {5} else {"six"}; // Compiler Error
    // let number = if condition { 5 } else { 6 };

    // // returning from loops
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // // looping through a collection
    // let a = [10, 20, 30, 40, 50];

    // for item in a.iter() {
    //     println!("The value is: {}", item);
    // }

    // let the_list = (1..4);
    // let the_last_range = the_list.last();

    // // for in range
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // // Ownership Rules
    // let mut s = String::from("Omotoye"); // this type of string can be mutated
    // let mut name = "Omotoye"; // this type of string cannot be mutated

    // let mut s2 = s;

    // // println!("{}", s); // Compiler Error, because the ownership of the value in s has been passed to s2
    // let r1 = &s2;
    // let r3 = &s2;

    // let r2 = &mut s2;
    // println!("{}", r2);
    // println!("{}", r2);

    // let mut s = String::from("hello world");
    // let word = first_word(&s[..]); // word will get the value of 5
    // let another_word = first_word("Omotoye ADekoya"); // word will get the value of 5

    // // s.clear(); // this empties the String, making it equal to ""
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];

    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
    // println!("{:?}", rect1);
    // println!("{}", rect1.area());
    // println!("{}", Rectangle::add(2, 4));

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // // Using the enum definition
    // let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind2::V6(String::from("::1"));

    // /******* The Option Enum **** */
    // let some_number = Some(5);
    // let some_string = Some("a string");

    // let absent_number: Option<i32> = None;

    // println!("{:?}", absent_number);

    // let dollar = Currency::Dollar(String::from("$"));
    // let pounds = Currency::Pounds(String::from("£"));
    // let euro = Currency::Euro(String::from("€"));
    // let dollar_value = value_of_currency(dollar);
    // let pounds_value = value_of_currency(pounds);
    // let euro_value = value_of_currency(euro);

    // println!("{}", dollar_value);
    // println!("{}", pounds_value);
    // println!("{}", euro_value);

    // Crating a new vector
    // let v: Vec<i32> = Vec::new();
    // let num_list: Vec<i8>  = Vec::from([1, 2, 3, 4, 5]);
    // println!("{:?}", num_list);
    // println!("{:?}", v);

    // // creating a new vector with the vec! macro
    // let mut v = vec![1, 2, 3];
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // println!("{:?}", v);

    // let half_v = &v[3..];
    // let third_v = &v[2];
    // let third_v = v.get(2);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    // println!("{:?}", half_v);

    // // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // match does_not_exist {
    //     Some(not_exist) => println!("{}", not_exist),
    //     None => println!("Does not Exist"),
    // }

    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v{
    //     *i += 50;
    // }
    // // println!("{}", does_not_exist);
    // println!("I'm still running!");

    // let row  = vec![
    //     SpreadsheelCell::Int(3),
    //     SpreadsheelCell::Text(String::from("blue")),
    //     SpreadsheelCell::Float(10.12),
    // ];

    // Creating a New String

    // let mut s = String::new();
    // s = "Omotoye".to_string();

    // let hello_in_ufo = String::from("ϢϜϴϠϋϡϼδϟ΍");
    // let hello_in_weird2 = String::from("Dobrý den");
    // let hello_in_english = String::from("Hello");
    // let hello_in_weird3 = String::from("ʭˣʬʝʕʹ");
    // let hello_in_indu = String::from("नमस्ते");
    // let hello_in_arabic = String::from("こんにちは");
    // let hello_in_weird4 = String::from("안녕하세요");
    // let hello_in_chineese = String::from("你好");
    // let hello_in_spanish = String::from("Olá");
    // let hello_in_russian = String::from("Здравствуйте");
    // let hello_in_spanish2 = String::from("Hola");

    // println!("{}", hello_in_weird2);
    // println!("{}", hello_in_weird3);
    // println!("{}", hello_in_weird4);
    // println!("{}", hello_in_english);
    // println!("{}", hello_in_arabic);
    // println!("{}", hello_in_chineese);
    // println!("{}", hello_in_indu);
    // println!("{}", hello_in_russian);
    // println!("{}", hello_in_spanish);
    // println!("{}", hello_in_spanish2);
    // println!("{}", hello_in_ufo);

    // // Updating a string
    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);

    // let len = String::from("Hola").len();
    // println!("{}", len);
    // let len = String::from("Здравствуйте").len();

    // let answer = &hello_in_russian[0..2];
    // let answer2 = &hello_in_english[2..=2];

    // println!("{}", len);
    // println!("{}", answer);
    // println!("{}", answer2);

    // // Itereating over Strings
    // for c in hello_in_arabic.chars() {
    //     println!("{}", c);
    // }
    // for c in hello_in_indu.chars() {
    //     println!("{}", c);
    // }

    // for c in hello_in_indu.bytes() {
    //     println!("{}", c);
    // }

    // // Creating a New Hash Map
    // let mut scores: HashMap<String, i32> = HashMap::new();

    // scores.insert(String::from("Hello"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // // Another way of crating Hash Map
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // // Ownership with HashMaps
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // let score = scores.get("Yellow");
    // match score {
    //     Some(value) => println!("The value of yellow is {}", value),
    //     None => println!("The given key does not exist")
    // }
    // // field_name and field_value are invalid at this point,
    // // using them would cause a compiler error

    // scores.insert(String::from("Yellow"), 60);
    // // Iterating over HashMap
    // for (key, value) in &scores{
    //     println!("{}: {}", key, value);
    // }

    // // Only inserting a value if the given key has no value
    // let the_value = scores.entry(String::from("Yellow")).or_insert(50);
    // println!("The value is {}", the_value);

    // println!("The new value is {}", scores.get("Yellow").unwrap());

    /* ERROR HANDLING  */

    // The `panic!` macro

    // panic!("crash and burn");

    // // Using a panic! Backtrace
    // let v = vec![1, 2, 3];
    // // v[99];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {

    //     }
    // });

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    /*** GENERICS TYPES, TRAITS, AND LIFETIMES */

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!(
    //     "The largest number from the list {:?} is {}",
    //     number_list, result
    // );

    // let number_list = vec![102, 34, 6000, 89, 50, 25, 2, 100, 65];
    // let result = largest(&number_list);
    // println!(
    //     "The largest number from the list {:?} is {}",
    //     number_list, result
    // );

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let int_and_float = Point { x: 1.0, y: 10 };


    //***** Validating References with LIfetimes */
    {
        let r; 

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
}

// struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// struct NewsArticle2 {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }
// struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: String,
//     pub retweet: String,
// }

// /****** Defining Traits ******* */
// trait Summary {
//     fn summarize(&self) -> String;
// }

// trait Summary2 {
//     fn summarize(&self) -> String{
//         String::from("(Read more...)")
//     }
// }


// // Implementing a trait on a Type
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// impl Summary2 for NewsArticle2{}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn largest(list: &Vec<i32>) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

/**** USING GENERIC TO REPRESENT THE largest function */
// fn generic_largest<T>(list: &Vec<T>) -> T{
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// enum SpreadsheelCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// #[derive(PartialEq)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }

// if coin == Coin::Penny {
//     1
// } else if coin == Coin::Nickel {
//     5
// } else if coin == Coin::Dime {
//     10
// } else if coin == Coin::Quarter {
//     25
// } else {
//     unimplemented!()
// }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// enum Currency {
//     Dollar(String),
//     Pounds(String),
//     Euro(String),
//     Naira(String),
// }

// fn value_of_currency(currency: Currency) -> f32 {
//     match currency {
//         Currency::Dollar(sign) => {
//             println!("The sign of Dollar is {}", sign);
//             1.0
//         }
//         Currency::Euro(sign) => {
//             println!("The sign of Euro is {}", sign);
//             1.35
//         }

//         Currency::Pounds(sign) => {
//             println!("The sign of Pounds is {}", sign);
//             1.23
//         }

//         Currency::Naira(sign) => {
//             println!("The sign of Naira is {}", sign);
//             750.0
//         }
//     }
// }
