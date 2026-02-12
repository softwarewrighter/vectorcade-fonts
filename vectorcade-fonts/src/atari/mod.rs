//! ATARI-style vector font glyphs.
//!
//! Boxy, utilitarian aesthetic inspired by Asteroids/Lunar Lander.

mod digits;
mod letters;

use glam::Vec2;
use vectorcade_shared::font::{GlyphPath, GlyphPathCmd};

pub fn glyph(ch: char) -> Option<Vec<GlyphPath>> {
    match ch {
        '0'..='9' => digits::glyph(ch),
        'A'..='Z' => letters::glyph(ch),
        ' ' => Some(vec![]),
        '.' => Some(path(&[(0.45, 0.1), (0.55, 0.1), (0.55, 0.2), (0.45, 0.2)], true)),
        ',' => Some(path(&[(0.5, 0.1), (0.4, 0.0)], false)),
        ':' => Some(two_dots()),
        '-' => Some(path(&[(0.2, 0.5), (0.8, 0.5)], false)),
        _ => None,
    }
}

fn path(pts: &[(f32, f32)], close: bool) -> Vec<GlyphPath> {
    let mut cmds: Vec<GlyphPathCmd> = pts
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| {
            if i == 0 { GlyphPathCmd::MoveTo(Vec2::new(x, y)) }
            else { GlyphPathCmd::LineTo(Vec2::new(x, y)) }
        })
        .collect();
    if close { cmds.push(GlyphPathCmd::Close); }
    vec![GlyphPath { cmds }]
}

fn two_dots() -> Vec<GlyphPath> {
    let dot = |y: f32| GlyphPath {
        cmds: vec![
            GlyphPathCmd::MoveTo(Vec2::new(0.45, y)),
            GlyphPathCmd::LineTo(Vec2::new(0.55, y)),
            GlyphPathCmd::LineTo(Vec2::new(0.55, y + 0.1)),
            GlyphPathCmd::LineTo(Vec2::new(0.45, y + 0.1)),
            GlyphPathCmd::Close,
        ],
    };
    vec![dot(0.25), dot(0.65)]
}
