fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn smallest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];

    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

fn main() {
    let numbers = vec![35, 50, 25, 75, 95];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let characters = vec!['a', 'b', 'c', 'd', 'e'];
    let result = smallest(&characters);
    println!("The smallest character is {}", result);
}
