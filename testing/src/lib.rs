pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Square {
    width: u32,
}

impl Square {
    pub fn area(&self) -> u32 {
        self.width * self.width
    }
}

pub fn average(numbers: &[i32]) -> f32 {
    let mut sum = 0;
    for num in numbers.iter() {
        sum += num;
    }
    return sum as f32 / numbers.len() as f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_of_rectangle() {
        let rect = Rectangle {
            width: 5.0,
            height: 4.0
        };
        // assert_eq macro assert that the left and right are equal
        assert_eq!(rect.area(), 20.0);
    }

    #[test]
    fn area_of_square() {
        let square = Square {
            width: 5,
        };
        // assert_ne macro asserts that the area should not be equal
        assert_ne!(square.area(), 20);
    }

    #[test]
    fn average_of_integral_values() {
        let numbers: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(average(&numbers), 3.0);
    }
}
