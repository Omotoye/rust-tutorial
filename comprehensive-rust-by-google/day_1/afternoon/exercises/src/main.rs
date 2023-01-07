/******************* EXERCISE 1:  Designing a library *****************/
// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.len() <= 0
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for (index, book) in self.books.iter().enumerate() {
            println!("{index}: {book}");
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        self.books.get(0)
    }
}

fn main() {
    // Little about the `Vec<T>` API
    let mut vec = vec![10, 20];
    vec.push(30);
    println!("middle value: {}", vec[vec.len() / 2]);

    for item in vec.iter() {
        println!("item: {item}");
    }

    /******************* EXERCISE 1:  Designing a library *****************/
    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());
    //
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    library.print_books();
    //
    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }
    //
    println!("Our library has {} books", library.len());


    /********** Iterators and Ownership ************
    Iterator 
    ===========

    Traits are like interfaces: they describe behavior (methods) for a type. The 
    `iterator` trait simply says that you can call `next` until you get `None` back:
    */
    

}
