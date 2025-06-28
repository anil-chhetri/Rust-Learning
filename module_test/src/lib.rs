use crate::back_of_house::struct_with_public::BreakFast;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> () {
            println!("added to waitlist");
        }
    }
}

fn deliver_order() {
    print!("delivered order");
}

mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        crate::deliver_order();
    }

    fn cook_order() {
        println!("cooking in progress")
    }

    pub mod struct_with_public {
        pub struct BreakFast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl BreakFast {
            pub fn Summer(toast: String) -> BreakFast {
                BreakFast {
                    toast: toast,
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub mod enum_with_public {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let _meal = back_of_house::struct_with_public::BreakFast::Summer("rye".to_string());

    // since the fields are private can't even create a Breakfast.
    // let breakfast = BreakFast {
    //     toast: "".to_string(),
    //     seasonal_fruit: "".to_string(),
    // };

    // enums -> all the element doesn't have to public.
    let _order1 = back_of_house::enum_with_public::Appetizer::Salad;
}
