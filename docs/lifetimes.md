# Rust Workshop - Lifetimes

## Sections:

* [Lifetime Concept](#lifetime-concept)
* [Explicit Lifetime Annotation](#explicit-lifetime-annotation)
* [Lifetimes and Functions](#lifetimes-and-functions)
* [Lifetimes and Methods](#lifetimes-and-methods)
* [Lifetimes and Structs](#lifetimes-and-structs)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Lifetime Concept

Lifetimes are a Rust Programming Language construct that ensure that all borrows are valid.

*Lifetimes begin when the variable is created and a lifetime ends when the variable is destroyed.*

```rust
fn main() {
    let num = 5;
    println!("num is {}", num);     // Lifetime for `num` starts here
    {
        let borrow_num = &num; // Lifetime for `borrow_num` start here
        println!("borrow_num is {}", borrow_num);
    } // Lifetime for `borrow_num` ends here
} // Lifetime for `num` ends here
```

## Explicit Lifetime Annotation

The notation to denote lifetimes in rust is apostrophe `'`, here is an example:

```rust
Dude<'a>
```

## Lifetimes and Functions

*We will talk later about elision :roll_eyes:*

Function Signatures with Lifetime constraints:

* A reference must have an annotated lifetime

```rust
fn main() {
    let num: u32 = 5;
    assert_eq!(*number(&num), 5);
    
    print_num(&num);
}

// 'n has the same lifetime as the the returned reference
fn number<'n>(num: &'n u32) -> &'n u32 {
    num
}

// The input reference 'n lives as long as the function
fn print_num<'n>(num: &'n u32) {
    println!("num is {}", num);
}
```

## Lifetimes and Methods

The lifetime annotation for methods is the same as for functions.

```rust
struct Soldier {
    name: String,
    age: u32,
}

impl Soldier {
    fn print_attrs<'s>(&'s self) -> String {
        format!(
            "{}\n{}",
            self.name,
            self.age
        )
    }
}

fn main() {
    let rambo = Soldier {
        name: String::from("John Rambo"),
        age: 32,
    };
    assert_eq!(rambo.print_attrs(), "John Rambo\n32");
}
```

## Lifetimes and Structs

Like methods the annotations for structs are the same

```rust
struct Unit<'u>(&'u f32);

struct Square<'l, 'w> {
    length: &'l i32,
    width: &'w i32,
}

impl<'l, 'w> Square<'l, 'w> {
    fn area<'a>(&'a self) -> i32 {
        self.length * self.width
    }
}

fn main() {
    let unit = Unit(&5.5);
    assert_eq!(*unit.0, 5.5);
    
    let sq = Square {
        length: &5,
        width: &4,
    };
    assert_eq!(sq.area(), 20);
}
```

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Generics](./generics.md) | [Testing](./testing.md) →
