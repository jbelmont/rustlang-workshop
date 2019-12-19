# Rust Workshop - Patterns

## Sections:

* [Different Pattern Types in Rust](#different-pattern-types-in-rust)
* [match args](#match-args)
* [if let](#if-let)
* [while let](while-let)
* [for loop](#for-loop)
* [let statement](#let-statement)
* [function parameter](#function-parameter)
* [Patterns koan](#patterns-koan)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Different Pattern Types in Rust

Patterns can be used in each of the following areas in Rust:

* `match args`

* `if let`

* `while let`

* `for loop`

* `let statement`

* `function parameter`

## match args

We covered the match statement earlier in the enum section of the workshop.

match arms are a powerful pattern matching concept in rust but they can be used in more circumstances than just enums they can be used with numbers and in different manners.

#### match arm with number

```rust
let number: u32 = 9;
    
let computation: u32 = 1;
let number = match number {
    1 => computation * 2,
    2 | 3 => computation * 2 * 3,
    3 ..= 7 => computation * 2 * 3 * 4 * 5 * 6 * 7,
    n @ 8 ..= 9 => computation * 2 * 3 * 4 * 5 * 6 * 7 * n,
    _ => 0,
};
// 1 * 2 * 3 * 4 * 5 * 6 * 7 * 9 => 45360
assert_eq!(number, 45360);
```

Notice that we assigned `number` a value from the match expresssion evaluation

[match arm playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=49ac031c2fd4993d0099442a0f74bf0a)

## if let

```rust
enum Animal {
    Dog(String),
    Cat(String),
}

impl Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Dog(d) => format!("Dog says: {}", d),
            Animal::Cat(c) => format!("Cat says: {}", c),
        }
    }
}

fn main() {
    let dog = Animal::Dog(String::from("arf arf growl!"));
    let cat = Animal::Cat(String::from("meow meow purr!"));
    
    assert_eq!(dog.speak(), String::from("Dog says: arf arf growl!"));
    
    if let Animal::Cat(c) = cat {
        assert_eq!(c, "meow meow purr!");
    }
}
```

Notice that in the code example above we used `if let` to match the `Animal::Cat(String)` enum.

[if let playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ad84cc3f506447310e4618c3d79ba0c3)

## while let

content

## for loop

content

## let statement

content

## function parameter

content

## Patterns koan

Content

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Enums](./enums.md) | [Modules](./modules.md) →
