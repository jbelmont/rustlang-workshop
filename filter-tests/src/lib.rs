#[allow(dead_code)]
fn average(numbers: Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for n in numbers.iter() {
        sum += n;
    }
    (sum / numbers.len() as i32) as f32
}

#[allow(dead_code)]
fn summation(numbers: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for n in numbers.iter() {
        sum += n;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let actual = average(vec![5, 6, 7, 8, 9, 10]);
        assert_eq!(actual, 7.0);
    }

    #[test]
    fn test_summation() {
        let actual = summation(vec![1, 2, 3, 4, 5]);
        assert_eq!(actual, 15);
    }

    #[test]
    #[ignore]
    fn test_too_expensive() {
        // I take months to compute
    }
}
