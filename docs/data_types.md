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
    * [Function](#function-item-types)
    * [Closure](#closure-types)
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

```rust
fn main() {
    let unsigned_int8: u8 = 255;
    
    let unsigned_int16: u16 = 65535;
    
    let unsigned_int32: u32 = 4294967295;
    
    let unsigned_int64: u64 = 18446744073709551615;
    
    let unsigned_int128: u128 = 340282366920938463463374607431768211455;
    
    println!("unsigned_int8: {}, unsigned_int16: {}", unsigned_int8, unsigned_int16);
    println!("unsigned_int32: {}, unsigned_int64: {}", unsigned_int32, unsigned_int64);
    println!("unsigned_int128: {}", unsigned_int128);
    
    // this will cause a panic by rust compiler since it is integer overflow
    // panicked at 'attempt to add with overflow'
    // MAX INT 8 VALUE OF 255 + 1 = 256 => integer overflow
    // println!("integer overflow for unsigned_int8: {}", unsigned_int8 + 1);
}
```

[Signed Integer Types Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=aff6912737ce6dda9bc67e99104ab335)

###### Floating Point Types

There are f32 and f64 floating point types in rust.

The IEEE 754 specification defines single precision floating point format which is usually 32 bits in computer memory

[Single-precision floating-point format Wikipedia Post](https://en.wikipedia.org/wiki/Single-precision_floating-point_format)

as well as [Double-precision floating-point format
](https://en.wikipedia.org/wiki/Double-precision_floating-point_format) which usually occupies 64 bits in computer memory.

```rust
use std::f32;
use std::f64;

fn main() {
    // smallest postive number
    let min = f32::MIN_POSITIVE;
    // largest possible value for f32
    let max = f32::MAX;
    
    println!("{}", min);
    println!("{}", max);
    
    let maximumf32: f32 = 0.00000000000000000000000000000000000001175494455;
    // a digit of precision is lost here since we reached our max
    // which is 47 digits long
    println!("{}", maximumf32);
    
    let minimum = f64::MIN_POSITIVE;
    println!("{}", minimum);
    
    let maximum = f64::MAX;
    println!("{}", maximum);
    
    let maximumf64: f64 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014;
    // 327 digits long
    println!("{}", maximumf64);
}
```

[Floating Point Types Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b05a82ec173cc8c7f9ec81475faa46bf)

###### Machine dependent integer types

[Machine dependent integer types](https://doc.rust-lang.org/reference/types/numeric.html#machine-dependent-integer-types)

> The usize type is an unsigned integer type with the same number of bits as the platform's pointer type. It can represent every memory address in the process.

> The isize type is a signed integer type with the same number of bits as the platform's pointer type. The theoretical upper bound on object and array size is the maximum isize value. This ensures that isize can be used to calculate differences between pointers into an object or array and can address every byte within an object along with one byte past the end.

```rust
fn main() {
    let number: isize = -5000050;
    let number2: usize = 300;
    println!("number = {}\nnumber2 = {}", number, number2);
}
```

[Machine dependent integer types playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5319aa16b20108a4a4e6eca3df2c862a)


#### Textual (string,char)

[textual types specification](https://doc.rust-lang.org/reference/types/textual.html)

> The types char and str hold textual data.

> A value of type char is a Unicode scalar value (i.e. a code point that is not a surrogate), represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF range. A char is effectively a UCS-4 / UTF-32 string.

> A value of type str is a Unicode string, represented as an array of 8-bit unsigned bytes holding a sequence of UTF-8 code points. Since str is a dynamically sized type, it is not a first-class type, but can only be instantiated through a pointer type, such as &str.

[stackoverflow post on difference between unicode code point and unicode scalar](https://stackoverflow.com/questions/48465265/what-is-the-difference-between-unicode-code-points-and-unicode-scalars)

```rust
fn main() {
    let max_char = std::char::MAX;
    println!("{}", max_char);
    
    let chars = "abcdefghi";
    println!("{}", chars);
    
    let strings = String::from("abcdefghi");
    println!("{}", strings);
    
    let word1: &'static str = "abc";
    let word2: &'static str = "def";
    // string concatenation is not as easy as you think here
    // these are str bute you have to transfer ownership
    let word3 = word1.to_owned() + &word2;
    println!("{}", word3);
    
    let word3 = String::from("123");
    let word4 = String::from("456");
    let word5 = word3 + &word4;
    println!("{}", word5);
    
    let letter: char = 'a';
    // invalid index here
    // println!("{}", &letters[0..1]);
    
    // slicing strings
    let word: &'static str = "bird bird is the word";
    println!("{}", &word[0..4]);
    
    let letters: &'static str = "abcdefghijklmnopqrstuvwxyz";
    for l in letters.chars() {
        println!("{}", l);
    }
}
```

[textual types playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f1e40a486bbf1fceb67cacd4338df4b9)

#### Never (!)

[Never (!) specification](https://doc.rust-lang.org/reference/types/never.html)

> The never type ! is a type with no values, representing the result of computations that never complete. Expressions of type ! can be coerced into any other type.

Please read this excellent [blog post on never pattern](http://smallcultfollowing.com/babysteps/blog/2018/08/13/never-patterns-exhaustive-matching-and-uninhabited-types-oh-my/) by [Niko Matsakis](https://twitter.com/nikomatsakis)

## Sequence Types

According to [Wikipedia](https://en.wikipedia.org/wiki/Sequence):

> In mathematics, a sequence is an enumerated collection of objects in which repetitions are allowed. Like a set, it contains members (also called elements, or terms). The number of elements (possibly infinite) is called the length of the sequence. Unlike a set, the same elements can appear multiple times at different positions in a sequence, and order matters. Formally, a sequence can be defined as a function whose domain is either the set of the natural numbers (for infinite sequences) or the set of the first n natural numbers (for a sequence of finite length n).

[Computing Definition](https://en.wikipedia.org/wiki/Sequence#Computing)

> In computer science, finite sequences are called lists. Potentially infinite sequences are called streams. Finite sequences of characters or digits are called strings.

#### Tuple

[Tuple Specification](https://doc.rust-lang.org/reference/types/tuple.html)

> A tuple type is a heterogeneous product of other types, called the elements of the tuple. It has no nominal name and is instead structurally typed.

> Tuple types and values are denoted by listing the types or values of their elements, respectively, in a parenthesized, comma-separated list.

> Because tuple elements don't have a name, they can only be accessed by pattern-matching or by using N directly as a field to access the Nth element.

```rust
fn main() {
    
    let (a, b, c, d) = (3, "one", 2.5, String::from("hello"));
    
    println!("a = {}\nb = {}\nc = {}\nd = {}", a, b, c, d);
    
    struct Point {
        x: u32,
        y: u32,
    }
    
    let tuple = (
        1,
        "two",
        2.5,
        Point {
            x: 3,
            y: 5,
        }
    );
    assert_eq!(tuple.0, 1);
    assert_eq!(tuple.1, "two");
    assert_eq!(tuple.2, 2.5);
    assert_eq!(tuple.3.x == 3 && tuple.3.y == 5, true);
}
```

[tuple types playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=53c1d042c65c730578b94b482b3c7575)

#### Array

[Array types specification](https://doc.rust-lang.org/reference/types/array.html)

> An array is a fixed-size sequence of N elements of type T. The array type is written as [T; N]. The size is an expression that evaluates to a usize.

```rust
fn main() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let mut sum: i32 = 0;
    for n in &numbers {
        sum += n;
    }
    println!("{}", sum);
    
    let boxed_array: Box<[i32]> = Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
   
    // Lines 13 - 15 don't work as you can't iterate over Box type
    // Box type doesn't implement an iterator
    // for b in boxed_array {
    //     println!("{}", b);
    // }
    
    assert_eq!(boxed_array[4], 5, "Should get 5 here!");
}
```

[Array types playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7b7df73a2dbfe10c4daadf5adbf4c4a9)

#### Slice

[Slice types specification](https://doc.rust-lang.org/reference/types/slice.html)

> A slice is a dynamically sized type representing a 'view' into a sequence of elements of type T. The slice type is written as [T].

```rust
fn main() {
    // This is a heap-allocated array that is being coerced
    // into a slice
    let boxed_array: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);
    
    assert_eq!(boxed_array[2], 3);
    
    // a shared slice into an array
    // this gets elements 0, 1, 2 into slice var
    let slice: &[i32] = &boxed_array[0..3];
    assert_eq!(slice[2], 3);
}
```

[Slice types playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4c1978754e6efef4ea6ee6125f757b6b)

## User-defined Types

User defined types let a programmer define attributes and particular state.

#### Struct

[Struct types specification](https://doc.rust-lang.org/reference/types/struct.html#structtype)

> A struct type is a heterogeneous product of other types, called the fields of the type.

> New instances of a struct can be constructed with a struct expression.

> The memory layout of a struct is undefined by default to allow for compiler optimizations like field reordering, but it can be fixed with the repr attribute. In either case, fields may be given in any order in a corresponding struct expression; the resulting struct value will always have the same memory layout.

```rust
fn main() {
    struct Soldier<'a, 'b> {
        name: &'a str,
        rank: &'b str,
        experience: u32,
    }
    
    let rambo = Soldier {
        name: "John Rambo",
        rank: "SFC",
        experience: 12,
    };
    
    assert_eq!(rambo.name, "John Rambo");
    assert_eq!(rambo.rank, "SFC");
    assert_eq!(rambo.experience, 12);
}
```

[Struct types playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9b7993f44fc0bc3f6acb8b56f3d35ef4)

#### Enum

[Enumerated Types specification](https://doc.rust-lang.org/reference/types/enum.html)

> An enumerated type is a nominal, heterogeneous disjoint union type, denoted by the name of an enum item.

> An enum item declares both the type and a number of variants, each of which is independently named and has the syntax of a struct, tuple struct or unit-like struct.

> New instances of an enum can be constructed in an enumeration variant expression.

```rust
fn main() {
    enum Army {
        Soldier {
            identifier: char
        }
    }
    let soldier = Army::Soldier {
        identifier: 'v',
    };
    
    let ch = match soldier {
       Army::Soldier { identifier: 'v' } => "matched v",
       _ => "not matched"
    };
    assert_eq!(ch, "matched v");
}
```

[Enum playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4617d7938a69074a718d445a7a9e40eb)

#### Union

[Union specification](https://doc.rust-lang.org/reference/items/unions.html)

> A union declaration uses the same syntax as a struct declaration, except with union in place of struct.

> The key property of unions is that all fields of a union share common storage. As a result writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field.

```rust
fn main() {
    union Soldier {
        age: u32,
    }
    
    let rambo = Soldier {
        age: 32,
    };
    assert_eq!(unsafe {rambo.age}, 32);
}
```

*Notice that I had to wrap read to the age field in unsafe blocks.*

*error[E0133]: access to union field is unsafe and requires unsafe function or block*

[Union playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b9afd8d57f7628fd620222f9d9030077)

## Function Types

#### Function Item Types

[Function Item specification](https://doc.rust-lang.org/reference/types/function-item.html)

> When referred to, a function item, or the constructor of a tuple-like struct or enum variant, yields a zero-sized value of its function item type. That type explicitly identifies the function - its name, its type arguments, and its early-bound lifetime arguments (but not its late-bound lifetime arguments, which are only assigned when the function is called) - so the value does not need to contain an actual function pointer, and no indirection is needed when the function is called.

```rust
fn main() {
   fn calculation<T> () {
       println!("2 + 2 is {}", 2 + 2)
   }
   
   calculation::<i32>();
}
```

#### Closure types

[Closure types specification](https://doc.rust-lang.org/reference/types/closure.html)

> A closure expression produces a closure value with a unique, anonymous type that cannot be written out. A closure type is approximately equivalent to a struct which contains the captured variables. For instance, the following closure:

```rust
fn main() {
    let compute = |numbers: std::vec::Vec<u32>| {
        numbers
        .iter()
        .fold(0, |sum, val| sum + val) / numbers.len() as u32
    };
    
    println!("{}", compute(vec![1, 2, 3, 4, 5]));
}
```

[Closures playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d37d5ebed6985456d0f2fbe844e84c5c)

## Pointer Types

[Pointer Wikipedia Definition](https://en.wikipedia.org/wiki/Pointer_%28computer_programming%29)

> In computer science, a pointer is a programming language object that stores the memory address of another value located in computer memory. A pointer references a location in memory, and obtaining the value stored at that location is known as dereferencing the pointer. As an analogy, a page number in a book's index could be considered a pointer to the corresponding page; dereferencing such a pointer would be done by flipping to the page with the given page number and reading the text found on that page. The actual format and content of a pointer variable is dependent on the underlying computer architecture.

[Pointer Types specification](https://doc.rust-lang.org/reference/types/pointer.html)

> All pointers in Rust are explicit first-class values. They can be moved or copied, stored into data structs, and returned from functions.

#### References

[References specification](https://doc.rust-lang.org/reference/types/pointer.html#shared-references-)

> These point to memory owned by some other value. When a shared reference to a value is created it prevents direct mutation of the value. Interior mutability provides an exception for this in certain circumstances. As the name suggests, any number of shared references to a value may exist. A shared reference type is written &type, or &'a type when you need to specify an explicit lifetime. Copying a reference is a "shallow" operation: it involves only copying the pointer itself, that is, pointers are Copy. Releasing a reference has no effect on the value it points to, but referencing of a temporary value will keep it alive during the scope of the reference itself.

#### Raw Pointer

[Raw Pointer specification](https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut)

> Raw pointers are pointers without safety or liveness guarantees. Raw pointers are written as *const T or *mut T, for example *const i32 means a raw pointer to a 32-bit integer. Copying or dropping a raw pointer has no effect on the lifecycle of any other value. Dereferencing a raw pointer is an unsafe operation, this can also be used to convert a raw pointer to a reference by reborrowing it (&* or &mut *). Raw pointers are generally discouraged in Rust code; they exist to support interoperability with foreign code, and writing performance-critical or low-level functions.


*Note that the Rust specification explicitly mentions that Raw Pointers are discouraged in Rust so take heed.*

*A lot of this is due to the inherent security flaw that occurs when you deallocate pointers when the memory address isn't correct.*

Here is a good [stack overflow post on the dangers of pointers](https://stackoverflow.com/questions/4705550/dangers-of-pointers)

```rust
fn main() {
    let number = 15;
    
    let raw_pointer = &number as *const i32;
    
    let mut num_mut = 25;
    
    let raw_mut = &mut num_mut as *mut i32;
    
    println!(
        "number = {}\nraw_pointer = {:?}\nnum_mut = {}\nraw_mut = {:?}",
        number,
        raw_pointer,
        num_mut,
        raw_mut,
    );
    
    let number2: u32 = 5;
    let raw = &number2 as *const u32;
    println!("{:?}", raw);
}
```

[Raw Pointer playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b82cd4f4440f11482afee84b59be0d3a)

#### Function Pointers

[Function pointer types specification](https://doc.rust-lang.org/reference/types/function-pointer.html)

> Function pointer types, written using the fn keyword, refer to a function whose identity is not necessarily known at compile-time. They can be created via a coercion from both function items and non-capturing closures.

```rust
fn main() {
    fn sum(numbers: &[i32]) -> i32 {
        let mut sum = 0;
        for n in numbers.iter() {
            sum += n;
        }
        sum
    }
    
    let mut summation = sum(&[1, 2, 3, 4, 5]);
    assert_eq!(summation, 15);
    type MathOp = fn(&[i32]) -> i32;
    let math: MathOp = sum;
    summation = math(&[5, 6, 7, 8, 9, 10]);
    println!("{}", summation);
}
```

[Function pointers playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a294fdb229fbe7d1373d900078cb610b)

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
