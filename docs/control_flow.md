# Rust Workshop - Control Flow

## Sections:

* [Rust control structures](#rust-control-structures)
* [if expression](#if-expression)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Rust control structures

Rust has the following control structures:

* [if](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-expressions)

* [if let](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)

* [match expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html)
    * similar to `switch` statements in other programming languages

* [loop expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops)
    * similar to `while(true) {}` c language or `for {}` in golang

* [while expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-loops)

* [while let expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops)

* [for expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops)

* [break expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions)

* [continue expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions)

* [label](https://doc.rust-lang.org/reference/expressions/loop-expr.html#loop-labels)

## if expression

[if specification](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-expressions)

> An if expression is a conditional branch in program control. The form of an if expression is a condition expression, followed by a consequent block, any number of else if conditions and blocks, and an optional trailing else block. The condition expressions must have type bool. If a condition expression evaluates to true, the consequent block is executed and any subsequent else if or else block is skipped. If a condition expression evaluates to false, the consequent block is skipped and any subsequent else if condition is evaluated. If all if and else if conditions evaluate to false then any else block is executed. An if expression evaluates to the same value as the executed block, or () if no block is evaluated. An if expression must have the same type in all situations.

Rust has control structures as you may have seen them in other programming languages.

*You will not see parenthesis typically in the `if`control structures however.*

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

## if let expression

[if let specification](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)

> An if let expression is semantically similar to an if expression but in place of a condition expression it expects the keyword let followed by a pattern, an = and a scrutinee expression. If the value of the scrutinee matches the pattern, the corresponding block will execute. Otherwise, flow proceeds to the following else block if it exists. Like if expressions, if let expressions have a value determined by the block that is evaluated.

#### if let example

```rust
fn main() {
    let num = Some(5);
    
    let matchedNone: Option<i32> = None;
    
    if let Some(x) = num {
        println!("Matched {:?}", x)
    }
    
    if let Some(x) = matchedNone {
        println!("Matched {:?}", x);
    } else {
        println!("Didn't match anything")
    }
}
```

When you run this example you get: 

```bash
Matched 5
Didn\'t match anything
```

[if let playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=56dd5b16e38d9abaad3a7a0b654848be)

## match expression

[match specification](https://doc.rust-lang.org/reference/expressions/match-expr.html)

> A match expression branches on a pattern. The exact form of matching that occurs depends on the pattern. A match expression has a scrutinee expression, which is the value to compare to the patterns. The scrutinee expression and the patterns must have the same type.

#### match example

```rust
fn main() {
    let num = 7;
    
    match num {
        1 => println!("matched 1"),
        2 | 3 | 5 => println!("matched prime"),
        2 ... 7 => println!("matched 2,3,4,5,6,7"),
        _ => println!("Didn't match something"),
    }
    
    let boolean = false;
    
    let switch = match boolean {
        true => 1,
        false => 0,
    };
    
    println!("{} -> {}", boolean, switch);
}
```

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
←  [stdin, stdout, and stderr](./stdin-stdout-stderr.md) | [Functions](./functions.md) →
