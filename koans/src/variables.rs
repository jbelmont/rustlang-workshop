pub fn variables() {
    // variables of scalar type
    let x = 5; // i32
    let y = 6; // i32
    assert_eq!(x, y);

    let str1 = String::from("Hello"); // String
    let str2 = String::from("Nope"); // String
    assert_eq!(str1, str2);

    // Rust has `const` keyword which is always immutable
    // similar to `let` variables but with differences
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html?highlight=const#differences-between-variables-and-constants
    const MAX_SIZE: i32 = 50;
    assert!(MAX_SIZE > 5);
}
