# Rakuga (落書き)

A Rust library for rendering TTF/OTF font characters as ASCII art in the terminal.

## Features

- Renders any Unicode character from TTF/OTF fonts as ASCII art
- Automatic terminal character aspect ratio compensation (2:1 height:width)
- Simple binary thresholding for clean output
- Fast rendering using `fontdue`

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rakuga = "0.1.0"
```

Basic example:

```rust
use rakuga::render_char;

fn main() {
    let lines = render_char(
        "path/to/font.ttf",
        'あ',  // Character to render
        30,   // Width in terminal characters
        30    // Height in terminal characters
    ).unwrap();

    for line in lines {
        println!("{}", line);
    }
}
```

## Running the Example

The example binary allows you to render any character from the command line:

```bash
# Render hiragana 'あ' at default size (30x30)
cargo run --example display あ

# Render katakana 'カ' at custom size
cargo run --example display カ 25 25

# Render with custom font
cargo run --example display A 20 20 /path/to/font.ttf
```

## How It Works

1. **Font Loading**: Uses `fontdue` to parse TTF/OTF font files
2. **Rasterization**: Converts glyphs to grayscale bitmaps at the target size
3. **Aspect Ratio Correction**: Accounts for terminal characters being ~2x taller than wide
4. **ASCII Conversion**: Maps bitmap pixels to characters using thresholding

## Primary Use Case

Originally designed for displaying Japanese kana (hiragana and katakana) in terminal-based review applications, but works with any Unicode character and font.

## Future Enhancements

- Grayscale ASCII art using multiple character densities (` .:-=#%@`)
- ANSI color output support
- Better font distribution strategy for examples/tests

## License

This project uses Noto Sans JP font for testing, which is licensed under the SIL Open Font License.
