fn multiples_of_five(number: i32) -> i32 {
    number * 5
}

fn print_message(message: &str) -> String {
    message.to_owned() + " is the message"
}

fn print_another_message(message: String) -> String {
    message + " another message to show"
}

fn average(numbers: &[i32]) -> f32 {
    let mut sum = 0;
    for num in numbers.iter() {
        sum += num;
    }
    // must divide by same type
    return sum as f32 / numbers.len() as f32;
}

fn main() {
    println!("multiples_of_five prints multiples of five such as {}", multiples_of_five(7));

    let message = print_message("aqua");
    println!("{}", message);

    let another_message = print_another_message("dude man".to_string());
    println!("{}", another_message);

    println!("{}", average(&[5, 6, 7, 8, 9]));
}
