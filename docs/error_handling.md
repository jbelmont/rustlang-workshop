# Rust Workshop - Error Handling

## Sections:

* [panic! macro](#panic\!-macro)
* [Option enum](#option-enum)
* [? operator](#\?-operator)
* [Flow Control with Combinators](#flow-control-with-combinators)
* [Result enum](#result-enum)
* [Dealing with Multiple Error Types](#dealing-with-multiple-error-types)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## panic! macro

The `panic!` macro should be used if your program reaches an unrecoverable state.

#### panic! macro options

Here is the rules to the macro:

```rust
macro_rules! panic {
    () => { ... };
    ($msg:expr) => { ... };
    ($msg:expr,) => { ... };
    ($fmt:expr, $($arg:tt)+) => { ... };
}
```

The `panic!` macro is used in the `unwrap` method for the `Option` and `Result` enums.

```rust
fn bad_number(n: u32) {
    if n == 22 {
        panic!("Who likes the number 22!");
    }
    println!("{}", "Nice Number!");
}

fn main() {
    bad_number(18);
    bad_number(22);
}
```

[panic! macro playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f4bc0db3dc0f37cff03c05576a93f32e)

## Option enum

The [Option](https://doc.rust-lang.org/std/option/index.html) enum represents an optional value with 2 states:

1. `Some`
2. `None`

```rust
fn input(phrase: Option<&str>) -> String {
    match phrase {
        Some("Workshopper") => String::from("Hello There"),
        Some(l) => String::from(l),
        None => String::from(""),
    }
}

fn main() {
    assert_eq!(input(Some("Workshopper")), "Hello There");
    assert_eq!(input(None), "");
    assert_eq!(input(Some("Yup")), "Yup");
}
```

In the program above we defined a function called `input` that takes an Option Enum as a parameter and uses pattern matching to return a string back to user.

[Option playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c839dca95aab0aaa1f1686bd577f5490)

## ? operator

We can use the `?` for error propagation in Rust.

```rust
fn version(ph: Option<i32>) -> Option<i32> {
    let guess_number: i32 = ph;
    Some(guess_number * 5)
}

fn main() {
    let sum = version(Some(5));
    assert_eq!(sum.unwrap(), 25);
    
    let problem = version(None);
    assert_eq!(problem, None);
}
```

Notice that here we propagated the error by using `?` in the program. simplying this program so we didn't need to use pattern matching.

[? playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a40672fa287a12c50694290a375c9a73)

## Flow Control with Combinators

The concept of a combinator in Rust is closely tied to its definition in Haskell:

[Combinator Haskell](https://wiki.haskell.org/Combinator)

which is a function or a definition with no free variables.

In Rust a combinator function will combine build some program parts from other program parts.


Combinators can be used to manage control flow such as if you use the `map` combinator method that is defined in the `Option` enum.

```rust
fn phrase(s: Option<String>) -> Option<usize> {
    s.map(|n| n.len())
}

fn main() {
    let count = phrase(Some(String::from("I am a phrase")));
    assert_eq!(count.unwrap(), 13);
    let none = phrase(None);
    assert_eq!(None, none);
}
```

Notice that here we use the map combinator method and it returns an option value and handles flow control for us.

[map combinator playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f91833e3433dcafec42e2557e4a0b65a)

## Result enum

The [Result](https://doc.rust-lang.org/std/result/enum.Result.html) enum is a type in Rust that represents Success => `Ok` or Failure => `Err`

Let us look at some example to better explain this concept:

```rust
fn dicey(d: Result<u32, &str>) -> Option<&str> {
    d.err()
}

fn main() {
    assert_eq!(dicey(Err("What the dice!")).unwrap(), "What the dice!");
}
```

In this example we look at a function that takes a Result as a parameter and then calls the `err` method which returns an Option type.

[Result Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=aa488f581b6e6be532165b55affc7caa)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Standard Library](./standard_library.md) | [Generic Types](./generic_types.md) →
