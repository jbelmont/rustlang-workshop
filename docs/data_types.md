# Rust Workshop - Data Types

## Sections:

* [Data Types](#data-types)
* [Primitive Types](#primitive-types)
    * [Boolean](#boolean)
    * [Numeric](#numeric)
    * [textual (string,char)](#textual-\(string,char\))
    * [Never (!)](#never-\(\!\))
* [Sequence Types](#sequence-types)
    * [Tuple](#tuple)
    * [Array](#array)
    * [Slice](#slice)
* [User-defined Types](#user\-defined-types)
    * [Struct](#struct)
    * [Enum](#enum)
    * [Union](#union)
* [Function Types](#function-types)
    * [Function](#function)
    * [Closure](#closure)
* [Pointer Types](#pointer-types)
    * [References](#references)
    * [Raw Pointer](#raw-pointer)
    * [Function Pointers](#function-pointers)
* [Trait Types](#trait-types)
    * [Trait Objects](#trait-objects)
    * [Impl trait](#impl-trait)
* [data types koan](#data-types-koan)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Data Types

> Every variable, item, and value in a Rust program has a type. The type of a value defines the interpretation of the memory holding it and the operations that may be performed on the value.

> Built-in types are tightly integrated into the language, in nontrivial ways that are not possible to emulate in user-defined types. User-defined types have limited capabilities.

Much like other statically types programming language has a type system:

* Primitive types
    * [Boolean](https://doc.rust-lang.org/reference/types/boolean.html)
    * [Numeric](https://doc.rust-lang.org/reference/types/numeric.html)
    * [Textual (String, char)](https://doc.rust-lang.org/reference/types/textual.html)
    * [Never (!)](https://doc.rust-lang.org/reference/types/never.html)

* Sequence Types
    * [Tuple](https://doc.rust-lang.org/reference/types/tuple.html)
    * [Array](https://doc.rust-lang.org/reference/types/array.html)
    * [Slice](https://doc.rust-lang.org/reference/types/slice.html)

* User-defined types
    * [Struct](https://doc.rust-lang.org/reference/types/struct.html)
    * [Enum](https://doc.rust-lang.org/reference/types/enum.html)
    * [Union](https://doc.rust-lang.org/reference/types/union.html)

* Function Types
    * [Function](https://doc.rust-lang.org/reference/types/function-item.html)
    * [Closure](https://doc.rust-lang.org/reference/types/closure.html)

* Pointer Types
    * [References](https://doc.rust-lang.org/reference/types/pointer.html#shared-references-)
    * [Raw Pointer](https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut)
    * [Function Pointers](https://doc.rust-lang.org/reference/types/function-pointer.html)

* Trait Types
    * [Trait Objects](https://doc.rust-lang.org/reference/types/trait-object.html)
    * [Impl trait](https://doc.rust-lang.org/reference/types/impl-trait.html)

## Primitive Types

A primitive type is usually a type that serves a singular purpose such as a boolean which can either be `true` or `false`

#### Boolean

[Boolean type](https://doc.rust-lang.org/reference/types/boolean.html#boolean-type)

> The bool type is a datatype which can be either true or false. The boolean type uses one byte of memory. It is used in comparisons and bitwise operations like &, |, and !.

```rust
fn main() {
    let flag: bool = true;
    let not = false;
    
    if flag {
        println!("flag is {}, not is {}", flag, not);
    }
}
```

[boolean primitive type playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=dccbb15c57b9a0baf42a4a5e46236629)

#### Numeric

There are integer types, and floating point types in rust

###### Unsigned Integer Types

Unsigned integer types consist of positive numbers up to certain limit:

| Unsigned Integer Type | Minimum | Maximum |
| --- | --- | --- | 
| `u8` | 0 | 2<sup>8</sup> - 1 => 255 |
| `u16` | 0 | 2<sup>16</sup> - 1 => 65535 |
| `u32` | 0 | 2<sup>32</sup> - 1 => 4294967295 |
| `u64` | 0 | 2<sup>64</sup> - 1 => 18446744073709551615 |
| `u128` | 0 | 2<sup>8</sup> - 1 => 340282366920938463463374607431768211455 |

*Notice here that the values are whole numbers meaning: 0, ... , <machine_integer_limit>*

###### Signed Integer Types

Signed integer types consist of both positive and negative numbers up to certain limit:

| Signed Integer Type | Minimum | Maximum |
| --- | --- | --- | 
| `i8` | 0 | -(2<sup>7</sup>) - 1, 2<sup>7</sup> - 1 => -128,..., 127 |
| `i16` | 0 | -(2<sup>15</sup>) - 1, 2<sup>15</sup> - 1 => -32768,..., 32767 |
| `i32` | 0 | -(2<sup>31</sup>) - 1, 2<sup>31</sup> - 1 => -2147483648,..., 2147483647 |
| `i64` | 0 | -(2<sup>63</sup>) - 1, 2<sup>63</sup> - 1 => -9223372036854775808,..., 9223372036854775807 |
| `i128` | 0 | -(2<sup>127</sup>) - 1, 2<sup>7</sup> - 1 => -170141183460469231731687303715884105728,..., 170141183460469231731687303715884105727 |

*Notice that the range here is a much larger set so when you create a signed type it will use more space in machine registers.*

###### Floating Point Types

There are f32 and f64 floating point types in rust.

#### Textual (string,char)

Content

#### Never (!)

Content

## Sequence Types

Content

#### Tuple

Content

#### Array

Content

#### Slice

Content

## User-defined Types

Content

#### Struct

Content

#### Enum

Content

#### Union

Content

## Function Types

Content

#### Function

Content

#### Closure

Content

## Pointer Types

Content

#### References

Content

#### Raw Pointer

Content

#### Function Pointers

Content

## Trait Types

Content

#### Trait Objects

Content

#### Impl trait

Content

## data types koan

[data types koan](../koans/src/data_types.rs)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Functions](./functions.md) | [Comments](./comments.md) →
