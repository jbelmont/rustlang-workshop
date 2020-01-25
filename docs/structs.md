# Rust Workshop - Structs

## Sections:

* [Struct Definition](#struct-definition)
* [Basic Struct Example](#struct-example)
* [Tuple Struct](#tuple-struct)
* [Unit Struct](#unit-struct)
* [Struct Update Syntax](#struct-update-syntax)
* [Struct Field init shorthand](#struct-field-init-shorthand)
* [Struct Playground](#struct-playground)
* [Practice Exercise -> Structs](#practice-exercise-\-->structs)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Struct Definition

[Struct Type](https://doc.rust-lang.org/reference/types/struct.html)

> A struct type is a heterogeneous product of other types, called the fields of the type.

> New instances of a struct can be constructed with a struct expression.

> The memory layout of a struct is undefined by default to allow for compiler optimizations like field reordering, but it can be fixed with the repr attribute. In either case, fields may be given in any order in a corresponding struct expression; the resulting struct value will always have the same memory layout.

## Basic Struct Example

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        gender: String,
    }
    
    let sandy = Person {
        name: String::from("Sandy Belmont"),
        age: 71,
        gender: String::from("Female"),
    };
    println!("{:?}", sandy);
}
```

## Tuple Struct

[Tuple Struct specification](https://doc.rust-lang.org/reference/expressions/struct-expr.html#tuple-struct-expression)

```rust
struct CartesianCoordinates(i32, i32);
    
let point = CartesianCoordinates(1, 4);
```

Notice that here we defined a struct that is enclosed in parenthesis making this a tuple struct.

## Unit Struct

[Unit Struct specification](https://doc.rust-lang.org/reference/expressions/struct-expr.html#unit-struct-expression)

> A unit struct expression is just the path to a unit struct item. This refers to the unit struct's implicit constant of its value. The unit struct value can also be constructed with a fieldless struct expression. For example:

```rust
struct Dude;

let d = Dude;
```

## Struct Update Syntax

Rust has a nice mechanism for you to create instances of another struct and reuse values.

It is using the `..` feature

Let us say that you had a struct named `Soldier` like this:

```rust
fn main() {
    #[derive(Clone)]
    struct Soldier {
        name: String,
        age: u32,
        rank: String,
        years_of_service: u32,
    }
    
    let soldier1 = Soldier {
        name: String::from("John Rambo"),
        age: 32,
        rank: String::from("SFC"),
        years_of_service: 12,
    };
    
    let soldier2 = Soldier {
        name: String::from("Chuck Norris"),
        ..soldier1.clone()
    };
    
    assert_eq!(soldier1.name, String::from("John Rambo"));
    assert_eq!(soldier2.name, String::from("Chuck Norris"));
    assert_eq!(soldier1.rank, soldier2.rank);
    assert_eq!(soldier1.years_of_service, soldier2.years_of_service);
    assert_eq!(soldier1.age, soldier2.age);
}
```

Notice that we needed to clone soldier1 as its value will be moved and no longer valid for us to use.

We added a new attribute called Clone for the struct value

## Struct Field init shorthand

[Struct field init shorthand specification](https://doc.rust-lang.org/reference/expressions/struct-expr.html#struct-field-init-shorthand)

> When initializing a data structure (struct, enum, union) with named (but not numbered) fields, it is allowed to write fieldname as a shorthand for fieldname: fieldname. This allows a compact syntax with less duplication.

```rust
struct Soldier {
    name: String,
    age: u32,
    rank: String,
    years_of_service: u32,
}

let name = String::from("John Rambo");
let age = 32;
let rank = String::from("SFC");
let years_of_service = 12;

let rambo = Soldier {
    name: name,
    age: age,
    rank: rank,
    years_of_service: years_of_service,
};

let rambo2 = Soldier {
    name,
    age,
    rank,
    years_of_service,
};
```

Notice that didn't add the value in rambo2 as it is the same name as the key.

## Struct Playground

[Playground 1](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ac48d773d3fe9bdd4e22f348b4beb581)

[Playground 2](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=38662be012b9ebf26959c168f3e016cf)

[Playground 3](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4bfa5a6c3e1c920452a13e9d2425c58b)

## Practice Exercise -> Structs

[Practice Exercise -> Structs](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e0ea71a30b60f13b170b216102a73fa8)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Slices](./slices.md) | [Methods](./methods.md) →
