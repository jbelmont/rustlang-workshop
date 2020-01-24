/// This function computes an average for a 
// vector of i32 data types.
///
/// # Examples
/// 
/// ```rust
/// let result = average::average(vec![1,2,3,4,5]);
/// assert_eq!(result, 3.0);
/// ```
#[allow(dead_code)]
pub fn average(numbers: Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for n in numbers.iter() {
        sum += n;
    }
    (sum / numbers.len() as i32) as f32
}
