# Rust Workshop - Functions

## Sections:

* [Functions Playground](#functions-playground)
* [Function Rust Specification](#function-rust-specification)
* [Recursion](#recursion)
* [Multiple Return Types](#multiple-return-types)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Functions Playground

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4e79050b978861a350b0616a6a4b7285)

## Function Rust Specification

[Function Rust Specification](https://doc.rust-lang.org/reference/items/functions.html)

* A function lets us wrap up a sequence of statements as a unit that can be called from elsewhere in a program

* Sometimes a function is reused repeatedly in order to promote dry code

To declare a function in *rust* you use the `fn` keyword

> Functions may declare a set of input variables as parameters, through which the caller passes arguments into the function, and the output type of the value the function will return to its caller on completion.

#### Example Rust Function

```rust
fn multiplier(x: i32, multiplier: i32) -> i32 {
    x * multiplier
}
```

## Recursion

Functions may be recursive, that is, they may call themselves, either directly or indirectly.

```rust
fn fibonacci(number: i32) -> i32 {
    if number <= 1 {
        return number
    }
    fibonacci(number-1) + fibonacci(number-2)
}

#[test]
fn it_should_return_fibonacci_sequence() {
    let actual = fibonacci(7);
    let expected = 13;
    assert!(actual == expected);
}
```

The example above is called the Fibonacci sequence and we are using recursion in rust.

*Be warned though that Rust does not support Tail Call Recursion.*

## Multiple Return Types

Functions in Rust can return multiple values, it is done with tuples.

```rust
fn compute(x: i32, multiplier: i32) -> (i32, i32) {
    (x * multiplier, x + multiplier)
}

#[test]
fn should_compute() {
    let (x, y) = compute(2, 3);
    assert!(x == 6 && y == 5);
}
```

*Notice that we had to put parenthesis around the assignment in the `compute` function call.*

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Control Flow](./control_flow.md) | [Data Types](./data_types.md) →
