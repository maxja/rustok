# Foundations

## Type specification

1. Can be infered from the value assigned to it;
2. Type can be detailed after value literal, as `10i32`;
3. Can be explicitly defined using the `let` keyword and type specifier after the variable name, and colon `:`;
4. Function's return type can be specified using the `->` syntax after the parameter list;
5. Function can have auto-return as a last statement declared in a block;
6. Auto-return statement can't end with a semicolon `;`;
7. Double quoted strings used generally for strings and single quoted characters for characters;

## Numbers

1. Rust uses infix notation;
2. Operands should be same type, there is no implicit conversion;
3. Specific numeric type can have it's own set of methods, like `.round()` for floating point numbers;
4. Floating point types started with `f` prefix and integer with `i`;
5. Number can be formatted in print macro via dot specifier, like `{:.4}` or `{:.4e}`;
6. Numbers with base different then `10` can be specified using `0x` for hexadecimal, `0o` for octal, and `0b` for binary;
7. Numbers can be printed with different base, like `{:x}` for hexadecimal, `{:o}` for octal, and `{:b}` for binary;

## Questions

1. There is an operator overload, can I do mine?
2. What is the other options to format numbers?
