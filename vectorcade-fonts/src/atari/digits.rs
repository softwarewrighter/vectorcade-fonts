//! Digit glyphs 0-9 for ATARI style.
//! Data-driven: static arrays converted to GlyphPathCmds.

use glam::Vec2;
use vectorcade_shared::font::{GlyphPath, GlyphPathCmd};

/// Glyph data: (is_move, x, y). is_move=true for MoveTo, false for LineTo.
/// Sequence ends with Close if last point connects back.
type GlyphData = &'static [(bool, f32, f32)];

static GLYPHS: [GlyphData; 10] = [
    // 0: box
    &[(true, 0.2, 0.1), (false, 0.8, 0.1), (false, 0.8, 0.9), (false, 0.2, 0.9)],
    // 1: vertical line
    &[(true, 0.5, 0.1), (false, 0.5, 0.9)],
    // 2: S-shape top-to-bottom
    &[(true, 0.2, 0.9), (false, 0.8, 0.9), (false, 0.8, 0.5), (false, 0.2, 0.5),
      (false, 0.2, 0.1), (false, 0.8, 0.1)],
    // 3: E-shape right side
    &[(true, 0.2, 0.9), (false, 0.8, 0.9), (false, 0.8, 0.1), (false, 0.2, 0.1),
      (true, 0.2, 0.5), (false, 0.8, 0.5)],
    // 4: |-| with vertical
    &[(true, 0.2, 0.9), (false, 0.2, 0.5), (false, 0.8, 0.5),
      (true, 0.8, 0.9), (false, 0.8, 0.1)],
    // 5: S-shape reversed
    &[(true, 0.8, 0.9), (false, 0.2, 0.9), (false, 0.2, 0.5), (false, 0.8, 0.5),
      (false, 0.8, 0.1), (false, 0.2, 0.1)],
    // 6: like 5 but closed bottom
    &[(true, 0.8, 0.9), (false, 0.2, 0.9), (false, 0.2, 0.1), (false, 0.8, 0.1),
      (false, 0.8, 0.5), (false, 0.2, 0.5)],
    // 7: angle down
    &[(true, 0.2, 0.9), (false, 0.8, 0.9), (false, 0.8, 0.1)],
    // 8: box with middle bar
    &[(true, 0.2, 0.1), (false, 0.8, 0.1), (false, 0.8, 0.9), (false, 0.2, 0.9),
      (false, 0.2, 0.1), (true, 0.2, 0.5), (false, 0.8, 0.5)],
    // 9: like 8 but open bottom-left
    &[(true, 0.2, 0.1), (false, 0.8, 0.1), (false, 0.8, 0.9), (false, 0.2, 0.9),
      (false, 0.2, 0.5), (false, 0.8, 0.5)],
];

static CLOSES: [bool; 10] = [true, false, false, false, false, false, false, false, false, false];

pub fn glyph(ch: char) -> Option<Vec<GlyphPath>> {
    let idx = ch.to_digit(10)? as usize;
    Some(vec![build_path(GLYPHS[idx], CLOSES[idx])])
}

fn build_path(data: GlyphData, close: bool) -> GlyphPath {
    let mut cmds: Vec<GlyphPathCmd> = data.iter().map(|&(is_mv, x, y)| {
        let p = Vec2::new(x, y);
        if is_mv { GlyphPathCmd::MoveTo(p) } else { GlyphPathCmd::LineTo(p) }
    }).collect();
    if close { cmds.push(GlyphPathCmd::Close); }
    GlyphPath { cmds }
}
