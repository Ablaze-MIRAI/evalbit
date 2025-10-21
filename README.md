# evalbit
A simple logical expression evaluation crate with RPN convertion

![Crates.io Version](https://img.shields.io/crates/v/evalbit?style=flat&logo=rust)

## Quickstart
Add `evalbit` as dependency to your `Cargo.toml`:
```toml
[dependencies]
evalbit = "<desired version>"
```

Then you can use `evalbit` to parse and evaluate expressions like this:
```rust
use evalbit::*;

let expr = "0 | (1 ^ 2 & 3)"
let rpn = parse(expr);

assert_eq!(rpn.to_string(), "0 1 2 3 & ^ |");
assert_eq!(rpn.exec(&[true, true, false, false]), true);

// and you can directly execute
assert_eq!(eval("0 | (1 ^ 2 & 3)", &[true, true, false, false]), true));
```

## Operators
The operator precedences are equal to Rust's ones.

## License
This crate is published under the [ISC License](./LICENSE).
