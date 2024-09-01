
# Gzip File Compressor

This Rust program compresses a file using gzip compression. It reads from a source file and writes the compressed output to a target file.

## Features

- Compresses a file using the gzip format.
- Prints the size of the source file and the compressed target file.
- Outputs the time taken for the compression process.

## Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/): The Rust programming language.
- [Cargo](https://doc.rust-lang.org/cargo/): The Rust package manager and build system.

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/BaibhavTiwari/rust_projects/
   ```

2. Navigate into the project directory:

   ```sh
   cd rust_projects
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

   This will compile the project and generate an executable in the `target/release` directory.

## Usage

Run the program with the following command:

```sh
cargo run -- <source-file> <target-file>
```

- `Make sure that the file you want to compress must exist in the project folder itself.`

- `<source-file>`: Path to the input file that you want to compress.
- `<target-file>`: Path where the compressed file will be saved.

### Example

```sh
cargo run -- input.txt output.gz
```

This command will read `input.txt`, compress it using gzip, and write the result to `output.gz`.

## Output

The program prints the following information to the console:

- Size of the source file.
- Size of the compressed target file.
- Time taken to perform the compression.

### Sample Output

```
Source len: 123456
Target len 78900
Elapsed : 2.345s
```

## Notes

- Make sure you provide valid file paths for both the source and target files.
- If the target file already exists, it will be overwritten.
