fn main() {
    // For Loop example
    let mut sum = 0;
    for x in 0..10 {
        sum += x;
        println!("for loop sum is {}", sum);
    }

    println!("");

    // while loop example
    let mut sum = 0;
    let mut counter = 0;
    while counter < 10 {
        sum += counter;
        println!("while loop sum is {}", sum);
        counter += 1;
    }

    // loop example
    let mut sentinel = -1;
    loop {
        println!("I am an infinite loop");
        sentinel += 1;
        if sentinel == 5 {
            println!("I have broken out of loop by condition");
            break;
        }
    }
}
