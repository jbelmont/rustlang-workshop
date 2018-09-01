fn main() {
    // Using the String::from function to create a String from a string literal
    let mut example1 = String::from("I am a string literal");

    // Updating a string with push_str method
    // push_str appends a given string slice onto the end of this String
    example1.push_str(" and don't you forget it");

    println!("{}", example1);

    // Updating a string with push method
    // push appends the given char to the end of this String, so just one character
    example1.push('!');

    // Concatenation with the + operator
    let first_name = String::from("Jean-Marcel ");
    let last_name = String::from("Belmont");
    let full_name = first_name + &last_name;
    println!("{}", full_name);

    let users = String::from("users");
    let user_id = String::from("bcd12345");
    let url = format!("/v3/{}/{}", users, user_id);
    println!("{}", url);

    // Get string length with len()
    println!("{}", url.len());

    // Using string slices
    // let us get the first name from the full_name variable
    // we can omit the first index of 0
    let index_fname = &full_name[..11]; // not necessary to do &full_name[0..11];
    println!("{}", index_fname);

    // iterate over string by using the chars method
    for c in index_fname.chars() {
        println!("{}", c);
    }

    // this comes into play and is more useful with Unicode scalar values
    let chinese_characters = "香港的經濟素以自由貿易".to_string();
    for c in chinese_characters.chars() {
        println!("{}", c);
    }
    // using string slices won't work as expected
    // println!("{}", &chinese_characters[..2]);
    // uncomment the line and it will panic
    println!("{}", &chinese_characters[0..3]);
}
