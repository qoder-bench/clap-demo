# Clap Demo

A Rust command-line argument parsing demo project based on [Clap](https://github.com/clap-rs/clap). Clap is the most powerful and commonly used command-line argument parsing library in the Rust ecosystem.

## Tech Stack

- **Rust**: 1.90
- **Clap**: 4.5.50

## Project Structure

```
clap-demo/
├── src/
│   └── main.rs          # Application entry point, command-line argument definitions
├── Cargo.toml           # Project dependency configuration
├── Justfile             # Just build tool configuration
└── README.md
```

## Features

- Command-line argument parsing
- Help information generation
- Version information display
- Parameter validation

## Quick Start

### Prerequisites

- Rust 1.90 or higher
- Cargo (Rust package manager)

### Installation and Running

```bash
# Build project
cargo build

# Run project (display help information)
cargo run -- --help

# Run project (with arguments)
cargo run -- --name "John Doe"
```

## Command-Line Options

### Available Parameters

- `--name <value>`: Set the name to use (optional)

### Usage Examples

```bash
# Display help information
cargo run -- --help

# Display version information
cargo run -- --version

# Use name parameter
cargo run -- --name "Alice"
```

## References

- [Clap Official Repository](https://github.com/clap-rs/clap)
- [Clap Example Code](https://github.com/clap-rs/clap/tree/master/examples)
- [Clap Documentation](https://docs.rs/clap)
