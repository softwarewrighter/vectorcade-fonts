use vectorcade_fonts::{AtariMini, Cinematronics, FontRegistry};
use vectorcade_shared::font::{FontStyleId, VectorFont};

#[test]
fn registry_finds_fonts() {
    let mut reg = FontRegistry::new();
    reg.register(AtariMini);
    reg.register(Cinematronics);
    assert!(reg.get(FontStyleId::ATARI).is_some());
    assert!(reg.get(FontStyleId::CINEMATRONICS).is_some());
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
fn cinematronics_covers_all() {
    let font = Cinematronics;
    for ch in '0'..='9' {
        assert!(font.has_glyph(ch), "missing digit {ch}");
        assert!(!font.glyph_paths(ch).is_empty(), "empty paths for {ch}");
    }
    for ch in 'A'..='Z' {
        assert!(font.has_glyph(ch), "missing letter {ch}");
        assert!(!font.glyph_paths(ch).is_empty(), "empty paths for {ch}");
    }
    for ch in [' ', '.', ',', ':', '-'] {
        assert!(font.has_glyph(ch), "missing punct '{ch}'");
    }
}
