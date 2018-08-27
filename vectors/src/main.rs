// average function using vectors
fn average(numbers: Vec<f32>) -> f32 {
    let numbers_length = numbers.len();
    let mut sum: f32 = 0.0;
    for i in numbers {
        sum += i;
    }
    return sum / numbers_length as f32;
}

fn main() {
    let numbers = vec![2.5,3.9,2.75,1.5,5.75];
    let avg = average(numbers);
    println!("{}", avg);

    // Declare vectory using Generic Syntax
    let mut v: Vec<f32> = Vec::new();
    v.push(1.5);
    println!("{:?}", v);

    let popped = v.pop();
    println!("{:?}", popped);

    let mut v = vec![1, 2, 3, 4, 5];
    v[1] += 5;
    println!("{:?}", v[1]);
}
