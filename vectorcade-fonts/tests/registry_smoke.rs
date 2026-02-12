use vectorcade_fonts::FontRegistry;
use vectorcade_fonts::styles::AtariMini;
use vectorcade_shared::font::FontStyleId;

#[test]
fn registry_finds_font() {
    let mut reg = FontRegistry::new();
    reg.register(AtariMini);
    assert!(reg.get(FontStyleId::ATARI).is_some());
    assert!(reg.get(FontStyleId::MIDWAY).is_none());
}
