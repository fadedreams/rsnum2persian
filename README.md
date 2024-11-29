### Num2Persian

A Rust library for converting numbers to their Persian representation.

## Features

- Converts numbers to Persian text.
- Supports ordinal numbers (e.g., "fifth" in Persian).
- Handles large numbers up to trillions.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsnum2persian = "0.1.0"
```

#### Usage
```Rust
use rsnum2persian;
fn main() {
    let mut level = 0;
    println!("{}", persian_number_converter::num_to_persian(5678, &mut level, false));
    // Output: پنج هزار و ششصد و هفتاد و هشت
}
```

#### Ordinal Numbers
```Rust
use rsnum2persian;
fn main() {
    let mut level = 0;
    println!("{}", persian_number_converter::num_to_persian(5678, &mut level, true));
    // Output: پنج هزار و ششصد و هفتاد و هشتم
}
```

#### Running Examples
This library includes an example demonstrating its usage:
```bash
cargo run --example main
```
