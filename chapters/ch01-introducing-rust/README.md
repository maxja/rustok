# Introducing Rust

## Create project using Cargo

Cargo is Rust's package manager and build tool. Actually, it's a multitool that helps you manage dependencies, build your project, and run tests.

```bash
cargo new [project_name]
```

Will Create a new Rust project from a template with the given name, and puts it in a directory with the same name. New file `main.rs` is created with a basic template, with a `main` function that prints "Hello, world!" to the console.

1. `fn` is a prefix for function declaration in Rust.
2. Exclamation mark `!` is a macro invocation.
3. Language respects curly braces `{}` and semicolons `;`.

> The author of the book suggested to create another project, but I will change created previously.

Will create a new function, which would print not only default "Hello, world!", but also same phrase on different other languages, to confirm that rust using utf-8 encoding. By default?

1. `let` is a prefix for variable declaration that can be assigned and initialized right away.
2. Rust language respects underscores in variable names.
3. Arrays (or vectors, or slices, or tuples?) can be initialized with square brackets `[]` and comma-separated values.
4. Strings can be initialized with double quotes `""` and can contain any unicode characters.
5. Imperative `for` loop can be used to iterate over an iterator.
6. Iterator can be produced from array via `.iter()` method.
7. `&` is a reference prefix in Rust. It is used to create a reference to a value, which allows you to access the value without taking ownership of it (only for reading).
8. `println!` is a macro that prints formatted text to the console, where `{}` is a placeholder for a value that will be inserted into the string.

## Questions

* How to point cargo to different template?
* Is it possible to switch it to different codepage for defined string literals?
