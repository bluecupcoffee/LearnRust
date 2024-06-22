pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {
            crate::front_of_house::serving::serve_order();
            super::serving::serve_order();
        }
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restuarant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
