fn main() {
    // regular variable
    let x = 5;
    println!("x is equal to {}", x);

    // const value must annotate type
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS);

    let x = 10;
    println!("x is shadowing the other x and is now {}", x);
}
