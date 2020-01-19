# Rust Workshop - Generics

## Sections:

* [Concept of Generics](#concept-of-generics)
* [Generic Function](#generic-function)
* [Implementations](#implementations)
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

```rust
struct Student<'a, T, U> {
    scores: &'a [T],
    name: U,
}

impl <T, U> Student<'_, T, U> {
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

## Bounds

content

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
