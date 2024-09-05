# Rust JSON Parser

This project demonstrates how to work with JSON in Rust using the `serde` library. The program deserializes a JSON string into Rust structs and prints the entire parsed structure.

## Features

- Parses JSON data into Rust structs.
- Prints the parsed data in a human-readable format using the `Debug` trait.

## Example JSON

The program is designed to handle JSON in the following format:

```json
{
  "article": "how to work with json in Rust",
  "author": "Baibhav",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
```

## Usage

### 1. Clone the repository:
```bash
git clone https://github.com/BaibhavTiwari/rust_projects
cd JsonReader
```

### 2. Build the project:
```bash
cargo build
```

### 3. Run the project:
```bash
cargo run
```

This will deserialize the hardcoded JSON data and print the parsed structure in a neatly formatted manner.

### Sample Output

```rust
Parsed Article Struct: Article {
    article: "how to work with json in Rust",
    author: "Baibhav",
    paragraph: [
        Paragraph {
            name: "starting sentences",
        },
        Paragraph {
            name: "body of the paragraph",
        },
        Paragraph {
            name: "end of the paragraph",
        },
    ],
}
```

## Dependencies

- [serde](https://crates.io/crates/serde): A framework for serializing and deserializing Rust data structures.
- [serde_json](https://crates.io/crates/serde_json): Provides JSON support for `serde`.

Add these dependencies to your `Cargo.toml` file:

```toml
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

