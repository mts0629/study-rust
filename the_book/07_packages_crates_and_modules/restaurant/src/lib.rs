// Define a module: `front_of_house`
mod front_of_house {
    // Submodule: `hosting`
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Submodule: `serving`
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
