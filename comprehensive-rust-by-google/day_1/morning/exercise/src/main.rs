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
    let mut a: [i8; 10] = [42; 10]; // Arrays: (Types) [T; N], (Literals) [20, 30, 40], [0; 3]
    a[5] = 0;

    println!("a: {:?}", a); // {:?} -> Debug format of printing (only works for types with the **Debug Trait**)

    // Tuple assignment and access:  // Tuples: (Types) (T1, T2, T3, ...), (Literals) ('x', 1.2, 0)
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0); // tuple members can be accessed with the dot operator
    println!("2nd index: {}", t.1);

    /*** References ****/
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

    // Dangling References 

    let ref_x; 
    {
        let x: i32 = 10; 
        ref_x = &x; 
    }

    // println!("ref_x: {ref_x}"); // Compiler error: `x` does not live long enough 
    // Rust will statically forbid dangling references 
    
}
