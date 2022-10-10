// Printing is handled by a series of macros defind in std::fmt
// some of which include:
// * format! -> write formatted text to String
// * print! -> same as format! but the text is printed to the console (io::stdout)
// * println! -> same as print! but a newline is appended.
// * eprint! -> same as print! but the text is printed to the standard error (io::stderr)
// * eprintln! -> same as eprint! but a newline is appended
//
// All parse text in the same fashion. As a plus, Rust checks formatting at compile time.

fn main() {
    // In general, the `{}` will be automatically replace with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("\n{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "\n{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character after a `:`
    println!("\nBase 10 repr:               {}", 69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);

    // You can right align text with a specified width. This will output
    // "     1". 4 white spaces and a "1", for a total width of 5.
    println!("\n{number:>5}", number = 1);

    // You can pad numbers with extra zeroes. This will output "00001"
    println!("\n{number:*>5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond" /*, "James" */)
    println!("\nMy name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`
    // User-defined types do not implement fmt::Display by default

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    // println!("This struct `{}` won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from 
    // a surrounding variable. Just like the above, this will output 
    // "    1". 5 white spaces and a "1"
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    /*** Activities ******/
    // Add a println! macro call that prints: Pi is roughly 3.142 by controlling
    // the number of decimal places shown. For the purpose of this exercise, 
    // use let pi = 3.141592 as an estimate for pi. 
    // (Hint: you may need to check the std::fmt documentation for setting the 
    // number of decimals to display)
    let pi: f64 = 3.141592;
    println!("\nPi is roughly {pi:.3}");

}

// std::fmt contains many traits which govern the diplay of text. The base form
// of two important ones are listed below:
// * fmt::Debug -> Uses the {:?} marker. Format text for debugging purposes. 
// * fmt::Display -> Uses the {} marker. Format text in a more elegant, user friendly fashion.

// Here, we used fmt::Display because the std library provides implementations for these 
// types. To print text for custom types, more steps are required.

// Implementing the fmt::Display trait automatically implements the ToString trait which allows
// us to convert the type to String.