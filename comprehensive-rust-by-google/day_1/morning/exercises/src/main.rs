fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    // Rust will not automatically apply implicit conversions between types
    // println!("{x} * {y} = {}", multiply(x, y)); // Compiler Error: incorrect argument

    /*
    Solution to this problem:
        The Rust integer types all implement the `From<T>` and `Into<T>` traits
        traits; are operations that a type can perform
            In the case of the `x` variable to convert x from i8 to i16, you can either use the `From<T>` or `Into<T>` trait
        * `x.into()` -> convert x into the destination type
        * `i16::from(x)` -> converts x into i16
    */

    /**** Exercise 1 ***

    1. Execute the above program and look at the compiler error.
    2. Update the code above and use `into()` to do the conversion.
    3. Change the types of `x` and `y` to other things (such as `f32`, `bool`, `i128`) to
        see which types you can convert to which other types. Try converting small types to
        big types and the other way around. Check the `stand library documentation` to see
        if `From<T>` is implemented for the paris you check.
    */

    // 2. Solution
    println!("{x} * {y} = {}", multiply(x.into(), y));

    // 3. Solution
    println!("x (i8): {}", i8::from(x));
    println!("x (i16): {}, y (i16): {}", i16::from(x), i16::from(y));
    println!("x (i32): {}, y (i32): {}", i32::from(x), i32::from(y));
    println!("x (i64): {}, y (i64): {}", i64::from(x), i64::from(y));
    println!("x (i128): {}, y (i128): {}", i128::from(x), i128::from(y));
    println!("x (f32): {}, y (f32): {}", f32::from(x), f32::from(y));
    println!("x (f64): {}, y (f64): {}", f64::from(x), f64::from(y));

    /**** Exercise 2 ***

    Use the above to write a function `pretty_print` which pretty-print a matrix and
    a function `transpose` which will transpose a matrix (turn rows into columns):

    Hard-code both functions to operate on 3 x 3 matrices.
    */

    // Solution
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("\nmatrix:\n=========");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("\ntransposed:\n===========");
    pretty_print(&transposed);

    /***** Bonus Question *****
     
    Could you use &[i32] slices instead of hard-coded 3 × 3 matrices
    for your argument and return types? Something like &[&[i32]]
    for a two-dimensional slice-of-slices. Why or why not? 
    
    */

    // ??????????????? NOT SURE WHAT THE ANSWER IS 
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    [
        [matrix[0][0], matrix[1][0], matrix[2][0]],
        [matrix[0][1], matrix[1][1], matrix[2][1]],
        [matrix[0][2], matrix[1][2], matrix[2][2]],
    ]
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}
