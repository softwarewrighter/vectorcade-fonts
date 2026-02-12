//! MIDWAY-style vector font: slightly rounded, wider proportions.
//! Inspired by Omega Race.

use glam::Vec2;
use vectorcade_shared::font::{FontStyleId, GlyphPath, GlyphPathCmd, VectorFont};

pub struct Midway;

impl VectorFont for Midway {
    fn style_id(&self) -> FontStyleId { FontStyleId::MIDWAY }
    fn has_glyph(&self, ch: char) -> bool { matches!(ch, '0'..='9' | 'A'..='Z' | ' ' | '.' | ',' | ':' | '-') }
    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath> { glyph(ch) }
    fn advance(&self, _ch: char) -> f32 { 1.1 }
}

fn glyph(ch: char) -> Vec<GlyphPath> {
    match ch {
        '0'..='9' => vec![build(DIGITS[(ch as u8 - b'0') as usize])],
        'A'..='Z' => vec![build(LETTERS[(ch as u8 - b'A') as usize])],
        ' ' => vec![],
        '.' => vec![GlyphPath { cmds: vec![
            GlyphPathCmd::MoveTo(Vec2::new(0.4, 0.0)), GlyphPathCmd::LineTo(Vec2::new(0.5, 0.1)),
            GlyphPathCmd::LineTo(Vec2::new(0.6, 0.0)), GlyphPathCmd::LineTo(Vec2::new(0.5, -0.05)), GlyphPathCmd::Close,
        ]}],
        ',' => vec![GlyphPath { cmds: vec![GlyphPathCmd::MoveTo(Vec2::new(0.5, 0.05)), GlyphPathCmd::LineTo(Vec2::new(0.35, -0.1))] }],
        ':' => colon_rounded(),
        '-' => vec![GlyphPath { cmds: vec![GlyphPathCmd::MoveTo(Vec2::new(0.15, 0.5)), GlyphPathCmd::LineTo(Vec2::new(0.85, 0.5))] }],
        _ => vec![],
    }
}

fn colon_rounded() -> Vec<GlyphPath> {
    let dot = |y: f32| GlyphPath { cmds: vec![
        GlyphPathCmd::MoveTo(Vec2::new(0.4, y)), GlyphPathCmd::LineTo(Vec2::new(0.5, y + 0.08)),
        GlyphPathCmd::LineTo(Vec2::new(0.6, y)), GlyphPathCmd::LineTo(Vec2::new(0.5, y - 0.08)), GlyphPathCmd::Close,
    ]};
    vec![dot(0.3), dot(0.7)]
}

fn build(data: &[(bool, f32, f32)]) -> GlyphPath {
    GlyphPath { cmds: data.iter().map(|&(m, x, y)| {
        if m { GlyphPathCmd::MoveTo(Vec2::new(x, y)) } else { GlyphPathCmd::LineTo(Vec2::new(x, y)) }
    }).collect() }
}

const M: bool = true;
const L: bool = false;

