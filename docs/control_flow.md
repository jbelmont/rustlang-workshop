# Rust Workshop - Control Flow

## Sections:

* [if and if let expression](#if-and-if-let-expressions)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## if and if let expression

[if and if let specification](https://doc.rust-lang.org/reference/expressions/if-expr.html)

> An if expression is a conditional branch in program control. The form of an if expression is a condition expression, followed by a consequent block, any number of else if conditions and blocks, and an optional trailing else block. The condition expressions must have type bool. If a condition expression evaluates to true, the consequent block is executed and any subsequent else if or else block is skipped. If a condition expression evaluates to false, the consequent block is skipped and any subsequent else if condition is evaluated. If all if and else if conditions evaluate to false then any else block is executed. An if expression evaluates to the same value as the executed block, or () if no block is evaluated. An if expression must have the same type in all situations.

Rust has control structures as you may have seen them in other programming languages.

*You will not see parenthesis typically in the `if` and `if let` control structures however.*

```rust
fn main() {
    let x = 10;
    if x > 0 {
        println!("in here");
    } else if x < 10 {
        println!("didn't get here")
    } else {
        println!("got in this block")
    }
}
```

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
←  [stdin, stdout, and stderr](./stdin-stdout-stderr.md) | [Functions](./functions.md) →
