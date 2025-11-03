# Rakuga (落書き)

A Rust library for rendering TTF/OTF font characters as ASCII art in the terminal.

## Features

- Renders any Unicode character from TTF/OTF fonts as ASCII art
- Simple, user-controlled dimensions - you choose what looks right
- Flexible rendering with separate character set and color mode options:
  - **Character sets:**
    - Density: 70-character grayscale ramp for detailed rendering
    - Blocks: Simple blocks/spaces (minimal, works best with colors)
  - **Color modes:**
    - None: Monochrome (most compatible)
    - ANSI 256-color: 24 grayscale shades using terminal colors
    - ANSI truecolor: Smooth 24-bit grayscale (best quality, requires support)
  - Mix and match: Use density characters with colors for maximum detail!
- Fast rendering using `fontdue`

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rakuga = "0.1.0"
```

Basic example:

```rust
use rakuga::{render_char, render_char_with_mode, RenderMode, CharacterSet, ColorMode};

fn main() {
    // Default mode: density characters, no colors (most compatible)
    let lines = render_char(
        "path/to/font.ttf",
        'あ',  // Character to render
        40,   // Width in terminal characters
        23    // Height in terminal characters
    ).unwrap();

    // Blocks with ANSI truecolor for smooth grayscale
    let lines = render_char_with_mode(
        "path/to/font.ttf",
        'あ',
        40,
        23,
        RenderMode::ANSI_TRUECOLOR  // Pre-configured mode
    ).unwrap();

    // Or mix density characters with colors for maximum detail!
    let lines = render_char_with_mode(
        "path/to/font.ttf",
        'あ',
        40,
        23,
        RenderMode::new(CharacterSet::Density, ColorMode::AnsiTruecolor)
    ).unwrap();

    for line in lines {
        println!("{}", line);
    }
}
```

## Running the Example

The example binary allows you to render any character from the command line:

```bash
# Render hiragana 'あ' at default size (30x30) with default mode
cargo run --example display あ

# Render katakana 'カ' at custom size (roughly square on 2:1 terminals)
cargo run --example display カ 40 23

# Render with blocks and ANSI 256-color mode
cargo run --example display あ 40 23 blocks ansi256

# Render with density characters and ANSI truecolor (best quality!)
cargo run --example display あ 40 23 density truecolor

# Render with custom font
cargo run --example display A 20 20 density none /path/to/font.ttf
```

## Aspect Ratio Considerations

Terminal character cells are typically taller than they are wide (often 2:1 ratio). This library treats your specified width and height as defining visual space, so you can adjust dimensions to match your terminal.

For a visually square character on a 2:1 terminal, use dimensions like 40×20. Experiment to find what looks best on your terminal!

## How It Works

1. **Font Loading**: Uses `fontdue` to parse TTF/OTF font files
2. **Rasterization**: Converts glyphs to grayscale bitmaps at the target size
3. **ASCII Conversion**: Maps bitmap pixels to characters using a 70-character density ramp (from ` ` to `$`) for smooth gradations

## Primary Use Case

Originally designed for displaying Japanese kana (hiragana and katakana) in terminal-based review applications, but works with any Unicode character and font.

## Future Enhancements

- ANSI color output support
- Half-block characters (▀▄█) for doubled vertical resolution
- Better font distribution strategy for examples/tests
- Configurable character density ramps

## License

This project uses Noto Sans JP font for testing, which is licensed under the SIL Open Font License.
