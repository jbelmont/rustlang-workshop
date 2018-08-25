#[derive(Debug)]
struct Soldier {
    name: String,
    rank: String,
    age: u32,
    active_duty: bool,
}

impl Soldier {
    fn print_info(&self) -> String {
        let yes_or_no_active_duty = if self.active_duty {
            "Yes"
        } else {
            "No"
        };
        let info = format!(
            "Name is {}\nRank is {}\nAge is {}\nIs Active Duty? {}",
            self.name,
            self.rank,
            self.age,
            yes_or_no_active_duty,
        );
        info
    }
    fn ready_for_active_duty(&self) -> bool {
        self.age > 21 && self.active_duty
    }
    fn update_soldier_attrs(soldier: Soldier) -> Soldier {
        Soldier {
            ..soldier
        }
    }
}

fn main() {
    let rambo = Soldier {
        name: String::from("John Rambo"),
        rank: String::from("SGT"),
        age: 21,
        active_duty: true,
    };
    println!("{}", rambo.print_info());
    rambo.ready_for_active_duty();
    let update_rambo = Soldier::update_soldier_attrs(
        Soldier {
            rank: String::from("SFC"),
            age: 26,
            ..rambo
        }
    );
    println!("{:?}", update_rambo);
}
