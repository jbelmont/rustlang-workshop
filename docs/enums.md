# Rust Workshop - Enums

## Sections:

* [Enum definition](#enum-definition)
* [Enum example](#enum-example)
* [Match expression](#match-expression)
* [if let expression](#if-let-expression)
* [Enum playground](#enum-playground)
* [Enum koan](#enum-koan)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Enum definition

[Enumerated Type specification](https://doc.rust-lang.org/reference/types/enum.html)

> An enumeration, also referred to as enum is a simultaneous definition of a nominal enumerated type as well as a set of constructors, that can be used to create or pattern-match values of the corresponding enumerated type.

> Enumerations are declared with the keyword enum.

## Enum example

```rust
enum Dance {
    Tango(String),
    Salsa(String),
    Meringue(String),
}

impl Dance {
    fn phrase(&self) {
        match self {
            Dance::Tango(c) => println!("{}: Time to Tango", c),
            Dance::Salsa(c) => println!("{}: Salsa time", c),
            Dance::Meringue(c) => println!("{}: Not your average meringue", c),
        }
    }
    
    fn new_tango() -> Dance {
        Dance::Tango(String::from("New Tango"))
    }
}
```

Notice that enumerations can also have methods defined on them and in this case the phrase for a particular dance is defined.

## Match Expression

[Match expression specification](https://doc.rust-lang.org/reference/expressions/match-expr.html)

> A match expression branches on a pattern. The exact form of matching that occurs depends on the pattern. A match expression has a scrutinee expression, which is the value to compare to the patterns. The scrutinee expression and the patterns must have the same type.

Match expressions are useful with enums as you can match on the enumerated items.

Notice that in our earlier example we matched against `Dance::Tango`, etc using the match operator and then a statement is logged to standard output if a match evaluates to true.

## if let expression

[if let specification](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)

> An if let expression is semantically similar to an if expression but in place of a condition expression it expects the keyword let followed by a pattern, an = and a scrutinee expression. If the value of the scrutinee matches the pattern, the corresponding block will execute. Otherwise, flow proceeds to the following else block if it exists. Like if expressions, if let expressions have a value determined by the block that is evaluated.

If you look at the enum playground link you will see that I used an `if let` expression to unwrap the string value. 

## Enum playground

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=181056a1656ba5aab3bfadfdd2e6336b)

## Enum koan

[Enum koan](../koans/src/enums.rs)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Methods](./methods.md) | [Pattern Matching](./pattern_matching.md) →
