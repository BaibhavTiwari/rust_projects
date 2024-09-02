# FileDecompressor

This is a simple Rust program that extracts files from a ZIP archive. The program reads the archive, iterates over the files inside, and extracts each file to the specified output path.

## Features

- Extracts files from a ZIP archive.
- Maintains file structure and permissions (on Unix-based systems).
- Displays comments associated with individual files in the archive.
- Automatically creates directories as needed.

## Usage

To use this program, you need to provide the path to the ZIP file as an argument when running the executable.

### Example

```sh
cargo run -- <filename.zip>
```

Replace `<filename.zip` with the file name.

`keep the file in the same folder as well.`

### Output

The program will print out the following information for each file in the archive:

- The path where the file is extracted.
- The size of the extracted file in bytes.
- Any comment associated with the file in the archive.

## Dependencies

- [zip](https://docs.rs/zip/) - A library to read and write ZIP archives.

## Building and Running

1. **Clone the repository**:
    ```sh
    git clone https://github.com/BaibhavTiwari/rust_projects
    cd FileDecompressor
    ```

2. **Build the project**:
    ```sh
    cargo build --release
    ```

3. **Run the program**:
    ```sh
    cargo run -- <filename.zip>
    ```

## Platform Support

- **Unix-based systems**: The program maintains the original file permissions from the ZIP archive.
- **Other platforms**: The program will extract files without preserving Unix-specific permissions.
