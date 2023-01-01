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

struct Point(i32, i32);
