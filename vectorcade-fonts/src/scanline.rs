//! VECTOR_SCANLINE-style font: broken segments simulating worn CRT phosphor.
//! Based on ATARI shapes with gaps introduced along strokes.

use glam::Vec2;
use vectorcade_shared::font::{FontStyleId, GlyphPath, GlyphPathCmd, VectorFont};

pub struct VectorScanline;

impl VectorFont for VectorScanline {
    fn style_id(&self) -> FontStyleId { FontStyleId::VECTOR_SCANLINE }
    fn has_glyph(&self, ch: char) -> bool { matches!(ch, '0'..='9' | 'A'..='Z' | ' ' | '.' | ',' | ':' | '-') }
    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath> { glyph(ch) }
    fn advance(&self, _ch: char) -> f32 { 1.0 }
}

fn glyph(ch: char) -> Vec<GlyphPath> {
    match ch {
        '0'..='9' => vec![build_broken(DIGITS[(ch as u8 - b'0') as usize])],
        'A'..='Z' => vec![build_broken(LETTERS[(ch as u8 - b'A') as usize])],
        ' ' => vec![],
        '.' => vec![build_broken(&[(M,0.45,0.1),(L,0.55,0.1),(L,0.55,0.2),(L,0.45,0.2),(L,0.45,0.1)])],
        ',' => vec![build_broken(&[(M,0.5,0.1),(L,0.4,0.0)])],
        ':' => vec![build_broken(&[(M,0.45,0.25),(L,0.55,0.25),(L,0.55,0.35),(L,0.45,0.35),(L,0.45,0.25),
                                   (M,0.45,0.65),(L,0.55,0.65),(L,0.55,0.75),(L,0.45,0.75),(L,0.45,0.65)])],
        '-' => vec![build_broken(&[(M,0.2,0.5),(L,0.8,0.5)])],
        _ => vec![],
    }
}

fn build_broken(data: &[(bool, f32, f32)]) -> GlyphPath {
    let mut cmds = Vec::new();
    let mut last_pos: Option<Vec2> = None;
    for &(is_move, x, y) in data {
        let p = Vec2::new(x, y);
        if is_move { last_pos = Some(p); cmds.push(GlyphPathCmd::MoveTo(p)); }
        else if let Some(from) = last_pos { add_broken_line(&mut cmds, from, p); last_pos = Some(p); }
    }
    GlyphPath { cmds }
}

fn add_broken_line(cmds: &mut Vec<GlyphPathCmd>, from: Vec2, to: Vec2) {
    let d = to - from;
    let len = d.length();
    if len < 0.15 { cmds.push(GlyphPathCmd::LineTo(to)); return; }
    let seg = 0.12;
    let gap = 0.04;
    let dir = d / len;
    let mut t = 0.0;
    while t < len {
        let end_t = (t + seg).min(len);
        cmds.push(GlyphPathCmd::LineTo(from + dir * end_t));
        t = end_t + gap;
        if t < len { cmds.push(GlyphPathCmd::MoveTo(from + dir * t)); }
    }
}

const M: bool = true;
const L: bool = false;

// Same shapes as ATARI for consistency
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
