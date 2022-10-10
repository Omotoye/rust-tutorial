fn main() {

    // println! is a macro that prints text to the console
    println!("Hello, world!");

    /****** Activity *******/
    println!("I'm a Rustacean");

    
    /***** Comments *******/
    // Varieties of comment type supported by Rust...

    // ------Regular Comments ---> which are ignored by the compiler
    // Line comments which go to the end of the line.
    /* Block comments which go to the closing delimiter. */

    // ------Doc Comments ---> which are parsed into HTML library
    // /// Generate library docs for the following item.
    // //! Generate library docs for the enclosing item.
    
    /**
     * This is another type of comment, a block comment. In general, 
     * line comments are the recommended comment style, but block comment
     * are extremely useful for temporarily disabling chuncks of code.
     * 
     * @author: Omotoye Shamsudeen Adekoya
     */

    // You can manipulate expression more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:

    // let x = 5 + /* 90 + */ 5;
    let x = 5 + 90 + 5;
    println!("Is `10 or 100? x = {}", x);
}
