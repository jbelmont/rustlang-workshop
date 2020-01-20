# Rust Workshop - Generics

## Sections:

* [Concept of Generics](#concept-of-generics)
* [Generic Function](#generic-function)
* [Implementations](#implementations)
* [Traits](#traits)
* [Bounds](#bounds)
* [Where Clauses](#where-clauses)
* [Associated Items](#associated-items)
* [Phantom Types](#phantom-types)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Concept of Generics

Much like when you want to make a general analogy about a concept you can utilize Generics in Rust to generalize your Rust Programs.

We could specify a function and have functions that behave in the same exact way but that would be unnecessary if we can utilize generics.

You will typically see generics denoted by angle brackets and one letter Capitalized alphabet such as:

```rust
<T> => List of <T>
<E> => List of <E>
```

## Generic Function

A function can be considered generic if it is preceded by a `<T>` before its parameters specified in `(...)` and its name:

```rust
// generic soldier
struct Soldier<T>(T);

fn i_am_generic<T>(s: Soldier<T>) {}
```

Here we have a struct named `Soldier` that takes a generic of T and our function named `i_am_generic` takes a generic argument of T and specifies an argument of `Soldier<T>`

## Implementations

In order for implementation to be generic the `<T>` must precede the type like this:

```rust
struct Student<'a, T, U> {
    scores: &'a [T],
    name: U,
}

impl<T, U> Student<'_, T, U> {
    fn get_scores(&self) -> &[T] {
        self.scores
    }
}

fn main() {
    let student = Student {
        scores: &vec![98, 99, 88, 85, 100],
        name: String::from("Jean-Claude Van Damme"),
    };
    
    let mut summation = 0;
    for score in student.get_scores().iter() {
        summation += score;
    }
    assert_eq!(summation / student.get_scores().len(), 94);
    assert_eq!(student.name, String::from("Jean-Claude Van Damme"));
}
```

Notice that here we have `impl<T, U>` defined before the type declaration for the `Student` struct type.

## Bounds

> Trait and lifetime bounds provide a way for generic items to restrict which types and lifetimes are used as their parameters. Bounds can be provided on any type in a where clause. There are also shorter forms for certain common cases:

[Trait Bounds specification](https://doc.rust-lang.org/reference/trait-bounds.html)

#### Traits Recap

*We previously mentioned Traits earlier in the workshop on the [Data Types -> Trait Types Section](./data_types.md#trait-types) so please check it out if you skipped to this section and do not know what a Trait is..*

```rust
struct Number {
    num1: i32,
    num2: i32,
}

trait Calculator {
    fn adder(&self) -> i32;
    fn subtracter(&self) -> i32;
}

impl Calculator for Number {
    fn adder(&self) -> i32 {
        self.num1 + self.num2
    }
    fn subtracter(&self) -> i32 {
        self.num1 - self.num2
    }
}

fn main() {
    let number = Number {
        num1: 5,
        num2: 4,
    };
    assert_eq!(number.adder(), 9);
    assert_eq!(number.subtracter(), 1);
}
```

[traits example recap playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e7fb35403f6f14529eb62303ec81c8e9)

#### Bounds example

```rust
#[derive(Debug,PartialEq)]
struct Army;

#[allow(dead_code)]
#[derive(Debug,PartialEq)]
struct Navy;

#[allow(dead_code)]
struct AirForce;

#[allow(dead_code)]
struct Marines;

#[allow(dead_code)]
struct CoastGuard;

trait Service {}

impl Service for Army {}

fn army<T: Service>(a: &T) -> &T {
    a
}

fn main() {
    let soldier = Army;
    // army is bound to Army and won't work with other services
    assert_eq!(&Army, army(&soldier));
}
```

Notice that Service is bound to Army here and not to other services.

[Bounds example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2b53fa275bb1483428a64e8acc1e782f)

## Where Clauses

A `Trait` bound can be expressed using a `where` clause rather than at the Type Level `<T>`.

A `where` clause can apply Trait bounds to arbitrary types and not just type parameters.

#### Here is how the `where` clause looks like in function:

```rust
// Example of where clause with function
fn func_thing<T, U>(t: T, u: U) -> f32 
    where T: Display + Clone, U: Clone + Debug {

    }
```

#### Here is how the `where` clause looks like in implementation:

```rust
use std::fmt::Debug;

trait Dude {
    fn he_who_abides(self) -> String;
}

impl<T: std::fmt::Display> Dude for T where Option<T>: Debug {
    fn he_who_abides(self) -> String {
        format!("{}", self)
    }
}

fn main() {
    let sentence = "The rug really tied the room together";
    let big_lebowski_phrase = String::from(sentence);
    assert_eq!(
        big_lebowski_phrase.he_who_abides(),
        sentence,
    );
}
```

## Associated Items

[Associated Types specification](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types)

> Associated types are type aliases associated with another type. Associated types cannot be defined in inherent implementations nor can they be given a default implementation in traits.

> An associated type definition defines a type alias on another type. It is written as type, then an identifier, then an =, and finally a type.

#### Associated Item example:

```rust
#[allow(dead_code)]
struct ServiceMember {
    branch: String,
    name: String,
    age: u32,
    rank: String,
    years_of_experience: u32,
}

trait MilitaryBranch {
    type Branch;
    
    fn member_of_branch(
        &self,
        _: &Self::Branch,
    ) -> String;
}

impl MilitaryBranch for ServiceMember {
    type Branch = String;
    
    fn member_of_branch(
        &self,
        branch: &String,
        
    ) -> String {
        match &self.branch {
            s if &self.branch == branch => format!("{} is in {}", &self.name, s),
            _ => format!("{}", "Branch not known"),
        }
    }
}

fn main() {
    let army_soldier = ServiceMember {
        branch: String::from("Army"),
        name: String::from("John Rambo"),
        age: 32,
        rank: String::from("SFC"),
        years_of_experience: 12,
    };
    let navy_soldier = ServiceMember {
        branch: String::from("Navy"),
        name: String::from("Fried Squid"),
        age: 21,
        rank: String::from("Ensign"),
        years_of_experience: 1,
    };
    let air_force_soldier = ServiceMember {
        branch: String::from("Air Force"),
        name: String::from("Chuck Norris"),
        age: 33,
        rank: String::from("Technical Sergeant"),
        years_of_experience: 10,
    };
    assert_eq!(
        army_soldier.member_of_branch(&String::from("Army")),
        "John Rambo is in Army",
    );
    assert_eq!(
        navy_soldier.member_of_branch(&String::from("Navy")),
        "Fried Squid is in Navy",
    );
    assert_eq!(
        air_force_soldier.member_of_branch(&String::from("Air Force")),
        "Chuck Norris is in Air Force",
    );
}
```

[Associated Item playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e43a7d0a07ef1a82d92b1855b4d21bfd)

## Phantom Types

content

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Error Handling](./error_handling.md) | [Lifetimes](./lifetimes.md) →
