# Design Document

## External Dependencies

### From vectorcade-shared

This crate implements traits and uses types defined in `vectorcade-shared::font`:

```rust
pub trait VectorFont {
    fn style_id(&self) -> FontStyleId;
    fn has_glyph(&self, ch: char) -> bool;
    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath>;
    fn advance(&self, ch: char) -> f32;
}

pub struct GlyphPath {
    pub cmds: Vec<GlyphPathCmd>,
}

pub enum GlyphPathCmd {
    MoveTo(Vec2),
    LineTo(Vec2),
    Close,
}

pub struct FontStyleId(pub u32);
impl FontStyleId {
    pub const ATARI: Self = Self(1);
    pub const CINEMATRONICS: Self = Self(2);
    pub const MIDWAY: Self = Self(3);
    pub const VECTOR_SCANLINE: Self = Self(4);
}
```

## Implementation Design

### Glyph Coordinate System

All glyphs are defined in a normalized coordinate space:
- Origin: bottom-left of glyph cell
- X: 0.0 (left edge) to 1.0 (right edge)
- Y: 0.0 (baseline/bottom) to 1.0 (top)

The renderer scales this to the requested text size.

### Font Style Implementation Pattern

Each font style is a zero-sized struct implementing `VectorFont`:

```rust
pub struct AtariMini;

impl VectorFont for AtariMini {
    fn style_id(&self) -> FontStyleId { FontStyleId::ATARI }

    fn has_glyph(&self, ch: char) -> bool {
        // Return true for supported characters
    }

    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath> {
        // Return stroke commands for the character
    }

    fn advance(&self, _ch: char) -> f32 { 1.0 }
}
```

### Style Differentiation Strategies

| Style | Stroke Characteristics |
|-------|----------------------|
| ATARI | Right angles only, uniform segments |
| CINEMATRONICS | 45° diagonals, thin proportions |
| MIDWAY | Slight curves (approximated with short segments), wider proportions |
| VECTOR_SCANLINE | Same as base style but with gap breaks mid-segment |

### FontRegistry Design

Simple linear search is sufficient given:
- Small number of fonts (4-8)
- Lookup happens once at game init, not per-frame

```rust
pub struct FontRegistry {
    fonts: Vec<Box<dyn VectorFont + Send + Sync>>,
}
```

Thread-safe (`Send + Sync`) to support potential future async loading.

## Future Considerations

### Glyph Caching

If profiling shows glyph_paths allocation is hot:
- Pre-compute all glyphs at registration time
- Store as `HashMap<char, Vec<GlyphPath>>` per font
- Return references instead of owned vectors

### Kerning

Not planned for MVP. If needed:
- Add `kern(&self, a: char, b: char) -> f32` to trait
- Default implementation returns 0.0
