# Product Requirements Document

## Overview

vectorcade-fonts provides vector stroke fonts for the VectorCade arcade game platform. The goal is to give each game/demo a distinct visual identity through typography that evokes classic arcade aesthetics.

## Goals

1. **Authenticity** - Fonts should feel appropriate for classic vector arcade games
2. **Variety** - Multiple distinct styles so games don't look identical
3. **Completeness** - Full alphanumeric coverage for scores, labels, messages
4. **Performance** - Minimal overhead; fonts are rendered every frame

## Required Font Styles

| Style ID | Aesthetic | Inspired By |
|----------|-----------|-------------|
| `ATARI` | Boxy, utilitarian | Asteroids, Lunar Lander |
| `CINEMATRONICS` | Thin, angular | Star Castle, Armor Attack |
| `MIDWAY` | Slightly rounded | Omega Race |
| `VECTOR_SCANLINE` | Broken segments, "beam jitter" | Worn CRT phosphor effect |

## Character Coverage

### Minimum (MVP)
- Digits: `0-9`
- Letters: `A-Z` (uppercase only)
- Punctuation: space, period, comma, colon, hyphen

### Extended (Post-MVP)
- Lowercase: `a-z`
- Symbols: `!`, `?`, `@`, `#`, `$`, `%`, `*`, `+`, `=`, `/`

## Non-Goals

- Filled/solid fonts (stroke only)
- Unicode beyond ASCII
- Font file loading (all fonts are code-defined)
- Anti-aliasing (handled by renderer)

## Success Criteria

1. All 4 font styles implemented with minimum character coverage
2. Each style visually distinguishable at game-typical sizes
3. Registry lookup by `FontStyleId` works correctly
4. No platform-specific dependencies
