#![allow(dead_code)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }
}    


pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast { toast: "wheat".to_string(), seasonal_fruit: "blueberries".to_string() } ;
    println!("I'd like {} toast and {} please", meal.toast, meal.seasonal_fruit);
    meal.toast = String::from("rye");
    meal.seasonal_fruit = String::from("peaches");
    println!("Actually, I'd like {} toast and {} please", meal.toast, meal.seasonal_fruit);

}    


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
