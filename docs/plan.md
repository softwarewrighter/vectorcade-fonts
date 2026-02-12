# Implementation Plan

## Phase 1: Complete ATARI Style (Current)

**Goal**: Full alphanumeric coverage for the existing AtariMini font.

### Tasks
- [ ] Add digits 2-9 (currently only 0, 1)
- [ ] Add letters A-Z
- [ ] Add basic punctuation (. , : -)
- [ ] Update tests for coverage validation

### Design Notes
- Boxy, utilitarian aesthetic
- Right angles preferred
- Uniform stroke segments
- Inspired by Asteroids/Lunar Lander score displays

## Phase 2: CINEMATRONICS Style

**Goal**: Second font with distinctly different aesthetic.

### Tasks
- [ ] Create `Cinematronics` struct in styles.rs
- [ ] Implement full alphanumeric set
- [ ] Register in tests
- [ ] Add visual comparison test/example

### Design Notes
- Thin, angular letterforms
- Heavy use of 45° diagonals
- Taller, narrower proportions than ATARI
- Inspired by Star Castle, Armor Attack

## Phase 3: MIDWAY Style

**Goal**: Third font with rounded characteristics.

### Tasks
- [ ] Create `Midway` struct in styles.rs
- [ ] Implement full alphanumeric set
- [ ] Approximate curves with 3-4 segment polylines

### Design Notes
- Slightly rounded corners
- Wider, friendlier proportions
- Curves approximated with short line segments
- Inspired by Omega Race

## Phase 4: VECTOR_SCANLINE Style

**Goal**: Special effect font simulating worn CRT phosphor.

### Tasks
- [ ] Create `VectorScanline` struct in styles.rs
- [ ] Implement as wrapper/modifier of base ATARI style
- [ ] Add configurable gap frequency/size

### Design Notes
- Same base shapes as ATARI
- Segments broken with small gaps
- Simulates "beam jitter" or phosphor wear
- May need randomization seed for variety

## Phase 5: Polish & Integration

### Tasks
- [ ] Performance review (allocation patterns)
- [ ] Documentation polish
- [ ] Integration test with vectorcade-games
- [ ] Example binary showing all fonts

## Dependencies

| Phase | Blocked By |
|-------|------------|
| 1 | None (can start immediately) |
| 2 | Phase 1 (establishes patterns) |
| 3 | Phase 1 |
| 4 | Phase 1 (uses ATARI as base) |
| 5 | Phases 1-4 |

Phases 2 and 3 can proceed in parallel after Phase 1.
