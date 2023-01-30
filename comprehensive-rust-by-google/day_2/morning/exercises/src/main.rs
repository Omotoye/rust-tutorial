// TODO: remove this when you're done with your implementation
#![allow(unused_variables, dead_code)]

struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User {
            name: name,
            age: age,
            weight: weight,
        }
    }

    pub fn name(&self) -> &str {
        
    }
}

fn main() {
    unimplemented!()
}
