//! ATARI-style vector font: boxy, utilitarian.
//! Inspired by Asteroids, Lunar Lander score displays.

use glam::Vec2;
use vectorcade_shared::font::{FontStyleId, GlyphPath, GlyphPathCmd, VectorFont};

pub struct AtariMini;

impl VectorFont for AtariMini {
    fn style_id(&self) -> FontStyleId { FontStyleId::ATARI }
    fn has_glyph(&self, ch: char) -> bool { matches!(ch, '0'..='9' | 'A'..='Z' | ' ' | '.' | ',' | ':' | '-') }
    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath> { glyph(ch) }
    fn advance(&self, _ch: char) -> f32 { 1.0 }
}

fn glyph(ch: char) -> Vec<GlyphPath> {
    match ch {
        '0'..='9' => vec![build(DIGITS[(ch as u8 - b'0') as usize])],
        'A'..='Z' => vec![build(LETTERS[(ch as u8 - b'A') as usize])],
        ' ' => vec![],
        '.' => vec![GlyphPath { cmds: vec![
            GlyphPathCmd::MoveTo(Vec2::new(0.45, 0.1)), GlyphPathCmd::LineTo(Vec2::new(0.55, 0.1)),
            GlyphPathCmd::LineTo(Vec2::new(0.55, 0.2)), GlyphPathCmd::LineTo(Vec2::new(0.45, 0.2)), GlyphPathCmd::Close,
        ]}],
        ',' => vec![GlyphPath { cmds: vec![GlyphPathCmd::MoveTo(Vec2::new(0.5, 0.1)), GlyphPathCmd::LineTo(Vec2::new(0.4, 0.0))] }],
        ':' => colon_dots(0.45, 0.55, 0.25, 0.65),
        '-' => vec![GlyphPath { cmds: vec![GlyphPathCmd::MoveTo(Vec2::new(0.2, 0.5)), GlyphPathCmd::LineTo(Vec2::new(0.8, 0.5))] }],
        _ => vec![],
    }
}

fn colon_dots(x1: f32, x2: f32, y1: f32, y2: f32) -> Vec<GlyphPath> {
    let dot = |y: f32| GlyphPath { cmds: vec![
        GlyphPathCmd::MoveTo(Vec2::new(x1, y)), GlyphPathCmd::LineTo(Vec2::new(x2, y)),
        GlyphPathCmd::LineTo(Vec2::new(x2, y + 0.1)), GlyphPathCmd::LineTo(Vec2::new(x1, y + 0.1)), GlyphPathCmd::Close,
    ]};
    vec![dot(y1), dot(y2)]
}

fn build(data: &[(bool, f32, f32)]) -> GlyphPath {
    GlyphPath { cmds: data.iter().map(|&(m, x, y)| {
        if m { GlyphPathCmd::MoveTo(Vec2::new(x, y)) } else { GlyphPathCmd::LineTo(Vec2::new(x, y)) }
    }).collect() }
}

const M: bool = true;
const L: bool = false;

static DIGITS: [&[(bool, f32, f32)]; 10] = [
    &[(M,0.2,0.1),(L,0.8,0.1),(L,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1)],
    &[(M,0.5,0.1),(L,0.5,0.9)],
    &[(M,0.2,0.9),(L,0.8,0.9),(L,0.8,0.5),(L,0.2,0.5),(L,0.2,0.1),(L,0.8,0.1)],
    &[(M,0.2,0.9),(L,0.8,0.9),(L,0.8,0.1),(L,0.2,0.1),(M,0.2,0.5),(L,0.8,0.5)],
    &[(M,0.2,0.9),(L,0.2,0.5),(L,0.8,0.5),(M,0.8,0.9),(L,0.8,0.1)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.5),(L,0.8,0.5),(L,0.8,0.1),(L,0.2,0.1)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(L,0.8,0.1),(L,0.8,0.5),(L,0.2,0.5)],
    &[(M,0.2,0.9),(L,0.8,0.9),(L,0.8,0.1)],
    &[(M,0.2,0.1),(L,0.8,0.1),(L,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(M,0.2,0.5),(L,0.8,0.5)],
    &[(M,0.2,0.1),(L,0.8,0.1),(L,0.8,0.9),(L,0.2,0.9),(L,0.2,0.5),(L,0.8,0.5)],
];

static LETTERS: [&[(bool, f32, f32)]; 26] = [
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.8,0.9),(L,0.8,0.1),(M,0.2,0.5),(L,0.8,0.5)],
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.7,0.9),(L,0.8,0.8),(L,0.8,0.6),(L,0.7,0.5),
      (L,0.2,0.5),(L,0.7,0.5),(L,0.8,0.4),(L,0.8,0.2),(L,0.7,0.1),(L,0.2,0.1)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(L,0.8,0.1)],
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.6,0.9),(L,0.8,0.7),(L,0.8,0.3),(L,0.6,0.1),(L,0.2,0.1)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(L,0.8,0.1),(M,0.2,0.5),(L,0.6,0.5)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(M,0.2,0.5),(L,0.6,0.5)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(L,0.8,0.1),(L,0.8,0.5),(L,0.5,0.5)],
    &[(M,0.2,0.9),(L,0.2,0.1),(M,0.8,0.9),(L,0.8,0.1),(M,0.2,0.5),(L,0.8,0.5)],
    &[(M,0.3,0.9),(L,0.7,0.9),(M,0.5,0.9),(L,0.5,0.1),(M,0.3,0.1),(L,0.7,0.1)],
    &[(M,0.3,0.9),(L,0.7,0.9),(M,0.5,0.9),(L,0.5,0.2),(L,0.3,0.1),(L,0.2,0.2)],
    &[(M,0.2,0.9),(L,0.2,0.1),(M,0.8,0.9),(L,0.2,0.5),(L,0.8,0.1)],
    &[(M,0.2,0.9),(L,0.2,0.1),(L,0.8,0.1)],
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.5,0.6),(L,0.8,0.9),(L,0.8,0.1)],
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.8,0.1),(L,0.8,0.9)],
    &[(M,0.2,0.1),(L,0.8,0.1),(L,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1)],
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.8,0.9),(L,0.8,0.5),(L,0.2,0.5)],
    &[(M,0.2,0.1),(L,0.8,0.1),(L,0.8,0.9),(L,0.2,0.9),(L,0.2,0.1),(M,0.6,0.3),(L,0.9,0.0)],
    &[(M,0.2,0.1),(L,0.2,0.9),(L,0.8,0.9),(L,0.8,0.5),(L,0.2,0.5),(L,0.8,0.1)],
    &[(M,0.8,0.9),(L,0.2,0.9),(L,0.2,0.5),(L,0.8,0.5),(L,0.8,0.1),(L,0.2,0.1)],
    &[(M,0.2,0.9),(L,0.8,0.9),(M,0.5,0.9),(L,0.5,0.1)],
    &[(M,0.2,0.9),(L,0.2,0.1),(L,0.8,0.1),(L,0.8,0.9)],
    &[(M,0.2,0.9),(L,0.5,0.1),(L,0.8,0.9)],
    &[(M,0.1,0.9),(L,0.3,0.1),(L,0.5,0.5),(L,0.7,0.1),(L,0.9,0.9)],
    &[(M,0.2,0.9),(L,0.8,0.1),(M,0.8,0.9),(L,0.2,0.1)],
    &[(M,0.2,0.9),(L,0.5,0.5),(L,0.8,0.9),(M,0.5,0.5),(L,0.5,0.1)],
    &[(M,0.2,0.9),(L,0.8,0.9),(L,0.2,0.1),(L,0.8,0.1)],
];
