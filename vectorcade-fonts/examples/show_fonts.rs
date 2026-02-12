//! Example: Display all font styles and their glyph counts.

use vectorcade_fonts::{AtariMini, Cinematronics, FontRegistry, Midway, VectorScanline};
use vectorcade_shared::font::VectorFont;

fn main() {
    println!("VectorCade Fonts Demo\n");

    let fonts: Vec<Box<dyn VectorFont>> = vec![
        Box::new(AtariMini),
        Box::new(Cinematronics),
        Box::new(Midway),
        Box::new(VectorScanline),
    ];

    for font in &fonts {
        print_font_info(font.as_ref());
    }

    println!("\nRegistry test:");
    let mut reg = FontRegistry::new();
    reg.register(AtariMini);
    reg.register(Cinematronics);
    reg.register(Midway);
    reg.register(VectorScanline);
    println!("  Registered {} fonts", 4);

    println!("\nSample glyph data for 'A' (AtariMini):");
    let paths = AtariMini.glyph_paths('A');
    for (i, path) in paths.iter().enumerate() {
        println!("  Path {}: {} commands", i, path.cmds.len());
    }
}

fn print_font_info(font: &dyn VectorFont) {
    let style = font.style_id();
    let digits = ('0'..='9').filter(|&c| font.has_glyph(c)).count();
    let letters = ('A'..='Z').filter(|&c| font.has_glyph(c)).count();
    let punct = [' ', '.', ',', ':', '-'].iter().filter(|&&c| font.has_glyph(c)).count();

    println!("Style {:?}:", style);
    println!("  Digits:  {}/10", digits);
    println!("  Letters: {}/26", letters);
    println!("  Punct:   {}/5", punct);
    println!("  Advance: {}", font.advance('A'));
    println!();
}
