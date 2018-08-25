fn main() {
    // if, else if, else example
    let a = 5;
    let b = 6;

    if a < b {
        println!("{} is less than {}", a, b);
    } else if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is equal to {}", a, b);
    }

    // if let statement example
    let number = 5;
    let condition = if number < 5 {
        "too small"
    } else {
        "just right"
    };
    println!("{}", condition);

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