// Wider proportions (0.1-0.9), rounded corners approximated with extra vertices
static DIGITS: [&[(bool, f32, f32)]; 10] = [
    // 0: rounded rectangle
    &[(M,0.2,0.1),(L,0.15,0.2),(L,0.15,0.8),(L,0.2,0.9),(L,0.8,0.9),(L,0.85,0.8),
      (L,0.85,0.2),(L,0.8,0.1),(L,0.2,0.1)],
    // 1: with base
    &[(M,0.35,0.75),(L,0.5,0.9),(L,0.5,0.1),(M,0.3,0.1),(L,0.7,0.1)],
    // 2: rounded top
    &[(M,0.15,0.75),(L,0.2,0.85),(L,0.3,0.9),(L,0.7,0.9),(L,0.8,0.85),(L,0.85,0.75),
      (L,0.85,0.55),(L,0.8,0.5),(L,0.2,0.5),(L,0.15,0.4),(L,0.15,0.15),(L,0.2,0.1),(L,0.85,0.1)],
    // 3: double curve
    &[(M,0.15,0.85),(L,0.2,0.9),(L,0.8,0.9),(L,0.85,0.85),(L,0.85,0.55),(L,0.8,0.5),(L,0.5,0.5),
      (M,0.8,0.5),(L,0.85,0.45),(L,0.85,0.15),(L,0.8,0.1),(L,0.2,0.1),(L,0.15,0.15)],
    // 4: angular with curve
    &[(M,0.15,0.9),(L,0.15,0.5),(L,0.2,0.45),(L,0.85,0.45),(M,0.7,0.9),(L,0.7,0.1)],
    // 5: rounded bottom
    &[(M,0.85,0.9),(L,0.15,0.9),(L,0.15,0.55),(L,0.2,0.5),(L,0.75,0.5),(L,0.85,0.45),
      (L,0.85,0.15),(L,0.8,0.1),(L,0.2,0.1),(L,0.15,0.15)],
    // 6: full rounded
    &[(M,0.8,0.85),(L,0.75,0.9),(L,0.25,0.9),(L,0.15,0.8),(L,0.15,0.2),(L,0.2,0.1),
      (L,0.8,0.1),(L,0.85,0.2),(L,0.85,0.45),(L,0.8,0.5),(L,0.15,0.5)],
    // 7: with curve
    &[(M,0.15,0.9),(L,0.8,0.9),(L,0.85,0.85),(L,0.5,0.1)],
    // 8: double rounded
    &[(M,0.25,0.5),(L,0.15,0.4),(L,0.15,0.15),(L,0.2,0.1),(L,0.8,0.1),(L,0.85,0.15),
      (L,0.85,0.4),(L,0.75,0.5),(L,0.25,0.5),(L,0.15,0.6),(L,0.15,0.85),(L,0.2,0.9),
      (L,0.8,0.9),(L,0.85,0.85),(L,0.85,0.6),(L,0.75,0.5)],
    // 9: rounded top loop
    &[(M,0.2,0.15),(L,0.25,0.1),(L,0.75,0.1),(L,0.85,0.2),(L,0.85,0.8),(L,0.8,0.9),
      (L,0.2,0.9),(L,0.15,0.8),(L,0.15,0.55),(L,0.2,0.5),(L,0.85,0.5)],
];

