enum Student {
    Freshman,
    Sophomore,
    Junior,
    Senior,
    None
}

impl Student {
    fn find_student(&self) -> String {
        match self {
            Student::Freshman => String::from("Freshman"),
            Student::Sophomore => String::from("Sophomore"),
            Student::Junior => String::from("Junior"),
            Student::Senior => String::from("Senior"),
            _ => String::from("no student type found")
        }
    }
}

fn multiply_by_five(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 5)
    }
}

fn if_let_example(student: &Student) -> String {
    if let Student::Senior = student {
        "This is a Senior Student".to_string()
    } else {
        "Did not match here".to_string()
    }
}

fn main() {
    let student = Student::Freshman;
    println!("{}", student.find_student());

    let no_student = Student::None;
    println!("{}", no_student.find_student());

    let student = Student::Sophomore;
    println!("{}", student.find_student());

    let student = Student::Junior;
    println!("{}", student.find_student());

    let student = Student::Senior;
    println!("{}", student.find_student());

    let nine = Some(9);
    let fourty_five = multiply_by_five(nine);
    println!("the some case is {:?}", fourty_five);

    let none = multiply_by_five(None);
    println!("the none case is {:?}", none);

    println!("{}", if_let_example(&Student::Senior));
}
