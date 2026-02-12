//! Font style implementations.

use vectorcade_shared::font::{FontStyleId, GlyphPath, VectorFont};

use crate::atari;

/// ATARI-style vector font: boxy, utilitarian.
/// Inspired by Asteroids, Lunar Lander score displays.
pub struct AtariMini;

impl VectorFont for AtariMini {
    fn style_id(&self) -> FontStyleId { FontStyleId::ATARI }

    fn has_glyph(&self, ch: char) -> bool {
        matches!(ch, '0'..='9' | 'A'..='Z' | ' ' | '.' | ',' | ':' | '-')
    }

    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath> {
        atari::glyph(ch).unwrap_or_default()
    }

    fn advance(&self, _ch: char) -> f32 { 1.0 }
}
