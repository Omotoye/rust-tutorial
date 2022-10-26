#![allow(unused)]

use clap::Parser;
use std;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    // pattern: pattern,
    // path: std::path::PathBuf::from(&path),
    // };

    let args = Cli::parse();

    // Trying to print the arguments that was given
    println!(
        "The pattern to find in the file {:?} is {}",
        &args.path, &args.pattern
    );

    // Opening the file that we got
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // Opening the file that we got with nicer error reporting
    let mut content = String::from("");
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(the_content) => {
            // println!("File content: {}", the_content);
            content = the_content;
        }
        Err(error) => println!("Oh noes: {}", error),
    }

    // Unwrapping
    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => {content},
    //     Err(error) => { panic!("Oh noes: {}", error)}
    // };

    // Shortcut method for the above
    // let content = std::fs::read_to_string(&args.path).unwrap();
    // println!("file content: {}", content);

    // No need to panic
    // let result = std::fs::read_to_string("test.txt");
    // let _content = match result {
    // Ok(content) => content,
    // Err(error) => {
    // return Err(error.into());
    // }
    // };

    // let content = std::fs::read_to_string("test.txt")?;
    // println!("file content: {}", content);
    // Ok(())

    // Attempting to read the file content with `BufReader`
    let _same_content_with_bufreader = BufReader::new(&std::fs::File::open(&args.path).unwrap());

    // Iterating over the lines and printing each one that contains our pattern
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
