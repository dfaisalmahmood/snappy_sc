# snappy_sc - Fast Screen Capture Library in Rust

`snappy_sc` is a minimal, fast and efficient screen capture library written in Rust. The library is designed to work across multiple platforms, although it currently only supports Windows. Further platform support is in development.

This library is used as the core component in the Snappy screen capture application, which is still under active development.

## Features

- Cross-platform support (currently Windows only, more platforms coming soon)
- Based on `winapi`
- High-performance screen capture
- Early development stage
- Open for contributions and usage, both for personal and commercial purposes

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. If you don't have them installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

Add `snappy_sc` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
snappy_sc = "0.1.0"
```

Then, include the library in your Rust code:

```rust
extern crate snappy_sc;
```

### Usage

To use the `snappy_sc` library, you can call the primary screen capture function:

```rust
use snappy_sc::{get_focused_display_info, take_screenshot};

fn main() {
    let (display_id, width, height) = get_focused_display_info().unwrap();
    let options = ScreenshotOptions {
        display_id,
        region: None,
        output_format: OutputFormat::Png,
    };
    let output: Vec<u8> = take_screenshot(&options).unwrap();
    // Process the output here

    // for example, save to file
    let mut file = File::create("screenshot.png").unwrap();
    file.write_all(&output).unwrap();
}
```

### Contributing

We welcome contributions from the community! If you'd like to contribute to snappy_sc, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes and commit them to your branch.
4. Open a pull request, describing your changes and why they should be merged.

### License

`snappy_sc` is released under the MIT License. You are free to use, modify, and distribute the library for both personal and commercial purposes.
