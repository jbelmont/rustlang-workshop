# Rust Workshop - Methods

## Sections:

* [Method definition](#method-definition)
* [Method example](#method-example)
* [Difference between method and associated function](#difference-between-method-and-associated-function)
* [Method Playground](#method-playground)
* [Practice Exercise -> Methods](#practice-exercise-\-->methods)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Method definition

Methods are like functions in that they are declared using the `fn` keyword and can return values like functions.

Methods are different in that they are defined within the context of a struct, enum, or trait object.

*A method always has `self` as the first parameter of its definition.*

## Method example

```rust
struct Soldier{
    name: String,
    age: u32,
    rank: String,
    years_of_service: u32,
}

impl Soldier {
    fn message(&self) -> String {
        format!(
            "Name: {},\nAge: {},\nRank: {},\nYears of Service: {}",
            self.name,
            self.age,
            self.rank,
            self.years_of_service,
        )
    }
    
    // fictious pay amount
    fn basic_pay_rate(&self) -> u32 {
        self.years_of_service * 4000
    }
    
    fn new(name: String, age: u32, rank: String, years_of_service: u32) -> Soldier {
        Soldier {
            name,
            age,
            rank,
            years_of_service,
        }
    }
}
```

## Difference between method and associated function

Notice that in the example above we defined a couple of methods and an associated function.

Associated methods are called using the `::` syntax and notice that we didn't use the `.` operator to call *new* method.

## Method Playground

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f22a750b80005e7b0b04e22a8ae81182)

## Practice Exercise -> Methods

[Practice Exercise -> Methods](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cd01830d2f3a93ed4bb3b2580f8072c8)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Structs](./structs.md) | [Enums](./enums.md) →
