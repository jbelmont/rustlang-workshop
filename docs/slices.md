# Rust Workshop - Slices

## Sections:

* [Slices in Rust](#slices-in-rust)
    * [Types that slices can hold](#types-that-slices-can-hold)
    * [Slices versus Arrays](#slices-versus-arrays)
* [Slices in Rust Example](#slices-in-rust-example)
* [Slices playground](#slices-playground)
* [Practice Exercise -> Slices](#practice-exercise-\-->slices)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Slices in Rust

Slices in Rust are similar to what are called Arrays in other programming languages.

#### Types that slices can hold

```rust
fn main() {
    let mut word: String = String::from("");
    let strs: [String; 3] = [
        String::from("Hello"), 
        String::from(" "), 
        String::from("There")
    ];
    for str in strs.iter() {
        word.push_str(str);
    }
    assert_eq!(word, "Hello There");
    
    let primes: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];
    assert_eq!(primes.len(), 7);
    
    let truth_table = [true, true, false, false, true, false];
    assert_eq!(truth_table[4..5][0], true);
    
    // next line causes panic
    // let random = vec![1, "str", false];
}
```

Notice that each slice/array holds the same type in Rust.

Slices can hold different values but you cannot have different types in a single slice, they must be the same!

#### Slices versus Arrays

Arrays sizes are known in compile time while slices are not.

## Slices in Rust Example

```rust
fn main() {
    // a fixed size array is defined here
    let arr: [i32; 3] = [2, 3, 5];
    
    // initialize elements in one go
    let fixed_nums: [i32; 100] = [0; 100];
    
    // 100 values of zero will print here
    for n in fixed_nums.iter() {
        print!("{}", n);
    }
    println!("");

    // a set of prime numbers
    let first_five_primes: [i32; 5] = [2, 3, 5, 7, 11];
    
    // indexing in rust
    println!(
        "1st: {}, Len() - 1: {}", 
        first_five_primes[0], 
        first_five_primes[first_five_primes.len() - 1],
    );
    
    // a slice section of an array
    let first_three_primes = &first_five_primes[..2];
    println!(
        "Last arg is: {}", 
        &first_three_primes[first_three_primes.len() - 1]
    );
    
    // slice of arr
    println!("one element of arr borrowing: {:?}", &arr[..1]);
}

// Arrays can be borrowed as a slice in rust
fn summation_of_slices(numbers: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for n in numbers.iter() {
        sum += n;
    }
    sum
}

#[test]
fn should_compute_summation() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let actual = summation_of_slices(&numbers);
    
    let expected = 55;
    assert_eq!(actual, expected);
}
```

Notice that we can do indexing of slice types in rust like other languages but we can also get a slice of arrays like the first 3 primes demonstrates.

## Slices playground

[Playground 1](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=94def6401cdb324df30fcdcd400f55b0)

[Playground 2](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=78b380921537f23988937af1eae6da03)

## Practice Exercise -> Slices

[Practice Exercise -> Slices](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=03852d8d9f1f4c5629b0aa849170129d)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Borrowing](./borrowing.md) | [Structs](./structs.md) →
