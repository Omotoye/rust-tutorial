// The type inference engine does more than looking at the type of the value
// expression during  an initialization. It also looks at how the variable is
// used afterwards to infer its type.
fn main() {
    // Becasue of the annotation, the compiler knows that `elem` has type u8
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just know that it's a vector of something (`Vec<_>`)

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);
}