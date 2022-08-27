fn main() {
    // loop {
    //     println!("again!");
    // }

    // returning values from loops
    // to do this, add the value you want retured after the break expression
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    /* Loop Labels to disambiguate between multiple loops */
    // loop label must begin with a single quote
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    /* while loops */
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    /* Looping through a collection with for */

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");
}
