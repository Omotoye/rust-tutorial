mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub fn eat_at_restaurant() {
    use front_of_house::serving::back_of_house;
    // Order a breakfast i the summer with Rye toast 
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like 
    meal.toast = String::from("Wheat"); 
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed 
    // to see or modify the seasonal fruit that comes with the meal 
    // meal.seasonal_fruit = String::from("blueberries");
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
