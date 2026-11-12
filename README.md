# minirg

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A fast, line-oriented recursive search tool written in Rust, inspired by [ripgrep](https://github.com/BurntSushi/ripgrep). `minirg` recursively searches directories for regular expression patterns while automatically respecting `.gitignore` rules, hidden files, and binary file skips.

---

## Features

- **Smart Directory Traversal:** Powered by the `ignore` crate to automatically honor `.gitignore`, `.ignore`, and global git ignore configurations.
- **Regex Engine:** Built on Rust's `regex` crate for linear-time pattern matching.
- **Zero-Noise Output:** Gracefully skips unreadable or binary files without halting execution.
- **Clean CLI Interface:** Ergonomic argument parsing and auto-generated help menus via `clap`.

---

## Project Structure

```text
minirg/
├── Cargo.toml       # Project manifest and dependencies
├── README.md        # Documentation
└── src/
    ├── main.rs      # Entry point and error handling
    ├── args.rs      # CLI argument definitions (clap)
    └── search.rs    # Directory walking and line-by-line regex matching

```

---

## Installation

### Prerequisites

* [Rust and Cargo](https://rustup.rs/) (edition 2021 or newer)

### Build from Source

1. Clone the repository:
```bash
git clone [https://github.com/bundlab/minirg.git](https://github.com/bundlab/minirg.git)
cd minirg

```


2. Compile the optimized release binary:
```bash
cargo build --release
```


3. *(Optional)* Install locally into your Cargo binary path (`~/.cargo/bin`):
```bash
cargo install --path .
```



---

## Usage

```bash
minirg <PATTERN> [PATH]
```

* `<PATTERN>`: The regular expression pattern to search for.
* `[PATH]`: Optional file or directory to search in (defaults to the current directory `.`).

### Examples

**Search for `fn main` in the current directory:**

```bash
minirg "fn main"
```

**Search for TODO or FIXME comments inside a specific folder:**

```bash
minirg "(TODO|FIXME):" ./src
```

**Search for IPv4 addresses across logs:**

```bash
minirg "\b\d{1,3}(\.\d{1,3}){3}\b" /var/log
```

**View help and version info:**

```bash
minirg --help
```

---

## Roadmap

* [ ] Parallel directory walking and file searching
* [ ] Colored terminal output for matched substrings and file paths
* [ ] Case-insensitive (`-i`) and inverted match (`-v`) CLI flags
* [ ] File type filtering (`-t rust`, `-t py`)

---

## License

This project is licensed under the [MIT License](LICENSE).


---
