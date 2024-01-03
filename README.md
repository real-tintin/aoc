# Advent of Code

Just for fun and learning.

## 2023

Solved in Rust, see [2023/aoc](2023/aoc).

### Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Build

Simply `cargo build`. Add `--release` for a release build.

### Usage

The build will create an executable in `./target/*/aoc` which enables a cli
e.g., `./target/release/aoc --help`:

```
Usage: aoc [OPTIONS] --input <INPUT> --day <DAY> --part <PART>

Options:
  -i, --input <INPUT>
  -d, --day <DAY>
  -p, --part <PART>
  -v, --verbose
  -h, --help           Print help
```

### Test

Simply `cargo test`.
