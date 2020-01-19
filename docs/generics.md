# Rust Workshop - Generics

## Sections:

* [Concept of Generics](#concept-of-generics)
* [Generic Function](#generic-function)
* [Implementations](#implementations)
* [Traits](#traits)
* [Bounds](#bounds)
* [Where Clauses](#where-clauses)
* [Associated Items](#associated-items)
* [Phantom Types](#phantom-types)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Concept of Generics

Much like when you want to make a general analogy about a concept you can utilize Generics in Rust to generalize your Rust Programs.

We could specify a function and have functions that behave in the same exact way but that would be unnecessary if we can utilize generics.

You will typically see generics denoted by angle brackets and one letter Capitalized alphabet such as:

```rust
<T> => List of <T>
<E> => List of <E>
```

## Generic Function

A function can be considered generic if it is preceded by a `<T>` before its parameters specified in `(...)` and its name:

```rust
// generic soldier
struct Soldier<T>(T);

fn i_am_generic<T>(s: Soldier<T>) {}
```

Here we have a struct named `Soldier` that takes a generic of T and our function named `i_am_generic` takes a generic argument of T and specifies an argument of `Soldier<T>`

## Implementations

In order for implementation to be generic the `<T>` must precede the type like this:

```rust
struct Student<'a, T, U> {
    scores: &'a [T],
    name: U,
}

impl<T, U> Student<'_, T, U> {
    fn get_scores(&self) -> &[T] {
        self.scores
    }
}

fn main() {
    let student = Student {
        scores: &vec![98, 99, 88, 85, 100],
        name: String::from("Jean-Claude Van Damme"),
    };
    
    let mut summation = 0;
    for score in student.get_scores().iter() {
        summation += score;
    }
    assert_eq!(summation / student.get_scores().len(), 94);
    assert_eq!(student.name, String::from("Jean-Claude Van Damme"));
}
```

Notice that here we have `impl<T, U>` defined before the type declaration for the `Student` struct type.

## Bounds

> Trait and lifetime bounds provide a way for generic items to restrict which types and lifetimes are used as their parameters. Bounds can be provided on any type in a where clause. There are also shorter forms for certain common cases:

[Trait Bounds specification](https://doc.rust-lang.org/reference/trait-bounds.html)

#### Traits Recap

*We previously mentioned Traits earlier in the workshop on the [Data Types -> Trait Types Section](./data_types.md#trait-types) so please check it out if you skipped to this section and do not know what a Trait is..*

```rust
struct Number {
    num1: i32,
    num2: i32,
}

trait Calculator {
    fn adder(&self) -> i32;
    fn subtracter(&self) -> i32;
}

impl Calculator for Number {
    fn adder(&self) -> i32 {
        self.num1 + self.num2
    }
    fn subtracter(&self) -> i32 {
        self.num1 - self.num2
    }
}

fn main() {
    let number = Number {
        num1: 5,
        num2: 4,
    };
    assert_eq!(number.adder(), 9);
    assert_eq!(number.subtracter(), 1);
}
```

[traits example recap playground]()https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e7fb35403f6f14529eb62303ec81c8e9

## Where Clauses

content

## Associated Items

content

## Phantom Types

content

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Error Handling](./error_handling.md) | [Lifetimes](./lifetimes.md) →
