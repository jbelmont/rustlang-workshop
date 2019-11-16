# Rust Workshop - Control Flow

## Sections:

* [Rust control structures](#rust-control-structures)
* [if expression](#if-expression)
* [if let expression](#if-let-expression)
    * [if let example](#if-let-example)
* [match expression](#match-expression)
    * [match example](#match-example)
* [loop expression](#loop-expression)
    * [loop example](#loop-example)
* [while expression](#while-expression)
    * [while example](#while-example)
* [while let expression](#while-let-expression)
    * [while let example](#while-let-example)
* [for expression](#for-expression)
    * [for example](#for-example)
* [break expression](#break-expression)
    * [break example](#break-example)
* [continue expression](#continue-expression)
    * [continue example](#continue-example)
* [label expression](#label-expression)
    * [label example](#label-example)
* [control flow koan](#control-flow-koan)
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

*match is somewhat similar to switch statement like other programming languages.*

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

## loop expression

[loop expression specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops)

> A loop expression repeats execution of its body continuously: `loop { println!("I live."); }`.

> A loop expression without an associated break expression is diverging and has type `!`. A loop expression containing associated break expression(s) may terminate, and must have type compatible with the value of the break expression(s).

#### loop example

```rust
fn main() {
    loop {
        println!("Hello People!");
    }
}
```

This will continuously print "Hello People!" to standard output since there is no terminating condition or `break` expression.

## while expression

[while expression specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-loops)

> A while loop begins by evaluating the boolean loop conditional expression. If the loop conditional expression evaluates to true, the loop body block executes, then control returns to the loop conditional expression. If the loop conditional expression evaluates to false, the while expression completes.

#### while example

```rust
fn main() {
    let mut number: i32 = 0;
    
    let mut sum: i32 = 0;
    while number < 100 {
        sum += number;
        number += 1;
    }
    println!("{}", sum);
}
```

[while example playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=299090103f4be9765c6d2dfaddb52441)

## while let expression

[while let specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops)

> A while let loop is semantically similar to a while loop but in place of a condition expression it expects the keyword let followed by a pattern, an =, a scrutinee expression and a block expression. If the value of the scrutinee matches the pattern, the loop body block executes then control returns to the pattern matching statement. Otherwise, the while expression completes.

## while let example

```rust
fn main() {
    let mut primes = vec![3, 5, 7];
    
    while let Some(y) = primes.pop() {
        println!("y = {}", y);
    }
}
```

[while let playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2d34940b85a215403c7d10cc5f1a6f3f)

## for expression

[for expression specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops)

> A for expression is a syntactic construct for looping over elements provided by an implementation of `std::iter::IntoIterator`. If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the for loop. If the iterator is empty, the for expression completes.

#### for example

```rust
fn main() {
    let numbers = vec![3, 5, 7];
    
    let mut multiplied_numbers = Vec::new();
    for prime in numbers {
        multiplied_numbers.push(prime * 5);
    }
    
    println!("{:?}", multiplied_numbers);
    
    assert_eq!(multiplied_numbers[0], 15);
}
```

[for example playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5a7f3aa7489760a1b09df4f9d07bb908)

## break expression

[break expression specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions)

> When break is encountered, execution of the associated loop body is immediately terminated

> A break expression is normally associated with the innermost loop, for or while loop enclosing the break expression, but a label can be used to specify which enclosing loop is affected.

#### break example

```rust
fn main() {
    let phrases = vec!["Here", "I", "Am"];
    
    for phrase in phrases {
        if phrase == "I" {
            break;
        } else {
            println!("{}", phrase);
        }
    }
}
```

[break example playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=62054bc7b559f87573012281d5d74f63)

## continue expression

[continue expression specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions)

> When continue is encountered, the current iteration of the associated loop body is immediately terminated, returning control to the loop head. In the case of a while loop, the head is the conditional expression controlling the loop. In the case of a for loop, the head is the call-expression controlling the loop.

> Like break, continue is normally associated with the innermost enclosing loop, but continue 'label may be used to specify the loop affected. A continue expression is only permitted in the body of a loop.

#### continue example

```rust
fn main() {
    let numbers = vec![3, 5 , 7, 11];
    
    for num in numbers {
        if num == 7 {
            continue
        }
        println!("{}", num);
    }
}
```

[continue example playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=413e58793df5ccbf29ae453ecbe3bc2f)

## label expression

[label expression specification](https://doc.rust-lang.org/reference/expressions/loop-expr.html#loop-labels)

> A loop expression may optionally have a label. The label is written as a lifetime preceding the loop expression, as in 'foo: loop { break 'foo; }, 'bar: while false {}, 'humbug: for _ in 0..0 {}. If a label is present, then labeled break and continue expressions nested within this loop may exit out of this loop or return control to its head. See break expressions and continue expressions.


## label example

```rust
fn main() {
    let numbers = vec![1, 3, 5];
    'outer: loop {
        for num in &numbers {
            if *num == 3 {
                break 'outer;   
            }
            println!("{}", *num * 5);
        }
    }
}
```

[label example playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0fb219093c247e29cd15d861e7672e84)

## control flow koan

[control flow koan](../koans/src/control_flow.rs)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
←  [stdin, stdout, and stderr](./stdin-stdout-stderr.md) | [Functions](./functions.md) →
