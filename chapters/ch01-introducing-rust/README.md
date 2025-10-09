# Introducing Rust

## Create project using Cargo

### Initialize

Cargo is Rust's package manager and build tool. Actually, it's a multitool that helps you manage dependencies, build your project, and run tests.

```bash
cargo new [project_name]
```

Will Create a new Rust project from a template with the given name, and puts it in a directory with the same name. New file `main.rs` is created with a basic template, with a `main` function that prints "Hello, world!" to the console.

1. `fn` is a prefix for function declaration in Rust.
2. `main` is an entry point for the program.
3. Exclamation mark `!` is a macro invocation.
4. Language respects curly braces `{}` and semicolons `;`.

> The author of the book suggested to create another project, but I will change created previously.

### Explore basic blocks

Will create a new function, which would print not only default "Hello, world!", but also same phrase on different other languages, to confirm that rust using utf-8 encoding. By default?

1. `let` is a prefix for variable declaration that can be assigned and initialized right away.
2. Rust language respects underscores in variable names.
3. Arrays (or vectors, or slices, or tuples?) can be initialized with square brackets `[]` and comma-separated values.
4. Strings can be initialized with double quotes `""` and can contain any unicode characters.
5. Imperative `for` loop can be used to iterate over an iterator.
6. Iterator can be produced from array via `.iter()` method.
7. `&` is a reference prefix in Rust. It is used to create a reference to a value, which allows you to access the value without taking ownership of it (only for reading).
8. `println!` is a macro that prints formatted text to the console, where `{}` is a placeholder for a value that will be inserted into the string.

### Explore condition checking and result capturer

Introduce `print_penguin_profiles` function (from ch 1.5).

1. `\` backslash character after the open double quotes (`"`) is used to disable newline characters in the multi-string literal. This is useful when you want to include a newline character in a string literal without actually starting a new line in the code.
2. String variable have a builtin method `.lines()` which returns an iterator over the lines of the string.
3. To get index of each line, you can use `.enumerate()` method on the iterator returned by `.lines()`. And for accepts a closure that takes two arguments: the index and the line itself.
4. `if` statement doesn't need to have parentheses surrounding the condition.
5. `continue` keyword is used to skip the rest of the loop body and move on to the next iteration. Assume that `break` keyword is used to exit the loop prematurely.
6. Variable declaration can specify the type of the variable, after the colons `:` following the name.
7. `Vec` is a collection type, represents a dynamic array of elements of the same type. Type of the elements specified in triangle brackets `<T>`.
8. `_` is a infer type specifier, which allows the compiler to infer the type of the variable based on assigned value.
9. Functions in Rust can be used as first-class citizens.
10. `cfg!` is a macro to check the configuration of the build environment.
11. `eprintln!` is a macro that prints formatted error messages to the standard error stream, as opposed to `println!` which prints to the standard output stream.
12. Elements of the vector can be accessed using index in square brackets.
13. Variable declaration-assignment can happen in condition block.
14. Builtin method `.parse()` specifies the type of the parsed value in the angle brackets `<T>`.
15. `Ok` is a one of validation-capturer, `Err` is an another one. It allows to skip ill-formatted input.
16. `#[allow(dead_code)]` is an attribute that allows the compiler to ignore the warning about unused code.
17. `cargo run` will compile and run the program with debug information. And `cargo run --release` will compile and run the program with optimizations enabled, ignoring `cfg!(debug_assertions)` block.
18. Comments can be done using `//` for single-line comments and `/* */` for multi-line comments.

### Safety

#### Memory Safety

Introduce enumeration type with fixed list of variants.

1. `#[derive(Debug)]` allowed print macros to access represenation of enumeration.
2. Initialize an empty vector and specify it as mutable, with `mut` declaration prefix.
3. Add new element to vector using `.push()` method.
4. To manually remove variable and it's value `drop` procedure is used.
5. Programm would not compile, as compiler detects attempt to late access to the variable in `println!` macro after it was moved.

#### Thread Safety

Introduce function with threads management, which would lead to race.

1. To be able to use library function `use` is used to import it.
2. `thread::spawn` is used to create a new thread, and accepts closure.
3. As compiler doesn't control the order of execution of threads, it's possible to have race conditions.

#### Buffer Overflow

Introduce function with buffer management, which would lead to overflow.

1. Programm will be compiled!
2. Runtime will detects out of bounds access.

#### In-loop Modification

Introduce function to demonstrate in loop modification with vector modification

#### Wrong Equal Safety

Introduce function to demonstrate wrong equal sign comparison.

1. Compiler expects boolean as a return type of an operation result, but got void type value called unit, and unit has no value.

#### In Memory Storing Control

1. Assigning a literal number to variable, put it on Stack.
2. Assigning a boxed `Box` value to variable, put it on Heap.
3. Boxed variable can be wrapped in Ref Counter `Rc`.
4. Number can be wrapped in Atomic counter `Arc`, and be access protected via `Mutex`.

## Questions

* How to point cargo to different template?
* Is it possible to switch it to different codepage for defined string literals?
* How to handle errors in Rust?
* How to assign value to an enum's values?
* Is `vec!` also macro?
* How to do threading in Rust properly?
* How Rust decides on memory allocation?
* How Rust assigns lifetimes?
