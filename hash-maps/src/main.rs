use std::collections::HashMap;

fn main() {
    let mut fruits = HashMap::new();

    fruits.insert(String::from("apples"), 5);
    fruits.insert(String::from("oranges"), 10);
    println!("{:?}", fruits);

    let fruits = vec![String::from("strawberries"), String::from("mangoes")];
    let count = vec![25, 50];
    let fruit_count: HashMap<_,_> = fruits.iter().zip(count.iter()).collect();
    println!("{:?}", fruit_count);

    let fruit = String::from("blueberries");
    let count = 3;
    let mut fruits = HashMap::new();
    fruits.insert(fruit, count);
    // cannot reference fruit nor count now since they have been moved

    // accessing values in hash maps using get method
    let map = vec![String::from("strawberries"), String::from("mangoes")];
    let count = vec![25, 50];
    let fruits: HashMap<_,_> = map.iter().zip(count.iter()).collect();

    let number_of_mangoes = fruits.get(&"mangoes".to_string());
    println!("{:?}", number_of_mangoes);

    // iterate over hash maps
    for (key, val) in &fruits {
        println!("{}: {}", key, val);
    }

    // Updating hash maps via overwrite
    let mut fruits = HashMap::new();
    fruits.insert(String::from("apples"), 5);
    fruits.insert(String::from("oranges"), 10);
    fruits.insert(String::from("apples"), 15);
    let number_of_apples = fruits.get(&String::from("apples"));
    // apples was overridden by second insert call
    println!("{:?}", number_of_apples);

    // Only update if the fruit key has no value by using entry method
    let mut fruits = HashMap::new();
    fruits.insert(String::from("apples"), 5);
    fruits.insert(String::from("oranges"), 10);
    fruits.entry(String::from("blackberries")).or_insert(25);
    println!("{:?}", fruits);

    // updating hash map based on older value
    let strange_text = "some crazy some thing crazy here";

    let mut map = HashMap::new();
    // here we split on each whitespace character or in this case the " " empty space
    // we then use entry and or_insert to update occurrence if the key does not exist
    for word in strange_text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
