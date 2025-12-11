# Anonymous

A Rust library that provides macros for creating anonymous structs, enums, and trait implementations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anonymous = "0.3.1"
```

## Usage

### Anonymous Structs (`structx!`)

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
}
```

### Anonymous Enums (`enumx!`)

You can use the `enumx!` macro to define ad-hoc enums.

```rust
use anonymous::enumx;

fn main() {
    // Define a type that can be i32 or String
    // Note: .into() is required for conversion
    let value: enumx!(i32 | String) = 10.into();

    match value {
        // Matches based on the order of types
        enumx::Enum2::V1(v) => println!("Int: {}", v),
        enumx::Enum2::V2(s) => println!("String: {}", s),
    }
}
```

### Anonymous Trait Implementations (`implx!`)

You can use the `implx!` macro to create an anonymous struct that implements a specific trait.

```rust
use anonymous::implx;

trait Greeter {
    fn greet(&self);
}

fn main() {
    let p = implx! {
        struct { name: String = "Guest".to_string() }
        impl Greeter {
            fn greet(&self) {
                println!("Hello, {}", self.name);
            }
        }
    };
    
    p.greet();
}
```

## Features

- **`structx!`**: Create structs on the fly with named fields and derive support.
- **`enumx!`**: Define anonymous enums (sum types) inline.
- **`implx!`**: Create anonymous structs implementing a trait with internal state.

## License

MPL-2.0
