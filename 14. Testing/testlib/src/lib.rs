pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)] //test can be called with cargo test
mod tests {
    use super::*;

    #[test] //this tells the compiler the following function is a test function
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore = "intentionally failing test"]
    fn it_fails(){
        let result = add(2, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn assert_not_equals(){
        let result = add(2, 2);
        assert_ne!(result, 3); //assert not equal
    }

    //specific tests can be called with cargo test <name of test>
    //or you can use the partial name like 'cargo test it' to get all tests which start with those chars
}
