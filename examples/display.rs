use rakuga::{render_char, render_char_with_aspect_ratio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <character> [width] [height] [aspect_ratio] [font_path]",
            args[0]
        );
        eprintln!("\nExamples:");
        eprintln!("  {} あ", args[0]);
        eprintln!("  {} カ 30 30", args[0]);
        eprintln!("  {} カ 30 30 1.67", args[0]);
        eprintln!("  {} A 20 20 2.0 /path/to/font.ttf", args[0]);
        std::process::exit(1);
    }

    // Parse character (first arg)
    let character = args[1].chars().next().unwrap_or('?');

    // Parse width (default: 30)
    let width = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30);

    // Parse height (default: 30)
    let height = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(30);

    // Parse aspect ratio (default: None, which means use default 2.0)
    let aspect_ratio: Option<f32> = args.get(4).and_then(|s| s.parse().ok());

    // Parse font path (default: Noto Sans JP)
    let font_path = args
        .get(5)
        .map(|s| s.as_str())
        .unwrap_or("fonts/noto_sans_jp/NotoSansJP-VariableFont_wght.ttf");

    if let Some(ratio) = aspect_ratio {
        println!(
            "Rendering '{}' at {}x{} (aspect ratio {}) using {}",
            character, width, height, ratio, font_path
        );
    } else {
        println!(
            "Rendering '{}' at {}x{} (default aspect ratio) using {}",
            character, width, height, font_path
        );
    }
    println!();

    let result = if let Some(ratio) = aspect_ratio {
        render_char_with_aspect_ratio(font_path, character, width, height, ratio)
    } else {
        render_char(font_path, character, width, height)
    };

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
