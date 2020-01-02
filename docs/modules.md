# Rust Workshop - Modules

## Sections:

* [Visibility](#visibility)
    * [Privacy](#privacy)
    * [pub keyword](#pub-keyword)
* [Module Tree](#module-tree)
    * [absolute path](#absolute-path)
    * [relative path](#relative-path)
    * [when to choose absolute or relative](#when-to-choose-absolute-or-relative)
* [use keyword](#use-keyword)
* [module separation](#module-separation)
    * [module separation example](#module-separation-example)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Visibility

In rust items in a module have private visibility turned on by default.

The privacy rules in rust extend to the following items:

* functions
* methods
* structs
* enums
* modules
* constants

Additionally for structs their fields are private by default.

#### Privacy

Let use look at the following example on privacy rules

```rust
fn main() {
    // will not compile since log is private
    // usage1::log();
    
    usage1::echo_message();
}

mod usage1 {
    #[allow(dead_code)]
    fn log() {
        println!("{}", "Log my message");
    }
    
    pub fn echo_message() {
        println!("{}", "I can by called!");
    }
}
```

We can call the `usage1` module because other items can access other items that exist in the same module, even though they are private.

We called the `usage1` module using relative path which we will talk about later in this section

[privacy playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=469ff4aea30607e07b3260e85e0a6073)

#### pub keyword

The `pub` keyword is a visibility modifier that lets us override the default `private` visibility in rust.

This helps ensure that proper encapsulation of modules is enforced. This also goes hand in hand with the principle of least privilege.

A module should only be able to access the {structs,constants,functions,enums,modules} that are necessary for its intended purpose.

*When we use the `pub` keyword we modify `{structs,constants,functions,enums,modules}` for usage outside of the module in which they are defined and therefore can be used in other modules that there were not defined in.*

Let us look at an example to better explain this concept:

```rust
// src/species.rs
mod animal {
    fn traits() {}
}

// src/person.rs
mod human {
    fn traits() {}
}
```

If we were to try to access `animal::traits()` in the `human` module we would get a compile time error since animal is a private module and its function traits is private as well.

If we were to change its visibility to public using the `pub` keyword we could access it on the person module.

```rust
// src/species.rs
pub mod animal {
    pub fn traits() {}
}

// src/person.rs
mod human {
    fn traits() {}
}
```

Notice that we used the `pub` keyword in the `mod` keyword and in the `fn` in order to call the traits function that lives in the animal module

## Module Tree

[modules specification](https://doc.rust-lang.org/reference/items/modules.html)

> A module is a container for zero or more items.

> A module item is a module, surrounded in braces, named, and prefixed with the keyword mod. A module item introduces a new, named module into the tree of modules making up a crate. Modules can nest arbitrarily.

> Modules and types share the same namespace. Declaring a named type with the same name as a module in scope is forbidden: that is, a type definition, trait, struct, enumeration, union, type parameter or crate can't shadow the name of a module in scope, or vice versa. Items brought into scope with use also have this restriction.

Rust needs to know the path of a function or module just like in Unix/Linux you can't call a bash script without specifying its path or editing the `PATH` variable so that the shell knows where to find the script you are calling.

[Module Tree Path](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)

A path can take two forms:

* An absolute path starts from a crate root by using a crate name or a literal crate.

* A relative path starts from the current module and uses self, super, or an identifier in the current module.

#### absolute path

You can use the `crate` keyword in order to specify the absolute path for where a module lives.

```rust
fn main() {
    // relative path usage
    let soldier = crate::military::Soldier::new("John Rambo", 32, "SFC", 12);
    assert_eq!(soldier.name, "John Rambo");
    // assert_eq!(soldier.age, 32);
}

mod military {
    #[allow(dead_code)]
    pub struct Soldier {
        pub name: String,
        age: u32,
        rank: String,
        years_of_service: u32,
    }
    impl Soldier {
        pub fn new(name: &str, age: u32, rank: &str, years_of_service: u32) -> Soldier {
            Soldier {
                name: String::from(name),
                age: age,
                rank: String::from(rank),
                years_of_service: years_of_service,
            }
        }
    }
}
```

[absolute path playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=383e42136fbbd07da238715abb64717e)

Notice that we used the `crate` keyword and then used called the `military` module. 

This usage is known as using the absolute path.

In order for us to use the Soldier struct outside of the `military` module we neen to change its visibility to public.

*Additionally each field that we want to use outside of the military module needs to be made public so that is why we added the `pub` access modifier to the `name` field in the `Soldier` struct.*

#### relative path

We can also call modules and its items by using their relative path.

Let us look at an example to further clarify this idea:

```rust
fn main() {
    let person = species::Human::new("John Rambo", 32, "Male");
    assert_eq!(person.name, "John Rambo");
}

mod species {
    #[allow(dead_code)]
    pub struct Human {
        pub name: String,
        age: u32,
        gender: String,
    }
    
    impl Human {
        pub fn new(name: &str, age: u32, gender: &str) -> Human {
            Human {
                name: String::from(name),
                age: age,
                gender: String::from(gender),
            }
        }
    }
}
```

Notice that we called the method in the `species` module using relative path syntax.

[relative path playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e58aa3191adebfd9fbdbf89533447a6d)

#### when to choose absolute or relative

Each software project will need to decide whether to use absolute or relative.

If we go back to the previous example and moved species module to another module then the absolute path of the module will have changed but the relative path would still be the same.

So to clarify if we created another module in another module called `animal_kingdom` then the absolute path would be:

`animal_kingdom::species::Human::new("John Rambo", 32, "Male")`

while the relative path would still just be:

`species::Human::new("John Rambo", 32, "Male")`

if we had a function in the same example like this:

```rust
mod species {
    #[allow(dead_code)]
    pub struct Human {
        pub name: String,
        age: u32,
        gender: String,
    }
    
    impl Human {
        pub fn new(name: &str, age: u32, gender: &str) -> Human {
            Human {
                name: String::from(name),
                age: age,
                gender: String::from(gender),
            }
        }
    }
}

pub fn logging_example() {
    crate::species::Human::new("John Rambo", 32, "Male");
}
```

If we were to move the logging_example function to another module then the absolute path `new` method call would stay the same but the relative path for the `logging_example` function would need to be updated.

## use keyword

The `use` keyword is used to bring items into scope in rust

Let us look at an example of the `use` keyword

```rust
fn main() {
    let pov = Truck::new("Ford", "F150", 2019);
    println!("{}", pov.message());
}

use vehicle::Truck;

mod vehicle {
    pub struct Truck {
        make: String,
        model: String,
        year: u32,
    }
    
    impl Truck {
        pub fn new(make: &str, model: &str, year: u32) -> Truck {
            Truck {
                make: String::from(make),
                model: String::from(model),
                year: year,
            }
        }
        
        pub fn message(&self) -> String {
            format!(
                "Make: {},\nModel: {}\nYear: {}",
                self.make,
                self.model,
                self.year,
            )
        }
    }
}
```

Notice that we were able to call the Truck method and its methods without specifying the vehicle module.

This is because the `use` statement of `use vehicle::Truck;` brought the Truck struct into scope for the current module.

[use keyword playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9ab9b22825a691ce2111c1cfe5dc3fa9)

## module separation

Modules can be separated into different files for further organization.

When you use the `;` (semicolon) instead of using a block statement then rust knows to lad teh contents of the module from another file with the sanme name as the module.

#### module separation example

There is an example of this in the koans crate in the workshop if you look at:

[Modules](../koans/src/modules.rs) => `pub mod usage;`

The statement `pub mod usage` loads the content of [usage](../koans/src/modules/usage.rs)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Patterns](./patterns.md) | [Standard Library](./standard_library.md) →
