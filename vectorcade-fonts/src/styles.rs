use glam::Vec2;
use vectorcade_shared::font::{FontStyleId, GlyphPath, GlyphPathCmd, VectorFont};

/// Tiny demo font: supports digits 0-9 and a few letters for "SCORE".
/// Agent should expand coverage and add multiple styles.
pub struct AtariMini;

impl VectorFont for AtariMini {
    fn style_id(&self) -> FontStyleId { FontStyleId::ATARI }

    fn has_glyph(&self, ch: char) -> bool {
        ch.is_ascii_digit() || matches!(ch, 'S'|'C'|'O'|'R'|'E'|' ')
    }

    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath> {
        // Font-local units: (0..1) square-ish.
        let p = |x: f32, y: f32| Vec2::new(x, y);
        let mk = |cmds: Vec<GlyphPathCmd>| GlyphPath { cmds };

        match ch {
            '0' => vec![mk(vec![
                GlyphPathCmd::MoveTo(p(0.2,0.1)),
                GlyphPathCmd::LineTo(p(0.8,0.1)),
                GlyphPathCmd::LineTo(p(0.8,0.9)),
                GlyphPathCmd::LineTo(p(0.2,0.9)),
                GlyphPathCmd::Close,
            ])],
            '1' => vec![mk(vec![
                GlyphPathCmd::MoveTo(p(0.5,0.1)),
                GlyphPathCmd::LineTo(p(0.5,0.9)),
            ])],
            'S' => vec![mk(vec![
                GlyphPathCmd::MoveTo(p(0.8,0.85)),
                GlyphPathCmd::LineTo(p(0.2,0.85)),
                GlyphPathCmd::LineTo(p(0.2,0.55)),
                GlyphPathCmd::LineTo(p(0.8,0.55)),
                GlyphPathCmd::LineTo(p(0.8,0.15)),
                GlyphPathCmd::LineTo(p(0.2,0.15)),
            ])],
            ' ' => vec![],
            _ => vec![mk(vec![
                GlyphPathCmd::MoveTo(p(0.2,0.2)),
                GlyphPathCmd::LineTo(p(0.8,0.8)),
            ])],
        }
    }

    fn advance(&self, _ch: char) -> f32 { 1.0 }
}
