pub fn functions() {
    // TODO: Read the https://www.marcelbelmont.com/rustlang-workshop/docs/functions.html section
    // for more information

    // functions in rust are specified with `fn` keyword
    // functions can take parameters and you must specify the type
    // the parameter must be
    fn adder(x: i32, y: i32) -> i32 {
        x + y
    }
    assert!(adder(1, 2) == __);

    // this function has no parameters and that is totally valid.
    fn log_statement() {
        println!("Ahoy matey!");
    }

    log_statement();

    // rust has anonymous functions called closures
    let difference = |x: i32, y: i32| x - y;
    assert!(difference(3, 2) == __);
}
