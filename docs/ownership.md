# Rust Workshop - Ownership

## Sections:

* [Rust Ownership](#rust-ownership)
    * [Ownership Rules](#ownership-rules)
    * [Ownership move example](#ownership-move-example)
    * [Rust Ownership playground example](#rust-ownership-playground-example)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Rust Ownership

Ownership in Rust deals with the what variables or values have ownership over what.

In Rust it is your job to make sure that ownership is given and used properly although the Rust compiler is very good at letting you know when you violate the ownership rules.

#### Ownership Rules

* Each value in Rust has a variable that’s called its owner.

* There can only be one owner at a time.

* When the owner goes out of scope, the value will be dropped.

#### Ownership move example

```rust
struct Soldier {
    name: String,
    age: i32,
}

fn print_soldier(soldier: Soldier) -> String {
    let str = format!("Hello {} and you say your age is {}", soldier.name, soldier.age);
    str
}

fn main() {
    let rambo = Soldier {
        name: String::from("Arnold"),
        age: 35,
    };
    
    print_soldier(rambo);
    // print_soldier(rambo);
}
```

If you uncomment the 2nd call to print_soldier function call you will get an error saying 
value has been moved.

*This is saying that ownership has been transferred to the print_soldier function for the rambo struct variable.*

If we use a concept called a reference we can get past the error this error.

#### Rust ownership playground example

[Ownership Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d371beb0ff897e7116b401fbf55efe73)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Comments](./comments.md) | [References](./references.md) →
