use std::fmt::Display;
// A tuple is a collection of values of different types. Tuples are constructed
// using parentheses (), and each tuple itself is a value with signature (T1, T2, ...)
// where T1, T2 are the types of its members. Functions can use tuples to return
// multiple values, as tuples can hold any number of values.

#[allow(unused)]
// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of  a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

/******** Activity *********/
// 1. Recap: Add the `fmt::Display` trait to the Matrix struct in the above example, so that if
// you switch from printing the debug format `{:?}` to the display format `{}`, you see the
// following output:
// ( 1.1 1.2 )
// ( 2.1 2.2 )

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// 2. Add a transpose function using the reverse function as a template, which accepts a matrix
// as an argument, and returns a matrix in which two element have been swapped.

// as a method --->
impl Matrix {
    fn transpose(self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

// as a static function --->
impl Matrix {
    fn transpose2(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }
}

// as a normal function --->
fn transpose3(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[allow(unused)]
fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2464, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, (2u32, -5i8)), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {tuple_of_tuples:?}");
    println!("tuple of tuples: {tuple_of_tuples:#?}");

    // Accessing tuple of tuple value
    println!(
        "Value of [0][2][1] in tuple of tuples: {}",
        tuple_of_tuples.0 .2 .1
    );

    // long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compliter error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuples can be destructed to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!();
    // println!("Transpose:\n{}", matrix.transpose());
    // println!("Transpose:\n{}", Matrix::transpose2(matrix));
    println!("Transpose:\n{}", transpose3(matrix));
}
