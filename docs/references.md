# Rust Workshop - References

## Sections:

* [References in Rust](#references-in-rust)
* [Reference example](#reference-example)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## References in Rust

The issue we got in the previous example in the ownership document was that our print_soldier function was taking ownership of the Soldier struct value.

If however we pass a reference then we are letting the print_soldier function borrow the value of a Soldier struct variable. This has the effect that we can still use the value of the struct after a call to this function.

## Reference example

We can fix the code example by doing the following:

```rust
struct Soldier {
    name: String,
    age: i32,
}

fn print_soldier(soldier: &Soldier) -> String {
    let str = format!("Hello {} and you say your age is {}", soldier.name, soldier.age);
    str
}

fn main() {
    let rambo = Soldier {
        name: String::from("Arnold"),
        age: 35,
    };
    
    print_soldier(&rambo);
    print_soldier(&rambo);
    print_soldier(&rambo);
}

#[test]
fn should_print_soldier() {
    let rambo = Soldier {
        name: String::from("John Rambo"),
        age: 34,
    };
    let expected = String::from("Hello John Rambo and you say your age is 34");
    assert_eq!(print_soldier(rambo), expected);
}
```

Notice the usage of the `&` in the rust example, this is how you pass a reference in Rust.

Now that we made 3 calls to the print_soldier function but did not get a move error like before.

*This is all because of references, when we didn't pass a reference the print_soldier took ownership and so we could not reuse the variable.*

[References playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c621e567ee1fabf7cd5593d2740a8bf3)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Ownership](./ownership.md) | [Borrowing](./borrowing.md) →
