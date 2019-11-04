fn main() {
    let language = "rust";
    let workshop = "workshop";

    let out = format!("{} {} {}", String::from("hello"), language, workshop);
    println!("{}", out);

    output();

    output2();
}


fn output() {
    let word = String::from("no newline!");
    print!("{}", word);
}

fn output2() {
    let err = String::from("an error logged!");
    eprint!("{}", err);
}
