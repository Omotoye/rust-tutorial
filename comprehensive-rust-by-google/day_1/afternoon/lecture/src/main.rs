/**** VAriables: Static and Constant Variables  **** */
// `const`
// you can declare compile-time constants
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

// `static`
// you can also declare static variables
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    /**** VAriables **** */

    println!("\nVAriables\n==================");
    // Rust provides type safety via static typing. Variable bindings
    // are immutable by default
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20; // Compiler Error: cannot assign twice to immutable variable
    // println!("x: {x}");

    /**** VAriables: Type Inference  **** */
    println!("\nVAriables: Type Inference:\n==============================================");

    // Rust will look at how the variable is used to determine the type:
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y); // Compiler Error: Argument to the function incorrect, this is because the first use of y has implied that y is of type i8

    /**** VAriables: Static and Constant Variables  **** */
    println!("\nVAriables: Static and Constant Variables:\n==============================================");
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
    println!("{BANNER}");

    /**** VAriables: Scopes and Shadowing  **** */
    println!("\nVAriables:  Scopes and Shadowing\n==============================================");
    // You can shadow variables, both those from outer scopes and variables from the same scope
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }
    println!("after: {a}");

    /******* Memory Management *******

    **The Stack vs The Heap**
    * Stack: Continuous area of memory for local variables
        * values have fixed sizes known at compile time.
        * extremely fast: just move a stack pointer.
        * easy to manage: follows functions calls.
        * great memory locality.
    * Heap: Storage of values outside of function calls
        * values have dynamic sizes determined at runtime.
        * slightly slower than the stack: some book-keeping needed.
        * no guarantee of memory locality.
    */

    /****** Memory Management: Stack Memory ****** */
    // Creating a `String` puts a fixed-sized data on the stack and dynamically sized data on the heap:
    let s1 = String::from("Hello");

    /****** Ownership ******
    All variable bindings have a scope where they are valid and it is an error to use
    a variable outside its scope:

    */

    println!("\nOwnership\n==================");

    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    } // p is drop from here and no longer valid
      // println!("y: {}", p.1); // Compiler Error: cannot find value `p` in this scope

    /****** Ownership: Move Semantics ******/

    println!("\nOwnership: Move Semantics\n===============================");
    // An assignment will transfer ownership between variables:
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}"); // Compiler Error: borrow of moved value
    /*
     * The assignment of `s1` to `s2` transfers ownership.
     * The data was moved from `s1` and `s1` is no longer accessible.
     * When `s1` goes out of scope, nothing happens; it has no on ownership.
     * When `s2` goes out of scope the string data is freed.
     * There is always exactly one variable binding which owns a value.
     */

    let name = String::from("Omotoye");
    println!("\nOwnership: Move in Function Calls\n===================================");
    say_hello(name);
    // say_hello(name);   // Compiler Error; Value used after move

    /**** Copying and Cloning *****
    While move semantics are the default, certain types are copied by default
    */
    println!("\nOwnership: Copying and Cloning\n===================================");
    let x = 42;
    let y = x; // These types use the `Copy` trait
    println!("x: {x}");
    println!("y: {y}");
    // You can opt-in your own types to use copy semantics:

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    /* Because of the `Copy` trait that was added to this struct,
     * After the assignment, both `p1` and `p2` own their own data.
     * We can also use `p1.clone()` to explicitly copy the data
     */

    /****** Borrowing ******
     * Instead of transferring ownership when calling a function, you can let a
     * function borrow the value:
     */
    println!("\nOwnership: Borrowing\n===================================");
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");

    /****** Shared and Unique Borrows
     * Rust puts constrainst on the ways you can borrow values:
     * YOu can have one or more `&T` immutable reference to a value at any given time, or
     * You can have exactly one `&mut T` mutable reference to a value.
     */

    let mut a: i32 = 10;
    let b = &a;

    {
        let c = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    //  println!("b: {b}"); // Compiler Error

    /********* Lifetimes **************

    A borrowed value has a lifetime:
     * The lifetime can be elided: `add(p1: &Point, p2: &Point) -> Point`
     * LIfetimes can also be explicit: `&'a Point, &'document str`
     * Read `&'a Point` as "a borrowed `Point` which is valid for at least the lifetime `a`".
     * LIfetimes are always inferred by the compiler: you cannot assign a lifetime yourself.
        * Lifetime annotations create constraints; the compiler verifies that there is a valid solution.

    */

    /********** Lifetimes in Function Calls ********/
    let p1 = Point(10, 10);
    let p2 = Point(20, 20); // Put into different scope
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);

    /******** LIfetimes in Data Structures ******* */
    println!("\nLIfetimes in Data Structures \n===================================");
    let text = String::from("The quick brown fox jumps over the lazy dog."); 
    let fox = HighLight(&text[4..19]); 
    let dog = HighLight(&text[32..43]); 
    // erase(text); // Compiler error: cannot move out of text because text is borrowed
    println!("{fox:?}");
    println!("{dog:?}");

}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE]; // `.unwrap_or(provided_default)` returns the contained `Some` value or a provided default
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

/* Moves in Function calls  */
fn say_hello(name: String) {
    println!("Hello {name}");
}

/***** Borrowing
 * The `add` function borrows two points and return a new point
 * The caller retains ownership of the inputs
*/
fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

/******** LIfetimes in Function Calls *********/
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}
/* 
    * `'a` is a generic parameter, it is inferred by the compiler. 
    * Lifetimes start with ' and 'a is a typical default name. 
    * Read `&'a Point` as "a borrowed Point which is valid for at least the lifetime a". 
        * The at least part is important when parameters are in different scopes 
*/

/***** Lifetimes in Data Structures *****
if a data type stores borrowed data, it must be annotated with a lifetime. 
*/

#[derive(Debug)]
struct HighLight<'doc>(&'doc str); 

fn erase(text: String) {
    println!("Bye {text}!");
}

// You can opt-in your own types to use copy semantics:
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);
