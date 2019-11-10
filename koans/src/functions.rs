pub fn functions() {
    // functions in rust are specified with `fn` keyword
    // functions can take parameters and you must specify the type
    // the parameter must be
    fn adder(x: i32, y: i32) -> i32 {
        x + y
    }
    assert!(adder(1, 2) == __);

    fn log_statement() {
        println!("Ahoy matey!");
    }

    log_statement();
}

#[test]
fn should_test_functions() {
    functions();
}
