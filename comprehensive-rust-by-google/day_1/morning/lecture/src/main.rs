fn main() {
    // Program entry point
    let mut x: i32 = 6; // Mutable variable binding
    print!("{x}"); // Macro for printing, like printf
    while x != 1 {
        // No parenthesis around expression
        if x % 2 == 0 {
            // Math like in other languages
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();

    /*** Compound Types */

    // Array assignment and access:
    println!("\nArray assignment and access: \n==============================");
    let mut a: [i8; 10] = [42; 10]; // Arrays: (Types) [T; N], (Literals) [20, 30, 40], [0; 3]
    a[5] = 0;

    println!("a: {:?}", a); // {:?} -> Debug format of printing (only works for types with the **Debug Trait**)

    // Tuple assignment and access:  // Tuples: (Types) (T1, T2, T3, ...), (Literals) ('x', 1.2, 0)
    println!("\nTuple assignment and access: \n==============================");
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0); // tuple members can be accessed with the dot operator
    println!("2nd index: {}", t.1);

    /*** References ****/
    println!("\nReferences: \n============");
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 21;
    let count_result = ref_x.count_ones(); // the `.count_ones()` method counts the number of ones in the binary representation of the calling object
    println!("x: {x}");
    println!("count result: {count_result}");

    /*
    Some differences from C++:
        * We must dereference `ref_x` when assigning to it, similar to C pointers,
        * Rust will auto-dereference in some cases, in particular when invoking methods
    */

    /*** Dangling References ***/

    let ref_x;
    {
        let x: i32 = 10;
        ref_x = &x;
    }

    // println!("ref_x: {ref_x}"); // Compiler error: `x` does not live long enough
    // Rust will statically forbid dangling references

    /**** Slices ** */
    // A slice gives you a view into a larger collection:

    println!("\nSlices: \n============");
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4]; // [2..4] is the same as [2:4] in python
                              // a[3] = 4; // Compiler error: slices borrow data from the sliced type, therefore you cannot modify day that is being borrow
    println!("s: {s:?}");

    /*** `String` vs `str` *****/

    // `&str` is an immutable reference to a string slice.
    // `String` is a mutable string buffer.

    let s1: &str = "Hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    /*** Functions *** */
    println!("\nFunctions: \n============");
    fizzbuzz(20); // Defined below, no forward declaration needed

    /**** Methods *** */
    println!("\nMethods: \n============");
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());


    /***** Function Overloading ***** */
    println!("\nFunctions Overloading: \n===========================");
    println!("coin toss: {}", pick_one("heads", "tails")); 
    println!("cash prize: {}", pick_one(500, 1000)); 

}

/*** Functions *** */
// A Rust version of the famous `FizzBuzz` interview question:

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // Corner case (zero division error), early return
    }
    lhs % rhs == 0 // The last expression is the return value (do not add a semicolon at the end of the last statement to make it a return value)
}

fn fizzbuzz(n: u32) -> () {
    // `()` means no return value
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) {
    // `-> ()` is normally omitted
    for n in 1..=n {
        // `1..=n` mean 1 to n including n
        fizzbuzz(n);
    }
}

/***** Methods **** */

// Creating Rectangle Struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

/***** Function Overloading ***** */

/*
Overloading is not supported:

* Each function has a single implementation
    * Always takes a fixed number of parameters.
    * Always takes a single set of parameter types.
* Default values are not supported:
    * All call sites have the same number of arguments
    * Macros are sometimes used as an alternative.
*/

// However, function parameters can be generic:
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}
