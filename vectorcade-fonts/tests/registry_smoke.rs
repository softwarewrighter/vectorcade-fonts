use vectorcade_fonts::FontRegistry;
use vectorcade_fonts::styles::AtariMini;
use vectorcade_shared::font::{FontStyleId, VectorFont};

#[test]
fn registry_finds_font() {
    let mut reg = FontRegistry::new();
    reg.register(AtariMini);
    assert!(reg.get(FontStyleId::ATARI).is_some());
    assert!(reg.get(FontStyleId::MIDWAY).is_none());
}

#[test]
fn atari_covers_digits() {
    let font = AtariMini;
    for ch in '0'..='9' {
        assert!(font.has_glyph(ch), "missing digit {ch}");
        assert!(!font.glyph_paths(ch).is_empty(), "empty paths for {ch}");
    }
}

#[test]
fn atari_covers_letters() {
    let font = AtariMini;
    for ch in 'A'..='Z' {
        assert!(font.has_glyph(ch), "missing letter {ch}");
        assert!(!font.glyph_paths(ch).is_empty(), "empty paths for {ch}");
    }
}

#[test]
fn atari_covers_punctuation() {
    let font = AtariMini;
    for ch in [' ', '.', ',', ':', '-'] {
        assert!(font.has_glyph(ch), "missing punct '{ch}'");
    }
}
