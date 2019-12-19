pub fn patterns() {
    enum Animal {
        Dog(String),
        Cat(String),
    }
    
    impl Animal {
        fn speak(&self) -> String {
            match self {
                Animal::Dog(d) => format!("Dog says: {}", d),
                Animal::Cat(c) => format!("Cat says: {}", c),
            }
        }
    }
    
    let dog = Animal::Dog(String::from("arf arf growl!"));
    let cat = Animal::Cat(String::from("hey there"));
    
    assert_eq!(dog.speak(), String::from("Dog says: arf arf growl!"));
    
    if let Animal::Cat(c) = cat {
        assert_eq!(c, __);
    }

    fn number_report((numbers, adder): (Vec<u32>, u32)) -> (u32, usize) {
        let mut sum = 0;
        
        for n in numbers.iter() {
            sum += n + adder
        }
        (sum, numbers.len())
    }
    
    let numbers = vec![2, 4, 6, 8, 10];
    
    let args = (numbers, 5);
    
    let (summation, length) = number_report(args);
    assert_eq!(summation, __);
    assert_eq!(length, __);
}
