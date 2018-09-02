fn main() {
    // this causes a panic if you uncomment the line
    // panic!("I am going to panic here");

    let numbers = vec![1, 2, 3, 4, 5];
    // another panic here since there is only 5 numbers here
    numbers[50];
}
