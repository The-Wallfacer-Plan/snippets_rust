mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    // pub mod inside { // can make the inner func visible
    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn main() {
    outermost::middle_function();
    // outermost::middle_secret_function(); // cannot call this one here
    outermost::inside::inner_function();
    // outermost::inside::secret_function();
}