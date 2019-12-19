# Rust Workshop - Patterns

## Sections:

* [Different Pattern Types in Rust](#different-pattern-types-in-rust)
* [match args](#match-args)
* [if let](#if-let)
* [while let](while-let)
* [for loop](#for-loop)
* [let statement](#let-statement)
* [function parameter pattern](#function-parameter-pattern)
* [Patterns koan](#patterns-koan)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Different Pattern Types in Rust

Patterns can be used in each of the following areas in Rust:

* `match args`

* `if let`

* `while let`

* `for loop`

* `let statement`

* `function parameter`

## match args

We covered the match statement earlier in the enum section of the workshop.

match arms are a powerful pattern matching concept in rust but they can be used in more circumstances than just enums they can be used with numbers and in different manners.

#### match arm with number

```rust
let number: u32 = 9;
    
let computation: u32 = 1;
let number = match number {
    1 => computation * 2,
    2 | 3 => computation * 2 * 3,
    3 ..= 7 => computation * 2 * 3 * 4 * 5 * 6 * 7,
    n @ 8 ..= 9 => computation * 2 * 3 * 4 * 5 * 6 * 7 * n,
    _ => 0,
};
// 1 * 2 * 3 * 4 * 5 * 6 * 7 * 9 => 45360
assert_eq!(number, 45360);
```

Notice that we assigned `number` a value from the match expresssion evaluation

[match arm playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=49ac031c2fd4993d0099442a0f74bf0a)

## if let

[if let specification](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)

`if let` is a useful pattern when using match expressions can be clunky/awkward.

It looks cleaner and will do the pattern match we are looking for.

```rust
enum Animal {
    Dog(String),
    Cat(String),
}

impl Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Dog(d) => format!("Dog says: {}", d),
            Animal::Cat(c) => format!("Cat says: {}", c),
        }
    }
}

fn main() {
    let dog = Animal::Dog(String::from("arf arf growl!"));
    let cat = Animal::Cat(String::from("meow meow purr!"));
    
    assert_eq!(dog.speak(), String::from("Dog says: arf arf growl!"));
    
    if let Animal::Cat(c) = cat {
        assert_eq!(c, "meow meow purr!");
    }
}
```

Notice that in the code example above we used `if let` to match the `Animal::Cat(String)` enum.

[if let playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ad84cc3f506447310e4618c3d79ba0c3)

## while let

The `while let` pattern is another pattern matching expression we can use with rust.

It can nicely replace patterns with match arms that are unwieldy.

A `while let` loop will run as long as the condition is true.

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let mut value = 9;
    while let Some(top) = numbers.pop() {
        assert_eq!(top, value);
        value -= 1;
    }
}
```

In this example the numbers vector has 9 values and each value is popped off the stack so when it gets to the last value of 1 it will cease being true and the expression will no longer be true.

The `pop` method has the following signature:

```rust
pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
        None
    } else {
        unsafe {
            self.len -= 1;
            Some(ptr::read(self.get_unchecked(self.len())))
        }
    }
}
```

Notice that it returns an `Option` type and it uses the `<T>` which will discuss later in the workshop.

When the length is equal 0 it returns `None`

[while let playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d23bd77e0e98d3977de111667cd252be)

## for loop

The `for` loop can also take a pattern that is evaluated.

```rust
fn main() {
    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    
    let mut counter = 0;
    for (index, ch) in chars.iter().enumerate() {
        match ch {
            'a' => assert_eq!(*ch, 'a'),
            'b' => assert_eq!(*ch, 'b'),
            'c' => assert_eq!(*ch, 'c'),
            'd' => assert_eq!(*ch, 'd'),
            'e' => assert_eq!(*ch, 'e'),
            _ => (),
        }
        assert_eq!(index, counter);
        counter += 1;
    }
}
```

[for pattern playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=97cd72da36fbf01ca844774cebe258fe)

## let statement

The `let` statement itself is a pattern that matches an expression evaluation

```rust
let num = 5;
```

This evaluates the primitive `i32` value and assigns the value 5.

```rust
let PATTERN = EXPRESSION;
```

We can assign tuple values and use destructuring in the let evaluation

```rust
fn main() {
    struct Ballerina {
        name: String,
        age: u32,
    }
    
    let anna = Ballerina {
        name: String::from("Anna Person"),
        age: 21,
    };
    
    let (name, age) = (anna.name, anna.age);
    
    assert_eq!(name, "Anna Person");
    assert_eq!(age, 21);
}
```

Notice that in this code example we instantiated a Ballerina struct value and assigned a tuple value using `let` statement.

[let statement playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0c304b9f382ac470b3cf1a2c732e5925)

## function parameter pattern

functions in rust can also be pattern themselves; let us look at a possible example:

```rust
fn main() {
    
    fn number_report((numbers, adder): (Vec<u32>, u32)) -> (u32, usize) {
        let mut sum = 0;
        
        for n in numbers.iter() {
            sum += n + adder
        }
        (sum, numbers.len())
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    let args = (numbers, 5);
    
    let (summation, length) = number_report(args);
    assert_eq!(summation, 40);
    assert_eq!(length, 5);
    
}
```

[function parameter pattern playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5e1e5b56cafc7e6a86efef3e03db3ed1)

## Patterns koan

[Patterns koan](../koans/src/patterns.rs)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Enums](./enums.md) | [Modules](./modules.md) →
