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

    // rustlang has `if`, `else if`, `else` branch keywords like other
    // languages
    let x = 7;
    let word;
    if x < -1 {
        word = "one".to_string();
    } else if x == 5 {
        word = "five".to_string();
    } else {
        word = "no matches".to_string();
    }
    assert!(word == "not sure");

    // rust has a `loop` keyword to indicate an infinite loop
    // similar to what you would do with a while(true) {} loop
    // in c-type languages
    
    let mut counter = 1;
    let mut sentence;

    // infinite loop
    // there is a keyword to break out of infinite loops
    loop {
        counter += 2;
        if counter == 3 {
            sentence = String::from("I am 3");
            __;
        } else if counter % 2 == 0 {
            sentence = String::from("divisible by even number");
        } else {
            sentence = String::from("not sure here");
        }
    }
    assert_eq!(sentence, __);

    let mut count = 1;
    let passphrase;
    while count < 101 {
        if count % 15 == 0 {
            passphrase = "fizzbuzz";
        } else if count % 3 == 0 {
            passphrase = "fizz";
        } else if count % 5 == 0 {
            passphrase = "buzz";
        } else {
            passphrase = "";
        }
        count += 1;
    }
    assert_eq!(passphrase, __);

    // rust also has a for loop except it is not like traditional for loop
    // for (int i = 0; i < SOME_VAR; i++) {} 
    // the above is not a valid for loop in rust
    let n;
    for num in 2..200 {
        if num % 2 == 0 {
            print!("what am i");
            n = 0;
        } else if num % 3 == 0 {
            print!("do you know?");
            n = 1;
        } else {
            print!("{}", num);
            n = -1;
        }
    }
    assert!(n == __);
}

#[test]
fn test_controlflow() {
    control_flow();
}
