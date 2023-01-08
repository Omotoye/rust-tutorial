use rand::Rng;

use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!(
            "{}: size {} bytes, align: {} bytes",
            stringify!($t),
            size_of::<$t>(),
            align_of::<$t>()
        );
    };
}

/********* Struct *******
LIke C and C++, Rust has support for custom structs:
*/

#[derive(Debug)] // Add `Debug` trait to the struct, which then allows you to print as it is using Debug printing
struct Person {
    name: String,
    age: u8,
}

// If you already have variables with the right names, then you can create the struct using a shorthand:
impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

/********* Tuple Struct *******/
struct Point(i32, i32);
struct PoundOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    unimplemented!()
}

/********************** Enums *********************
The `enum` keyword allows the creation of a type which has a few different variants:
*/
fn generate_random_number() -> i32 {
    rand::thread_rng().gen()
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

// Variant Payloads
// You can define richer enums where the variants carry data. You can then use
// the `match` statement to extract the data from each variant:

enum WebEvent {
    PageLoad,                 // Variant without payload
    KeyPress(char),           // Tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

enum Foo {
    A,
    B,
}

/********************** Enums: Methods *********************
 Rust allows you to associate functions with your new types. You do this with an `impl` block:
*/

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

/************* Method Receiver
    * &self: borrows the object from the caller using a shared and immutable reference. The object
        can be used again afterwards.
    * &mut self: borrows the object from the caller using a unique and mutable reference. The object
        can be used again afterwards.
    * self: takes ownership of the object and moves it away from the caller. The method becomes the
        owner of the object. The object will be dropped (deallocated) when he method returns, unless
        it's ownership is explicitly transmitted.
    * No receiver: this becomes a static method on the struct. Typically used to create constructors
        which are called `new` by convention.
*/

// Method Receiver Example
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {
        // No receiver, a static method
        Race {
            name: String::from(name),
            laps: Vec::new(),
        }
    }
    fn add_lap(&mut self, lap: i32) {
        // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

#[rustfmt::skip]
fn main() {
    /********* Struct *******/
    println!("\n\nStructs\n===================");
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");

    println!("{} is {} years old", peter.name, peter.age);

    /********* Tuple Structs *******
     This is often used for single-field wrappers(called newtypes):
    */
    println!("\n\nTuple Structs\n===================");
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    // let force = compute_thruster_force();
    // set_thruster_force(force); // Compiler Error; mismatched types

    /********************** Enums *********************/
    println!("\n\nEnums\n===================");
    println!("You got: {:?}", flip_coin());

    /********************** Enums: Variant Payloads *********************/
    println!("\n\nEnums: Variant Payloads\n===================");

    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);

    println!("\n\nEnums: Sizes\n===================");
    dbg_size!(Foo);

    /********************** Enums: Methods *********************/
    println!("\n\nEnums: Methods\n===================");
    peter.say_hello();

    /************** Method Receiver Example **************/
    println!("\n\nMethod Receiver Example\n========================");
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

    /************** Pattern Matching ************
    The `match` keyword let you match a value against one or more patterns. The comparisons
    are done from top to bottom and the first match wins

    The patterns can be simple values, similar to `switch` in C and C++
    */
    println!("\n\nPattern Matching\n========================");
    let input = 'x';

    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        _ => println!("Something else"), // the `_` pattern is a wildcard pattern which matches any value
    }
    /************** Destructuring Enums ************/
    let n = 100;
    println!("\n\nDestructuring Enums\n=======================");
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }

    /************** Destructuring Structs ************/
    println!("\n\nDestructuring Structs\n=======================");
    let foo = StructFoo {x: (1, 2), y: 3}; 
    match foo {
        StructFoo {x: (1, b), y} => println!("x.0 = 1, b = {b}, y = {y}"), 
        StructFoo { y: 2, x: i} => println!("y = 2, i = {i:?}"), 
        StructFoo {y, ..} => println!("y = {y}, other fields were ignored"), 
    }   
 
    /************** Destructuring Arrays ************/
    println!("\n\nDestructuring Arrays\n=======================");
    let triple = [0, -2, 3]; 
    println!("Tell me about {triple:?}"); 
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"), 
        [1, ..] => println!("First is 1 and the rest were ignored"), 
        _ => println!("All elements were ignored"), 
    }

    /*************** Match Guards *********
    When matching you can add a guard to a pattern. This is an arbitrary boolean expression which will
    be executed if the pattern matches. 
    */

    println!("\n\nMatch Guards\n=======================");
    let pair = (2, -2);
    println!("Tell me about {pair:?}"); 
    match pair {
        (x, y) if x == y => println!("These are twins"), 
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"), 
        (x, _) if x % 2 == 1 => println!("The first one is odd"), 
        _ => println!("No correlation..."), 
    }
}

/************** Destructuring Enums ************
 Patterns can also be used to bind variables to parts of your values. This is how you inspect
 the structure of you types.
*/

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {} into two equal parts", n))
    }
}

struct StructFoo {
    x: (u32, u32),
    y: u32,
}
