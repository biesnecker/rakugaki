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
/// Terminal characters are typically ~2:1 (height:width) in aspect ratio,
/// which is accounted for in the rasterization.
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

    // Account for terminal character aspect ratio (~2:1 height:width)
    // We need to scale the pixel height to compensate
    let px_per_char_width = 1.0;
    let px_per_char_height = 2.0; // Terminal chars are roughly twice as tall as wide

    let pixel_width = width as f32 * px_per_char_width;
    let pixel_height = height as f32 * px_per_char_height;

    // Use the larger dimension to determine font size for best quality
    let font_size = pixel_width.max(pixel_height);

    // Rasterize the glyph
    let (metrics, bitmap) = font.rasterize(codepoint, font_size);

    // Convert bitmap to ASCII using simple thresholding
    bitmap_to_ascii(&bitmap, metrics.width, metrics.height, width, height)
}

/// Converts a grayscale bitmap to ASCII art using simple thresholding
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

    let mut result = Vec::with_capacity(target_height);

    // Sample the bitmap to fit target dimensions
    for row in 0..target_height {
        let mut line = String::with_capacity(target_width);

        for col in 0..target_width {
            // Map target position to bitmap position
            let bmp_y = (row * bitmap_height) / target_height.max(1);
            let bmp_x = (col * bitmap_width) / target_width.max(1);

            let idx = bmp_y * bitmap_width + bmp_x;
            let pixel = if idx < bitmap.len() { bitmap[idx] } else { 0 };

            // Simple threshold: 128 is the midpoint of 0-255
            let ch = if pixel > 128 { '#' } else { ' ' };
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
        // 2x2 bitmap with a simple pattern
        let bitmap = vec![255, 0, 0, 255];
        let result = bitmap_to_ascii(&bitmap, 2, 2, 2, 2).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "# ");
        assert_eq!(result[1], " #");
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
