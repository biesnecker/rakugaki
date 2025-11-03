use rakugaki::{render_char_with_mode, CharacterSet, ColorMode, RenderMode};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <character> [width] [height] [charset] [colors] [font_path]",
            args[0]
        );
        eprintln!("\nExamples:");
        eprintln!("  {} あ", args[0]);
        eprintln!("  {} カ 40 23  # Roughly square on 2:1 terminals", args[0]);
        eprintln!("  {} あ 40 23 blocks ansi256  # 256-color blocks", args[0]);
        eprintln!(
            "  {} あ 40 23 density truecolor  # Colored density chars",
            args[0]
        );
        eprintln!("  {} A 20 20 density none /path/to/font.ttf", args[0]);
        eprintln!("\nCharacter sets:");
        eprintln!("  density    - 70-character density ramp (most detail, default)");
        eprintln!("  blocks     - Simple blocks/spaces (minimal, works best with colors)");
        eprintln!("\nColor modes:");
        eprintln!("  none       - No colors, monochrome (default, most compatible)");
        eprintln!("  ansi256    - ANSI 256-color grayscale (24 shades)");
        eprintln!("  truecolor  - ANSI truecolor grayscale (smoothest, requires support)");
        eprintln!("\nNote: Adjust width and height to match your terminal's aspect ratio.");
        eprintln!("For terminals with 2:1 cells, use height ≈ width/2 for square chars.");
        std::process::exit(1);
    }

    // Parse character (first arg)
    let character = args[1].chars().next().unwrap_or('?');

    // Parse width (default: 30)
    let width = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30);

    // Parse height (default: 30)
    let height = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(30);

    // Parse character set (default: density)
    let charset = args
        .get(4)
        .map(|s| match s.as_str() {
            "blocks" => CharacterSet::Blocks,
            _ => CharacterSet::Density,
        })
        .unwrap_or(CharacterSet::Density);

    // Parse color mode (default: none)
    let colors = args
        .get(5)
        .map(|s| match s.as_str() {
            "ansi256" => ColorMode::Ansi256,
            "truecolor" => ColorMode::AnsiTruecolor,
            _ => ColorMode::None,
        })
        .unwrap_or(ColorMode::None);

    let mode = RenderMode::new(charset, colors);

    // Parse font path (default: Noto Sans JP)
    let font_path = args
        .get(6)
        .map(|s| s.as_str())
        .unwrap_or("fonts/noto_sans_jp/NotoSansJP-VariableFont_wght.ttf");

    let charset_str = match charset {
        CharacterSet::Density => "density",
        CharacterSet::Blocks => "blocks",
    };

    let colors_str = match colors {
        ColorMode::None => "none",
        ColorMode::Ansi256 => "ansi256",
        ColorMode::AnsiTruecolor => "truecolor",
    };

    println!(
        "Rendering '{}' at {}x{} (charset: {}, colors: {}) using {}",
        character, width, height, charset_str, colors_str, font_path
    );
    println!();

    let result = render_char_with_mode(font_path, character, width, height, mode);

    match result {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
