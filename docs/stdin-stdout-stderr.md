# Rust Workshop - stdin, stdout, and stderr

## Sections:

* [Formatting strings and printing](#formatting-strings-and-printing)
* [format!](#format\!-macro)
* [print!](#print\!-macro)
* [println!](#println\!-macro)
* [eprint!](#eprint\!-macro)
* [eprintln!](#eprint\!-macro)
* [positional arguments](#positional-arguments)
* [text alignment](#text-alignment)
* [exercise](#exercise)
* [Bread Crumb Navigation](#bread-crumb-navigation)

## Formatting strings and printing

Formatting strings is a common occurrence when dealing with characters and strings in programming.

If you are creating a script in the command line then you might want to add some padding and justify text.

You can do this with rust by using some of the macros defined in the [std::fmt](https://doc.rust-lang.org/std/fmt):

* [format!](https://doc.rust-lang.org/std/macro.format.html)
    * Creates a String using interpolation of runtime expressions.
    * write formatted text to String

* [print!](https://doc.rust-lang.org/std/fmt/#print)
    * emits output to stdout (console) io:stdiou

* [println!](https://doc.rust-lang.org/std/macro.println.html)
    * same as print! but a newline is appended.

* [eprint!](https://doc.rust-lang.org/std/fmt/#eprint)
    * The eprint! macro is identical to print! and println! except *eprint!* emits its output to standard error.

* [eprintln!](https://doc.rust-lang.org/std/macro.eprintln.html)
    * same as eprint! but a newline is appended.

#### format! macro

[format! playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=404cca813e347f58b6b442a1aa890e1e)

```rust
fn main() {
    let language = "rust";
    let workshop = "workshop";

    let out = format!("{} {} {}", String::from("hello"), language, workshop);
    println!("{}", out);
}
```

#### print! macro

[print! playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0e33f47d8dd7966f25148f246c60d812)

#### println! macro

[println! playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c847dec8d6df6013b9b54099405aac07)

#### eprint! macro

[eprint! playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=758ea8e092566fc4129730f7d8c39ac0)

#### eprintln! macro

[eprintln! playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c122370aea3d1040d1a6233bc0e7331b)

#### positional arguments

You can use positional arguments in the *std::fmt* module

[Positional parameters](https://doc.rust-lang.org/std/fmt/#positional-parameters)

> Each formatting argument is allowed to specify which value argument it's referencing, and if omitted it is assumed to be "the next argument". For example, the format string {} {} {} would take three parameters, and they would be formatted in the same order as they're given. The format string {2} {1} {0}, however, would format arguments in reverse order.

[positional argument playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8a7f22da3006013a008034ad3fa9d0c2)

#### text alignment

Text can be aligned or padded a certain way by assigning in the arguments to the std::fmt module macros

[text alignment playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=37ca1d2ad04c66be91baf746fcf36cb1)

#### exercise

Please fix the issue you see in the following [playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=299c6ac05558f1715a5c3797372fef1d)

## Bread Crumb Navigation
_________________________

Previous | Next
:------- | ---:
← [Variables](./variables.md) | [Control Flow](./control_flow.md) →
