# Rust Workshop - Borrowing

## Sections:

* [Borrowing in Rust](#borrowing-in-rust)
* [Borrowing playground](#borrowing-playground)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Borrowing in Rust

We already mentioned the concept of borrowing in the previous section.

#### Pass by Value vs Pass by Reference

When you pass by value you are copying the value of the variable while when you pass by reference you are giving a reference to the variable or a pointer to the variable.

In Rust there is a borrower checker and the borrower checker ensures that when you pass a value to a function its ownership is transferred thereby preventing 2 different copies or 2 pointers to the same variable and violating the borrow checker terms.

```rust
fn mutate_value_by_reference(number: &mut i32) -> &i32 {
    *number = 7;
    number
}

fn main() {
    let mut number: i32 = 1;
    println!("{}", number);
    mutate_value_by_reference(&mut number);
    println!("{}", number);
}
```

Notice here how we passed a reference to number to the mutate_value_by_reference function call.

Since we passed a reference to the mutable variable it changed the value of number in the mutate_value_by_reference function.

## Borrowing playground

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c7bcbeeb4a67d8de47d70344155fd0d4)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [References](./references.md) | [Slices](./slices.md) →
