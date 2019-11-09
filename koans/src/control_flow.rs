pub fn control_flow() {
    // variable with type declaration
    let flag: bool = false;

    // variables are immutable by default so
    // we need to add `mut` keyword to make it mutable
    let mut actual = String::from("");

    // type is inferred from the assignment
    let expected = String::from("Righteous Path");
    
    // notice there is not any `()` in this if block
    // like other c-type languages
    if flag {
        actual = String::from("Path");
    }

    assert!(actual == expected);

    // Rust lang doesn't have a `switch` keyword
    // instead it has a `match` keyword that has
    // advanced pattern matching
    let number: i32 = 5;
    let mut word = String::from("");
    match number {
        1 => {
            print!("Not a prime")
        },
        3 | 5 => {
            println!("3 or 5 are prime numbers");
            word = "is a prime".to_string();
        },
        _ => println!("Not a prime"),
    }
    assert!(word == "is a prime");
}
