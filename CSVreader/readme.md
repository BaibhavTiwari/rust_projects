
# CSV Reader in Rust

This project demonstrates how to read CSV files in Rust using the `csv` crate. It reads a CSV file and prints each record to the console.

## Project Structure

- `src/main.rs`: Contains the main logic to read from a CSV file and handle errors.
- `Cargo.toml`: Manages dependencies and metadata for the Rust project.
- `customers.csv`: Sample CSV file to be read by the program.

## Dependencies

This project uses the following crate:

- `csv`: Provides a fast and flexible CSV parser.

Add it to your `Cargo.toml` file:

```toml
[dependencies]
csv = "1.1"
```

## How It Works

1. The function `read_from_file` takes the path of a CSV file as input and returns a `Result<(), Box<dyn Error>>`.
2. It uses the `csv::Reader` to read the file and iterates over each record.
3. Each record is printed to the console using `println!`.
4. The `main` function calls `read_from_file`, and if an error occurs, it prints the error message.

## Usage

1. Ensure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository:
   ```bash
   git clone https://github.com/BaibhavTiwari/rust_projects
   ```
3. Navigate to the project directory:
   ```bash
   cd CSVreader 
   ```
4. Run the project:
   ```bash
   cargo run