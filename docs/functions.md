# Rust Workshop - Functions

## Sections:

* [Functions Playground](#functions-playground)
* [Function Rust Specification](#function-rust-specification)
* [Recursion](#recursion)
* [Multiple Return Types](#multiple-return-types)
* [const functions](#const-functions)
* [functions koan](#functions-koan)
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

*Notice that we had to put parenthesis around the assignment in the `compute` function call since we returned a tuple.*

#### const functions

[const functions specification](https://doc.rust-lang.org/reference/items/functions.html#const-functions)

> Functions qualified with the const keyword are const functions. Const functions can be called from within const contexts. When called from a const context, the function is interpreted by the compiler at compile time. The interpretation happens in the environment of the compilation target and not the host. So usize is 32 bits if you are compiling against a 32 bit system,irrelevant of whether you are building on a 64 bit or a 32 bit system.

[const functions playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=09b23233bb92e44373e7b48894cbdd5e)

```rust
const fn triple(x: i32) -> i32 {
    x * 3
}

const THREE: i32 = 3;
const NINE: i32 = triple(THREE);

fn main() {
   assert_eq!(3, THREE);
   assert_eq!(9, NINE);
}
```

## functions koan

[functions koan](../koans/src/functions.rs)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Control Flow](./control_flow.md) | [Data Types](./data_types.md) →
