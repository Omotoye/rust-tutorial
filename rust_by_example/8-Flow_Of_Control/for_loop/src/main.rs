// for and range
// The for in construct can be used to iterate an `Iterator`. One of the easiest
// ways to create an iterator is to ue the range notation a..b. This yields values
// from a (inclusive) to b (exclusive) in steps of one.

// Writing FizzBuzz using for instead of while
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // alternatively 1..=100 can be used to include both ends.
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // By default the for loop will apply the `into_iter` function to the
    // collection. However, this is not the only means of converting collection
    // into_iter, iter and iter_mut all handle the conversion of a collection
    // into an iterator in different ways, by providing different views on the data
    // within.

    // * `iter` - This borrows each element of the collection through each iteration
    // Thus leaving the collection untouched and available for reuse after the loop.

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // * `into_iter  - This consumes the collection so that on each iteration the
    // exact data is provided. Once the collecton has been consumed it is no longer
    // available for reuse as it has been 'moved' within the loop

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Err: a move has occured
    // println!("names: {:?}", names);

    // * `iter_mut` - This mutably borrows each element of the collection, allowing for the
    // collection to be modified in place

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
