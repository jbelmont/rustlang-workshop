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

content

#### Hash Maps

Content

#### Strings

content

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Modules](./modules.md) | [Error Handling](./error_handling.md) →
