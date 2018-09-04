extern crate testing;

#[test]
fn it_computes_average() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(testing::average(&numbers), 3.0);
}
