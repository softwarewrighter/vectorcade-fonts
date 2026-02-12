# Architecture

## Repository Position in DAG

```
vectorcade-shared (root)
        │
        ▼
  vectorcade-fonts  ◀── YOU ARE HERE
        │
        ▼
  vectorcade-games
        │
        ▼
  vectorcade-web-yew
```

This repo depends on `vectorcade-shared` for:
- `VectorFont` trait definition
- `FontStyleId` enum
- `GlyphPath` and `GlyphPathCmd` types
- `glam::Vec2` (re-exported)

## Module Structure

```
vectorcade-fonts/
├── src/
│   ├── lib.rs          # Public API exports
│   ├── registry.rs     # FontRegistry for style lookup
│   └── styles.rs       # Font implementations (AtariMini, etc.)
└── tests/
    └── registry_smoke.rs
```

## Key Components

### FontRegistry

Central lookup for font styles. Games request fonts by `FontStyleId`, registry returns the appropriate `VectorFont` implementation.

```rust
let mut reg = FontRegistry::new();
reg.register(AtariMini);
let font = reg.get(FontStyleId::ATARI);
```

### Font Implementations

Each font style implements `VectorFont` from shared:
- `style_id()` - Returns which `FontStyleId` this font provides
- `has_glyph(char)` - Check character coverage
- `glyph_paths(char)` - Return stroke commands for rendering
- `advance(char)` - Character width for layout

## Design Principles

1. **Pure Rust** - No platform dependencies (wasm, webgpu, web-sys)
2. **Stroke-based** - Fonts defined as line segments, not filled shapes
3. **Stylistically distinct** - Each font should evoke a different arcade era/manufacturer
4. **Minimal allocation** - Glyph data should be static where possible
