enum Student {
    Freshman,
    Sophomore,
    Junior,
    Senior,
}

impl Student {
    fn find_student(&self) -> String {
        match self {
            Student::Freshman => String::from("Freshman"),
            Student::Sophomore => String::from("Sophomore"),
            Student::Junior => String::from("Junior"),
            Student::Senior => String::from("Senior"),
        }
    }
}

fn main() {
    let student = Student::Freshman;
    println!("{}", student.find_student());
}
