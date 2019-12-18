pub fn slices() {
    let strings = vec!["One", "Two", "Three"];

    assert_eq!(strings[0], __);

    // write an assertion that get the first and second element using slices
    let phrase: [String; 5] = [
        String::from("I"),
        String::from("am"),
        String::from("a"),
        String::from("little"),
        String::from("phrase"),
    ];

    // concatenate the strings from the phrase slice
    let mut words = String::from("");
    for w in phrase.iter() {
        words.push_str(w);
    }
    // what should be the output here
    assert_eq!(words, "");
}
