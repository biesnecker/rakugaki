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
/// Uses a default aspect ratio of 2.0 (terminal chars are ~2x taller than wide).
/// For custom aspect ratios, use `render_char_with_aspect_ratio`.
pub fn render_char(
    font_path: impl AsRef<Path>,
    codepoint: char,
    width: usize,
    height: usize,
) -> Result<Vec<String>, String> {
    render_char_with_aspect_ratio(font_path, codepoint, width, height, 2.0)
}

/// Renders a single Unicode character as ASCII art with configurable aspect ratio.
///
/// # Arguments
/// * `font_path` - Path to the TTF/OTF font file
/// * `codepoint` - The Unicode character to render
/// * `width` - Target width in terminal characters
/// * `height` - Target height in terminal characters
/// * `aspect_ratio` - Character cell height-to-width ratio (e.g., 2.0 means cells are 2x taller than wide)
///
/// # Returns
/// A vector of strings, one per line, representing the ASCII art
///
/// # Common Aspect Ratios
/// * 2.0 - Traditional terminal fonts (e.g., 8x16)
/// * 1.67 - Modern monospace fonts (3:5 width:height ratio)
/// * 1.25 - Some terminals like VT220 (8x10 cells)
pub fn render_char_with_aspect_ratio(
    font_path: impl AsRef<Path>,
    codepoint: char,
    width: usize,
    height: usize,
    aspect_ratio: f32,
) -> Result<Vec<String>, String> {
    // Load font file
    let font_data = std::fs::read(font_path).map_err(|e| format!("Failed to read font: {}", e))?;
    let font = Font::from_bytes(font_data.as_slice(), fontdue::FontSettings::default())
        .map_err(|e| format!("Failed to parse font: {}", e))?;

    // To render correctly, we need to think in "physical" pixels.
    // If terminal cells are aspect_ratio:1 (height:width), then to fill
    // our target area with square pixels, we need:
    // - width characters × 1 unit wide each = width physical pixels wide
    // - height characters × aspect_ratio units tall each = height × aspect_ratio physical pixels tall
    //
    // We want to render the glyph at a size that will look right when sampled
    // into our character grid accounting for non-square cells.

    let physical_width = width as f32;
    let physical_height = height as f32 * aspect_ratio;

    // Use the larger physical dimension to render at good quality
    // (fontdue's size parameter roughly corresponds to pixel height)
    let font_size = physical_width.max(physical_height);

    // Rasterize the glyph
    let (metrics, bitmap) = font.rasterize(codepoint, font_size);

    // Convert bitmap to ASCII using simple thresholding
    bitmap_to_ascii(
        &bitmap,
        metrics.width,
        metrics.height,
        width,
        height,
        aspect_ratio,
    )
}

/// Converts a grayscale bitmap to ASCII art using simple thresholding
///
/// # Arguments
/// * `bitmap` - Grayscale bitmap data (0-255 values)
/// * `bitmap_width` - Width of the bitmap in pixels
/// * `bitmap_height` - Height of the bitmap in pixels
/// * `target_width` - Target width in terminal characters
/// * `target_height` - Target height in terminal characters
/// * `aspect_ratio` - Terminal cell height-to-width ratio
fn bitmap_to_ascii(
    bitmap: &[u8],
    bitmap_width: usize,
    bitmap_height: usize,
    target_width: usize,
    target_height: usize,
    aspect_ratio: f32,
) -> Result<Vec<String>, String> {
    if bitmap.is_empty() {
        return Ok(vec![" ".repeat(target_width); target_height]);
    }

    let mut result = Vec::with_capacity(target_height);

    // Each terminal cell represents a rectangle in square-pixel space:
    // - Width: 1 unit
    // - Height: aspect_ratio units
    // We need to map our terminal grid back to the bitmap accounting for this.

    // The bitmap represents the glyph in square pixels. To sample it correctly,
    // we need to think of our terminal grid as covering a rectangular area where
    // each cell is 1 unit wide but aspect_ratio units tall.

    // Total physical height covered by our terminal grid
    let physical_grid_height = target_height as f32 * aspect_ratio;
    let physical_grid_width = target_width as f32;

    // Scale factors from physical space to bitmap space
    let scale_x = bitmap_width as f32 / physical_grid_width;
    let scale_y = bitmap_height as f32 / physical_grid_height;

    for row in 0..target_height {
        let mut line = String::with_capacity(target_width);

        for col in 0..target_width {
            // Map terminal cell position to bitmap position in square pixel space
            let phys_x = col as f32;
            let phys_y = row as f32 * aspect_ratio;

            let bmp_x = (phys_x * scale_x) as usize;
            let bmp_y = (phys_y * scale_y) as usize;

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
        let result = bitmap_to_ascii(&[], 0, 0, 5, 3, 2.0).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "     ");
    }

    #[test]
    fn test_bitmap_to_ascii_simple() {
        // 2x2 bitmap with a simple pattern (assuming square cells for simplicity)
        let bitmap = vec![255, 0, 0, 255];
        let result = bitmap_to_ascii(&bitmap, 2, 2, 2, 2, 1.0).unwrap();
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
