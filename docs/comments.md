# Rust Workshop - Comments

## Sections:

* [Comments overview](#comments-overview)
* [Line Comment](#line-comment)
* [Block Comment](#block-comment)
* [Inner Line Document](#inner-line-document)
* [Inner Block Document](#inner-block-document)
* [Outer Line Document](#outer-line-document)
* [Outer Block Document](#outer-block-document)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Comments overview

A comment in computer programming is where the programmer explains or annotates source code for other humans to read.

They are meant to help explain source code for others to gain better understanding of the code.

Comments can also be used as documentation generators much like JavaDoc comments are used to generate documentation for Java programs.

## Line Comment

Rust uses the same type of syntax of C/C++ of line comments such as

```rust
// The data type of number should be i32 here I believe
let number = 5;
```

## Block Comment

Rust also uses block level comments like C/C++ comments such as

```rust
/* Some Documentation on 
 * crazyFunc function
 * 
 */
fn crazyFunc() {}
```

## Inner Line Document

An inner line document in rust are usually used for documentation generation purposes and look like this:

```rust
//! A doc comment 
//! This is the documentation for `todo`
//!
//! # Examples for todo
```

## Inner Block Document

An inner block document can be used for one liner type commments in one block such as:

```rust
/*! My inner block here */
```

## Outer Line Document

An outer line document is another type of comment

```rust
//! My inner line
/// 
//! Important bit here
/// is an outer line document
```

## Outer Block Document

An outer block document can be used for extended block comments (i.e. multiline document comments)

```rust
/**
 * Marcels crazy
 * func can be explained
 */
```

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Data Types](./data_types.md) | [Ownership](./ownership.md) →
