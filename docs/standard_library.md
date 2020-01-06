# Rust Workshop - Standard Library

## Sections:

* [Rust Standard Library](#rust-standard-library)
    * [Common Collections](#common-collections)
    * [Input/Output (IO)](#input\/output-\(IO\))
    * [Multithreading](#multithreading)
    * [Primitive Types](#primitive-types)
    * [The Rust Prelude](#the-rust-prelude)
    * [Threading](#threading)
* [Common Collections](#common-collections)
    * [Vectors](#vectors)
    * [Hash Maps](#hash-maps)
    * [Strings](#strings)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Rust Standard Library

Rust has a standard library much like other programming languages. 

From what I have learned Rust seems to follow the philosophy of C/C++ of maintaining a smaller standard library.

Rust does not proscribe to Python's Batteries included mentality for standard libraries.

#### Common Collections

Rust maintains a smaller standard library core and uses common collections such as:

* Sequences:
    * [Vectors](https://doc.rust-lang.org/std/vec/struct.Vec.html)
    * [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
    * [LinkedList](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)


* Maps:
    * [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
    * [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)


* Sets:
    * [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
    * [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)


* Binary Trees:
    * [Binary Heap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)

#### Input/Output (IO)

Rust has a package called [std::io](https://doc.rust-lang.org/std/io/index.html) for dealing with input and output.

Rust deals with common types of IO such as:

* io
    * commonly needed io operations module
    * [io](https://doc.rust-lang.org/std/io/index.html)


* files
    * filesystem module
    * [fs](https://doc.rust-lang.org/std/fs/index.html)


* TCP/UDP Communication 
    * This module provides networking functionality for the Transmission Control and User Datagram Protocols, as well as types for IP and socket addresses.
    * [net](https://doc.rust-lang.org/std/net/index.html)

#### Multithreading

Rust has a package for multithreading operations called [std::thread](https://doc.rust-lang.org/std/thread/index.html)

#### Primitive Types

[Primitives in Standard Library](https://doc.rust-lang.org/std/index.html#primitives)

#### The Rust Prelude

[Prelude](https://doc.rust-lang.org/std/prelude/index.html)

> The prelude is the list of things that Rust automatically imports into every Rust program. It's kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.

```rust
extern crate std;
use std::prelude::v1::*;
```

The snippet of code above is inserted into the root of every crate in every module, so you do not need to have a `use` statement to bring them into scope in any rust module.

`std::prelude::v1` contents:

* `std::marker::{Copy, Send, Sized, Sync, Unpin}`
    * The marker traits indicate fundamental properties of types.


* `std::ops::{Drop, Fn, FnMut, FnOnce}` 
    * Various operations for both destructors and overloading ().


* `std::mem::drop`
    * a convenience function for explicitly dropping a value.


* `std::boxed::Box` 
    * a way to allocate values on the heap.


* `std::borrow::ToOwned` 
    * The conversion trait that defines to_owned, the generic method for creating an owned type from a borrowed type.


* `std::clone::Clone`
    * the ubiquitous trait that defines clone, the method for producing a copy of a value.


* `std::cmp::{PartialEq, PartialOrd, Eq, Ord }` 
    * The comparison traits, which implement the comparison operators and are often seen in trait bounds.


* `std::convert::{AsRef, AsMut, Into, From}` 
    * Generic conversions, used by savvy API authors to create overloaded methods.


* `std::default::Default`
    * types that have default values.


* `std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}`
    * Iterators of various kinds.


* `std::option::Option::{self, Some, None}` 
    * A type which expresses the presence or absence of a value. This type is so commonly used, its variants are also exported.


* `std::result::Result::{self, Ok, Err}`
    * A type for functions that may succeed or fail. Like Option, its variants are exported as well.


* `std::string::{String, ToString}`
    * heap allocated strings.


* `std::vec::Vec` 
    * a growable, heap-allocated vector.

#### Threading

> An executing Rust program consists of a collection of native OS threads, each with their own stack and local state. Threads can be named, and provide some built-in support for low-level synchronization.

[thread module](https://doc.rust-lang.org/std/thread/index.html)

###### sync module

The [sync](https://doc.rust-lang.org/std/sync/index.html) module contains useful synchronization primitives.

###### atomic module

The [atomic] module provides primitive shared-memory communication between threads, and are the building blocks of other concurrent types

###### mpsc module

The [mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html) module provides message-based communication over channels.

## Common Collections

> Rust's standard collection library provides efficient implementations of the most common general purpose programming data structures. By using the standard implementations, it should be possible for two libraries to communicate without significant data conversion.

Rust has collections types for `Sequences`, `Maps`, `Sets` and `Binary Trees` but for the most part you will only ever need to use Vectors => `Vec` and Maps => `HashMaps`.

#### Vectors

[Vectors](https://doc.rust-lang.org/std/vec/struct.Vec.html)

By and large the `Vec` collection will be the collection type you use most often in rust and has many useful methods you can use.

We have already seen the `Vec` type throughout the workshop but haven't formally talked about them yet.

A Vector is a resizable array and like slices their size cannot be known at compile time.

Vectors can grow and shrink at any time.

A vector can be represented by 3 `parameters`:

* Represent a `pointer` to data
* Number of existing elements aka `length`
* Amount of space that is allocated that is allocated for future elements aka `capacity`

###### Create vector with new method

You can create a vector by calling the `new` method defined in the `Vec<T>` struct

```rust
// create a vector with new method
let numbers = Vec::new();
```

###### Create vector using `vec!` macro

You can also use the `vec!` macro that is defined in the standard library to create a new vector.

The `vec!` macro is defined like this in standard library:

```rust
...................................
macro_rules! vec {
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n)
    );
    ($($x:expr),*) => (
        <[_]>::into_vec(box [$($x),*])
    );
    ($($x:expr,)*) => ($crate::vec![$($x),*])
}
...................................
```

By using the `vec!` macros you can create a new vector using the array syntax that we previously used:

```rust
fn main() {
    let primes_array: [u32; 5] = [2, 3, 5, 7, 11];
    
    for prime in &primes_array {
        println!("{}", prime);
    }
    
    let primes_vec_macro = vec![2, 3, 5, 7, 11];
    for (index, prime) in primes_vec_macro.iter().enumerate() {
        assert_eq!(&primes_array[index], prime);
    }
}
```

Notice that in the `primes_array` we created an array and specified the type and capacity, while in the `primes_vec_macro` we simply passed in the values and the `vec!` macro will infer the type and set a capacity for the vector for us.

[array and vector comparison playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4a84d5ae20b24d0fa4cfb29365d234b4)

##### Vector methods

[Vector Methods](https://doc.rust-lang.org/1.30.0/std/vec/struct.Vec.html#methods)

There are many methods defined in the standard library but we will only go over some of them in this workshop. 

Please read the standard library about them as you need them.

We already showed one of the methods earlier which was `Vec::new()` which was used to create a new empty `Vec<T>` type.

###### Vec `get` method

The [get](https://doc.rust-lang.org/1.30.0/std/vec/struct.Vec.html#method.get) method is much safer than accessing by index which can result in an out of bounds indexing panic during runtime.

```rust
fn main() {
    let primes_first_three = vec![2, 3, 5];
    
    assert_eq!(primes_first_three[2], 5);
    
    // assert_eq!(primes_first_three[4], 0);
    
    assert_eq!(primes_first_three.get(4), None);
}
```

If you were to uncomment the direct vector index it would be a runtime panic.

The `get` is much safer and you can check if a vector has a value without causing a runtime panic like this:

```rust
if let None = primes_first_three.get(4) {
    println!("{}", "Invalid index here!");
}
```

[Vec::get() method playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a1d3f3b7218b9385ca68ce1fbf972d7b)

##### vector used a stack

You can use `Vec<T>` as an efficient stack as well by using the `push()` and `pop()` methods.

Remember that a stack structure is much like a stack of plates when washing dishes. Each plate that you stack on top is `pushed` onto the stack and you don't remove plates from the bottom of the stack instead you `pop` a plate from the top. 

Likewise the `push` method pushes items to the end of the stack while the `pop` method removes elements from the top of the stack.

###### `Vec::push` method

[Vec::push](https://doc.rust-lang.org/1.30.0/std/vec/struct.Vec.html#method.push)

```rust
let mut primes = Vec::new();

primes.push(2);
primes.push(3);
primes.push(5);

let five: usize = 5;

assert_eq!(primes[primes.len() - 1], five);
```

Notice here that we pushed => `2, 3, 5` onto the vector variable `primes` and that just like a stack the top of the item is the last item pushed which in this case is the `5` item.

Notice that we also used the 

###### `Vec::pop` method

[Vec::pop](https://doc.rust-lang.org/1.30.0/std/vec/struct.Vec.html#method.pop)

```rust1
.................................
primes.push(7);
assert_eq!(primes.pop(), Some(7));
```

Notice here that pushed `7` onto the stack and then when we called the `pop` method it gave us the item on the top of the stack which is `7`. 

Also note that the `pop` method returns `Option<T>` type because it is possible that vector is empty in which case the `None` enum value is returned which represents no value.

*This is why we asserted `Some(7)` instead of just `7`.*

###### Iterating through vectors

Remember that `iterators` are used to traverse data structures like stacks.

`Iterators` will traditionally have ` next()` method and you can read more about this pattern in the [Iterator_Pattern](https://en.wikipedia.org/wiki/Iterator_pattern) wikipedia link.

We can iterate through the primes vector variable like this:

```rust
let mut primes = Vec::new();

primes.push(2);
primes.push(3);
primes.push(5);
primes.push(7);
primes.push(11);


let mut iterator = primes.iter();
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&3));
assert_eq!(iterator.next(), Some(&5));
assert_eq!(iterator.next(), Some(&7));
assert_eq!(iterator.next(), Some(&11));
```

Notice here that we called the `next` method. The `next` method is a deref method and in rust it uses the `Deref` trait and traits will be introduced later in the workshop but know that this emulates inheritance in rust.

With that being said it is a method we can use with the vector.

We can also use call the enumerate method and get an index value and use a `for` loop with rust like this:

```rust
let primes2 = vec![2, 3, 5, 7, 11];

for (index, prime) in primes2.iter().enumerate() {
    assert_eq!(&primes[index], prime);
}
```

###### push|pop|next|iter|enumerate playground

[push|pop|next|iter|enumerate playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=45b2057fe2b31242b594bdfe0b2d9737)

###### `iter_mut` method

We can also use the `iter_mut` method to return an iterator that allows us to modify each value in the `Vec<T>` variable

```rust
let mut numbers = vec![1, 2, 3, 4, 5];

for number in numbers.iter_mut() {
    *number *= 2;
}

let factor_of_two = vec![2, 4, 6, 8, 10];

for (index, number) in numbers.iter().enumerate() {
    assert_eq!(number, &factor_of_two[index]);
}
```

#### Hash Maps

The `HashMap` module will not be used as often as the `Vec` Module but it is another important module to be aware of rust.

You will typically use the `HashMap` module when you are in need of Map data structure.

The `HashMap` module is the equivalent of what is known as associative arrays, maps, symbol table, or dictionary in other programming languages.

A `HashMap` like a `Vector` is an [abstract data type](https://en.wikipedia.org/wiki/Abstract_data_type) or in other words a data type that has a defined behavior which is implemented through methods.

This abstract data type (ADT) usually has the following methods in its API:

* addition of a pair to its collection
    * `put|insert|set`


* removal of a pair to its collection
    * `delete|remove`


* modification of an existing pair
    * `insert`

* lookup of value that is associated with a particular key
    * `get`

* check if it is empty
    * `is_empty|isEmpty`


* the number of entries aka `size`
    * `size|len`


* a list of keys in the `symbol table|map|dictionary`
    * `keys`

The Rust API for `HashMap` has these methods and more already defined for us:

###### `insert` method

The [insert](https://doc.rust-lang.org/1.30.0/std/collections/struct.HashMap.html#method.insert) method inserts a key:value pair into the map.

The insert method returns an `Option<T>` type and so will return `None` if no key is present and will will return a `Some` Type and modify current value if it exists.

```rust
use std::collections::HashMap;

fn main() {
    let mut soldiers = HashMap::new();
    
    #[derive(Debug, Clone)]
    struct Soldier {
        name: String,
        age: u32,
        rank: String,
        years_of_service: u32,
    }
    let rambo = Soldier {
        name: String::from("John Rambo"),
        age: 32,
        rank: String::from("SFC"),
        years_of_service: 12,
    };
    soldiers.insert("rambo", &rambo);

    let chuck = Soldier {
        name: String::from("Chuck Norris"),
        age: 34,
        rank: String::from("MSGT"),
        years_of_service: 13,
    };
    soldiers.insert("chuck", &chuck);
    
    assert_eq!(soldiers.get("rambo").unwrap().name, rambo.name);
    assert_eq!(soldiers.get("chuck").unwrap().name, chuck.name);
    
    let mut junior = rambo.clone();
    junior.name = String::from("Son Of Rambo");
    
    soldiers.insert("rambo", &junior);
    assert_eq!(soldiers.get("rambo").unwrap().name, String::from("Son Of Rambo"));
    assert_eq!(soldiers.get("rambo").unwrap().age, 32);
}
```

Here we inserted 2 entries into the soldiers `HashMap` and then used `get` to retrieve our key/value pair.

Notice that we had to call `unwrap` as the call to `get` returns an `Option<&V>` value

[`HashMap` insert playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1b8d08cb20822a68e58fa6cebee66d1a)

Notice that in the 2nd call to insert we set a name value and insert over

###### `remove` method

The `remove` method will remove the key and will return the old value that was set for the key. 

On any subsequent calls to remove, the `None` enum type will be returned.

```rust
...............................................................
assert_eq!(soldiers.remove("rambo").unwrap().name, String::from("Son Of Rambo"));
assert_eq!(soldiers.remove("rambo"), None);
```

Notice that when we tried to call `remove` again we got the `None` type returned

###### `is_empty` method

The [is_empty](https://doc.rust-lang.org/1.30.0/std/collections/struct.HashMap.html#method.is_empty) method returns a boolean value

```rust
...................................
assert_eq!(soldiers.is_empty(), false);
```

###### `keys` method

The [keys](https://doc.rust-lang.org/1.30.0/std/collections/struct.HashMap.html#method.keys) returns a list of keys in an arbitrary order.

```rust
for key in soldiers.keys() {
    assert_eq!(*key, String::from("chuck"));
}
```

###### HashMap playground example

[HashMap Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8c807b7a11cd1a58cf3244495fabbc52)

## Strings

In Rust there is 2 types of strings:

1. `String`
    1. A String is stored as a vector of bytes (`Vec<u8>`), but guaranteed to always be a valid utf-8 sequence. 
    2. String is heap allocated, growable and not null terminated.

2. `&str`
    1. `&str` is a slice (`&[u8]`) that points to a valid utf-8 sequence.
    2. The utf-8 sequence can be used to see the String.

We have already used both of these types of strings in the workshop but haven't formally talked about them in detail.

#### `std::str` module

The [std::str](https://doc.rust-lang.org/std/str/) module explains in detail Structs and function you can use.

```rust
let message = "I am a message!";
```

This is known as a String Literal or String Slice. 

String Literals have a static lifetime which means that the variable `message` has a value for as long as the duration of the running program.

#### `std::string::String` module

We have been using the `String` module methods all throughout the workshop

We have been using the `String::from` method for the most part like this:

```rust
let message = String::from("I am a message!");
```

There are other ways to create a `String` type like this:

```rust
let message = "I am a message!".to_string();
let message: String = "message here".into();
```

###### String concatenation

We can concatenate strings using the `+` but we must understand the borrowing rules at times.

```rust
let word = "word".to_string();
let up = word + " up";
assert_eq!(up, String::from("word up"));
```

###### `std::str` primitive type methods with string

There is methods available in the primitive type `str` that we can use 

```rust
fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    assert_eq!(pangram.len(), 43);
    let mut pangram_reversed_list = "dog lazy the over jumps fox brown quick the".split_whitespace();
    for word in pangram.split_whitespace().rev() {
        assert_eq!(word, pangram_reversed_list.next().unwrap());
    }
}
```

Notice here that we used the [len](https://doc.rust-lang.org/std/primitive.str.html#method.len) method to compute the length and the [split_whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) primitive methods for the string slice.

##### String module methods

There are several methods in the `String` module that you can use one of which is the 

`String::new` method.

We have already looked at the `String::from` method throughout the workshop.

###### Final Strings example

```rust
fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    assert_eq!(pangram.len(), 43);
    let mut pangram_reversed_list = "dog lazy the over jumps fox brown quick the".split_whitespace();
    for word in pangram.split_whitespace().rev() {
        assert_eq!(word, pangram_reversed_list.next().unwrap());
    }
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut builder = String::new();
    for ch in alphabet.split("") {
        builder.push_str(&(ch.to_owned() + "|"));
    }
    assert_eq!(builder, String::from("|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z||"));
}
```

###### String playground

[String playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=040a615a785b4928a3c408cf1b3ff962)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Modules](./modules.md) | [Error Handling](./error_handling.md) →