static LETTERS: [&[(bool, f32, f32)]; 26] = [
    // A: pointed with wide base
    &[(M,0.1,0.1),(L,0.5,0.9),(L,0.9,0.1),(M,0.25,0.4),(L,0.75,0.4)],
    // B: double bumps rounded
    &[(M,0.15,0.1),(L,0.15,0.9),(L,0.7,0.9),(L,0.85,0.8),(L,0.85,0.6),(L,0.75,0.5),
      (L,0.15,0.5),(L,0.75,0.5),(L,0.85,0.4),(L,0.85,0.2),(L,0.7,0.1),(L,0.15,0.1)],
    // C: wide curve
    &[(M,0.85,0.8),(L,0.8,0.9),(L,0.2,0.9),(L,0.15,0.8),(L,0.15,0.2),(L,0.2,0.1),
      (L,0.8,0.1),(L,0.85,0.2)],
    // D: rounded right
    &[(M,0.15,0.1),(L,0.15,0.9),(L,0.6,0.9),(L,0.8,0.8),(L,0.85,0.7),(L,0.85,0.3),
      (L,0.8,0.2),(L,0.6,0.1),(L,0.15,0.1)],
    // E: wide
    &[(M,0.85,0.9),(L,0.15,0.9),(L,0.15,0.1),(L,0.85,0.1),(M,0.15,0.5),(L,0.7,0.5)],
    // F
    &[(M,0.85,0.9),(L,0.15,0.9),(L,0.15,0.1),(M,0.15,0.5),(L,0.7,0.5)],
    // G: wide with bar
    &[(M,0.85,0.8),(L,0.8,0.9),(L,0.2,0.9),(L,0.15,0.8),(L,0.15,0.2),(L,0.2,0.1),
      (L,0.8,0.1),(L,0.85,0.2),(L,0.85,0.5),(L,0.5,0.5)],
    // H: wide
    &[(M,0.15,0.9),(L,0.15,0.1),(M,0.85,0.9),(L,0.85,0.1),(M,0.15,0.5),(L,0.85,0.5)],
    // I: with serifs
    &[(M,0.25,0.9),(L,0.75,0.9),(M,0.5,0.9),(L,0.5,0.1),(M,0.25,0.1),(L,0.75,0.1)],
    // J: curved bottom
    &[(M,0.3,0.9),(L,0.75,0.9),(M,0.6,0.9),(L,0.6,0.2),(L,0.55,0.1),(L,0.3,0.1),(L,0.2,0.2)],
    // K: wide diagonal
    &[(M,0.15,0.9),(L,0.15,0.1),(M,0.85,0.9),(L,0.15,0.5),(L,0.85,0.1)],
    // L: wide
    &[(M,0.15,0.9),(L,0.15,0.1),(L,0.85,0.1)],
    // M: wide peaks
    &[(M,0.1,0.1),(L,0.1,0.9),(L,0.5,0.5),(L,0.9,0.9),(L,0.9,0.1)],
    // N: wide diagonal
    &[(M,0.15,0.1),(L,0.15,0.9),(L,0.85,0.1),(L,0.85,0.9)],
    // O: rounded
    &[(M,0.2,0.1),(L,0.15,0.2),(L,0.15,0.8),(L,0.2,0.9),(L,0.8,0.9),(L,0.85,0.8),
      (L,0.85,0.2),(L,0.8,0.1),(L,0.2,0.1)],
    // P: rounded top
    &[(M,0.15,0.1),(L,0.15,0.9),(L,0.75,0.9),(L,0.85,0.8),(L,0.85,0.55),(L,0.75,0.45),(L,0.15,0.45)],
    // Q: rounded with tail
    &[(M,0.2,0.1),(L,0.15,0.2),(L,0.15,0.8),(L,0.2,0.9),(L,0.8,0.9),(L,0.85,0.8),
      (L,0.85,0.2),(L,0.8,0.1),(L,0.2,0.1),(M,0.6,0.25),(L,0.95,0.0)],
    // R: rounded with kick
    &[(M,0.15,0.1),(L,0.15,0.9),(L,0.75,0.9),(L,0.85,0.8),(L,0.85,0.55),(L,0.75,0.45),
      (L,0.15,0.45),(M,0.55,0.45),(L,0.85,0.1)],
    // S: double curve
    &[(M,0.85,0.8),(L,0.8,0.9),(L,0.2,0.9),(L,0.15,0.8),(L,0.15,0.55),(L,0.2,0.5),
      (L,0.8,0.5),(L,0.85,0.45),(L,0.85,0.2),(L,0.8,0.1),(L,0.2,0.1),(L,0.15,0.2)],
    // T: wide
    &[(M,0.1,0.9),(L,0.9,0.9),(M,0.5,0.9),(L,0.5,0.1)],
    // U: rounded bottom
    &[(M,0.15,0.9),(L,0.15,0.2),(L,0.2,0.1),(L,0.8,0.1),(L,0.85,0.2),(L,0.85,0.9)],
    // V: wide
    &[(M,0.1,0.9),(L,0.5,0.1),(L,0.9,0.9)],
    // W: wide double valley
    &[(M,0.05,0.9),(L,0.25,0.1),(L,0.5,0.5),(L,0.75,0.1),(L,0.95,0.9)],
    // X: wide cross
    &[(M,0.1,0.9),(L,0.9,0.1),(M,0.9,0.9),(L,0.1,0.1)],
    // Y: wide fork
    &[(M,0.1,0.9),(L,0.5,0.5),(L,0.9,0.9),(M,0.5,0.5),(L,0.5,0.1)],
    // Z: wide
    &[(M,0.1,0.9),(L,0.9,0.9),(L,0.1,0.1),(L,0.9,0.1)],
];
