# Rust Workshop - Variables

## Sections:

* [Variables](#variables)
    * [Lexical Scope](#lexical-scope)
    * [Immutable variables](#mutable-variables)
    * [Mutable variables](#mutable-variables)
    * [Constants](#constants)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## variables 

Variables in rust are much like variables in other programming languages.

#### Lexical Scope

Variables will have lexical scope based on where the variable is defined so for example if you define a variable in an if block in rustlang then that variable will only have scope within that `if` block while a variable defined at the top of a function will have scope anywhere while it is in that respective function.

#### Immutable variables 

In rust unless you preface a variable with the `mut` it is considered immutable so you cannot reassign a variable.

For example:

```rust
let a = 3;

a = 4;
```

This will throw a compiler error like this:

error[E0384]: cannot assign twice to immutable variable `a`
 --> src/main.rs:3:5
  |
2 |     let a = 3;
  |         -
  |         |
  |         first assignment to `a`
  |         help: make this binding mutable: `mut a`
3 |     a = 4;
  |     ^^^^^ cannot assign twice to immutable variable

[Compiler error for immutable variable](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3c14f565c306ce93eb86876c1b1e31e5)

#### Mutable variables

If you want to make a variable mutable then you need to use the `mut` keyword so if we rewrote the program like this:

```rust
let mut a = 3;
a = 4;
```

Then we will not get error like we did before.

#### Constants

[const playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=40b213461f7448931e9ec9429b651993)

```rust
fn main() {
    const num: i32 = 5;
    num = 6;
}
```

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Hello World](./hello_world.md) | [stdin, stdout, and stderr](./stdin-stdout-stderr.md) →
