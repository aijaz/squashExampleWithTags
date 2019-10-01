#![allow(dead_code)]
#![allow(unused_variables)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
    

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
