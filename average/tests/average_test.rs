extern crate average;

#[test]
fn test_average() {
    let actual = average::average(vec![5, 6, 7, 8, 9, 10]);
    println!("actual is {}", actual);
    assert_eq!(actual, 7.0);
}
