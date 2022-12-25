mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        pub fn take_payment() {}

        pub mod back_of_house {
            // use crate::eat_at_restaurant;
            use crate::eat_at_restaurant;

            use super::serve_order;

            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Self {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            fn fix_incorrect_order() {
                cook_order();
                // Absolute path
                crate::front_of_house::front_of_house::serving::serve_order();
                serve_order();

                eat_at_restaurant();
                eat_at_restaurant();
            }

            fn cook_order() {}
        }
    }
}
