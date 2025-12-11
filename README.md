# Anonymous

A Rust library that provides a macro for creating anonymous structs.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anonymous = "0.1.0"
```

## Usage

You can use the `structx!` macro to create anonymous structs with named fields.

```rust
use anonymous::structx;

fn main() {
    // Create an anonymous struct
    let person = structx! {
        derive(Debug);
        name: String = "Alice".to_string(),
        age: u32 = 30,
    };

    println!("{:?}", person);
    // Output: Anon { name: "Alice", age: 30 }

    // Create another anonymous struct with different fields
    let point = structx! {
        derive(Debug, Clone);
        x: i32 = 10,
        y: i32 = 20,
    };

    println!("{:?}", point);
}
```

## Features

- Create structs on the fly without defining them first.
- Support for `derive` macros.
- Type-safe field access.

## License

MPL-2.0
