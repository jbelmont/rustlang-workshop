fn smallest(list: &[i32]) -> i32 {
    let mut smallest = list[0];

    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = smallest(&number_list);
    println!("The smallest number is {}", result);
   assert_eq!(result, 25);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
   assert_eq!(result, 6000);
}
