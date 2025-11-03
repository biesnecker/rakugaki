use fontdue::Font;
use std::path::Path;

/// Renders a single Unicode character as ASCII art within the given dimensions.
///
/// # Arguments
/// * `font_path` - Path to the TTF/OTF font file
/// * `codepoint` - The Unicode character to render
/// * `width` - Target width in terminal characters
/// * `height` - Target height in terminal characters
///
/// # Returns
/// A vector of strings, one per line, representing the ASCII art
///
/// # Note
/// The width and height define the visual space for rendering. Users should
/// adjust these dimensions to account for their terminal's character cell
/// aspect ratio. For example, if terminal cells are 2:1 (height:width),
/// a visually square character would use dimensions like 40×20.
pub fn render_char(
    font_path: impl AsRef<Path>,
    codepoint: char,
    width: usize,
    height: usize,
) -> Result<Vec<String>, String> {
    // Load font file
    let font_data = std::fs::read(font_path).map_err(|e| format!("Failed to read font: {}", e))?;
    let font = Font::from_bytes(font_data.as_slice(), fontdue::FontSettings::default())
        .map_err(|e| format!("Failed to parse font: {}", e))?;

    // Render to fill the visual space defined by width × height.
    // We treat the dimensions as defining a square-pixel space and let the user
    // choose dimensions that look right for their terminal.

    // Use the larger dimension to render at good quality
    let font_size = (width.max(height) as f32) * 1.2;

    // Rasterize the glyph
    let (metrics, bitmap) = font.rasterize(codepoint, font_size);

    // Convert bitmap to ASCII using grayscale character density mapping
    bitmap_to_ascii(&bitmap, metrics.width, metrics.height, width, height)
}

/// Converts a grayscale bitmap to ASCII art using character density mapping
fn bitmap_to_ascii(
    bitmap: &[u8],
    bitmap_width: usize,
    bitmap_height: usize,
    target_width: usize,
    target_height: usize,
) -> Result<Vec<String>, String> {
    if bitmap.is_empty() {
        return Ok(vec![" ".repeat(target_width); target_height]);
    }

    // Grayscale character ramp ordered by visual density (lightest to darkest)
    // This ramp provides smooth gradations for better character rendering
    const DENSITY_RAMP: &[char] = &[
        ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_',
        '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n',
        'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p',
        'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
    ];

    let mut result = Vec::with_capacity(target_height);

    // Simple nearest-neighbor sampling from bitmap to target dimensions
    for row in 0..target_height {
        let mut line = String::with_capacity(target_width);

        for col in 0..target_width {
            // Map target position to bitmap position
            let bmp_x = (col * bitmap_width) / target_width.max(1);
            let bmp_y = (row * bitmap_height) / target_height.max(1);

            let idx = bmp_y * bitmap_width + bmp_x;
            let pixel = if idx < bitmap.len() { bitmap[idx] } else { 0 };

            // Map pixel intensity (0-255) to character density
            // Higher pixel values (lighter in fontdue's output) = denser characters
            let density_idx = ((pixel as usize) * (DENSITY_RAMP.len() - 1)) / 255;
            let ch = DENSITY_RAMP[density_idx];
            line.push(ch);
        }

        result.push(line);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_to_ascii_empty() {
        let result = bitmap_to_ascii(&[], 0, 0, 5, 3).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "     ");
    }

    #[test]
    fn test_bitmap_to_ascii_simple() {
        // 2x2 bitmap with a simple pattern (max brightness and dark)
        let bitmap = vec![255, 0, 0, 255];
        let result = bitmap_to_ascii(&bitmap, 2, 2, 2, 2).unwrap();
        assert_eq!(result.len(), 2);
        // 255 maps to darkest char ('$'), 0 maps to lightest (' ')
        assert_eq!(result[0].chars().next().unwrap(), '$');
        assert_eq!(result[0].chars().nth(1).unwrap(), ' ');
        assert_eq!(result[1].chars().next().unwrap(), ' ');
        assert_eq!(result[1].chars().nth(1).unwrap(), '$');
    }

    #[test]
    fn test_render_hiragana_a() {
        // Test rendering hiragana 'あ' (a)
        let font_path = "fonts/noto_sans_jp/NotoSansJP-VariableFont_wght.ttf";

        // Skip test if font file doesn't exist
        if !std::path::Path::new(font_path).exists() {
            eprintln!("Skipping test - font not found at {}", font_path);
            return;
        }

        let result = render_char(font_path, 'あ', 20, 20);
        assert!(result.is_ok(), "Failed to render: {:?}", result.err());

        let lines = result.unwrap();
        assert_eq!(lines.len(), 20, "Expected 20 lines of output");

        // Each line should be 20 characters wide
        for line in &lines {
            assert_eq!(line.len(), 20, "Line width should be 20 chars");
        }

        // The output should contain some non-space characters (the glyph)
        let total_chars: String = lines.join("");
        let non_space_count = total_chars.chars().filter(|c| *c != ' ').count();
        assert!(
            non_space_count > 0,
            "Should have some visible character data"
        );

        // Print the result for visual inspection
        println!("\nRendered 'あ' (hiragana a) at 20x20:");
        for line in &lines {
            println!("{}", line);
        }
    }

    #[test]
    fn test_render_katakana_ka() {
        // Test rendering katakana 'カ' (ka)
        let font_path = "fonts/noto_sans_jp/NotoSansJP-VariableFont_wght.ttf";

        if !std::path::Path::new(font_path).exists() {
            eprintln!("Skipping test - font not found at {}", font_path);
            return;
        }

        let result = render_char(font_path, 'カ', 15, 15);
        assert!(result.is_ok());

        let lines = result.unwrap();
        assert_eq!(lines.len(), 15);

        println!("\nRendered 'カ' (katakana ka) at 15x15:");
        for line in &lines {
            println!("{}", line);
        }
    }
}
