#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
}
