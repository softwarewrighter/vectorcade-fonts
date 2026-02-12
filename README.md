# vectorcade-fonts

Vector glyph rendering data + font styles for VectorCade.

Goal: provide *distinct* vector-ish alphanumeric looks per game/demo.

This repo depends on `vectorcade-shared` and implements `VectorFont`.

## Build

```bash
cargo test
```

## Scope (Agent B focus)

- Implement 3–5 `VectorFont` styles:
  - `ATARI` (boxy, utilitarian)
  - `CINEMATRONICS` (thin, angular)
  - `MIDWAY` (slightly rounded)
  - `VECTOR_SCANLINE` (broken segments for "beam jitter")

- Provide a `FontRegistry` to pick styles by `FontStyleId`.
