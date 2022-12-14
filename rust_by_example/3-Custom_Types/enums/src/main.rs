// The `enum` keyword allows the creation of a type which may be one of a few different
// variants. Any variant which is valid as a `struct` is also valid as an enum.

// Create an `enum` to classify a web event. Note how both names
// and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
//  Each is different and independent
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,

    // like tuple structs,
    KeyPress(char),
    Paste(String),

    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),

        // Destructing `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),

        // Destructing `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

/********* Type aliases ************/
// if you use a type alias, you can refer to each enum variant via its
// alias. This might be useful if the enum's name is too long or too generic,
// and you want to rename it.
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// alias is also usually used in the impl block, as `Self` which reps
// the name of the type the implementation is done on

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');

    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // We can refer to each variant via its alias, not its
    // long and inconvenient name.
    let x = Operations::Add;
}
