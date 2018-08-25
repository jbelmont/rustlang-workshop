#[derive(Debug)]
struct Soldier {
    name: String,
    rank: String,
    age: u32,
    active_duty: bool,
}

fn build_soldier(name: String, rank: String, age: u32) -> Soldier {
    Soldier {
        name,
        rank,
        age,
        active_duty: true,
    }
}

#[derive(Debug)]
struct Compass(String, String);

#[derive(Debug)]
struct Square {
    size: u32,
}

fn area(square: &Square) -> u32 {
    square.size * square.size
}

fn main() {
    // Struct examples
    let rambo = build_soldier(
        String::from("John Rambo"), 
        String::from("SSG"), 
        27
    );
    println!("{:#?}", rambo);

    let cage = Soldier {
        name: String::from("Luke Cage"),
        rank: String::from("SGT"),
        age: 28,
        active_duty: true,
    };
    println!("{:#?}", cage);

    // tuple struct
    let direction = Compass(
        String::from("North"), 
        String::from("West"),
    );
    println!("{:?}", direction);

    println!("{:?}", area(
        &Square{
            size: 5,
        }
    ));
}
