# Agent Guide

This repo is part of a multi-repo DAG:

- `vectorcade-shared` (root) → shared types + traits
- `vectorcade-fonts` → vector glyph/stroke fonts (depends on shared)
- `vectorcade-games` → game logic crates (depends on shared + fonts)
- `vectorcade-render-wgpu` → renderer backend (depends on shared)
- `vectorcade-web-yew` → integration app (depends on shared + games + renderer)

## Rules of engagement

- Keep `vectorcade-shared` pure (no wasm/webgpu/web-sys).
- Prefer deterministic logic (fixed timestep, seeded RNG) in games.
- All rendering must go through `DrawCmd` (display-list). No game touches wgpu.

## Local multi-repo dev (recommended)

Clone all repos into the same parent directory:

```
sw-fun/
  vectorcade-shared/
  vectorcade-fonts/
  vectorcade-games/
  vectorcade-render-wgpu/
  vectorcade-web-yew/
```

Library repos (shared, fonts, render-wgpu) use **direct path dependencies** in
their Cargo.toml files. Only the integration repo (vectorcade-games) should use
`.cargo/config.toml` patches since it's the downstream consumer of all libraries.

This avoids "patch was not used" warnings and keeps dependency flow clear.
