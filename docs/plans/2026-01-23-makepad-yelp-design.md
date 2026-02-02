# makepad-yelp Design

A Yelp clone built with Makepad framework - both a learning showcase and functional app.

## Status

**MVP Complete** - App has working search list, map view, business details, and tab navigation.

## Goals

1. **Learning/Demo** - Showcase Makepad's capabilities (smooth animations, custom shaders, cross-platform)
2. **Functional Clone** - Working app with mock data (Yelp API integration planned)

## Platforms

- Desktop first (mobile-style UI at 390x844 iPhone size)
- Same codebase runs on iOS, Android, WASM

## Features (MVP)

| Feature | Status | Description |
|---------|--------|-------------|
| Search + List | Done | Business listings with photos/ratings/distance |
| Business Details | Done | Full page with ratings, contact info, action buttons |
| Map View | Done | Interactive map with business markers (uses makepad-map) |
| Tab Navigation | Done | Pill-shaped tab bar with icons |
| Star Ratings | Done | 5-pointed star shader with polar SDF |

**Out of scope for MVP:** Filters, user accounts, reviews submission, Yelp API

---

## Current Project Structure

```
makepad-yelp/
├── Cargo.toml
├── src/
│   ├── app.rs              # All widgets, screens, and app logic
│   ├── lib.rs              # Library exports
│   └── main.rs             # Entry point
├── docs/
│   └── plans/
│       ├── 2026-01-23-makepad-yelp-design.md  # This file
│       └── 2026-01-26-yelp-ui-redesign.md     # UI redesign plan (completed)
└── resources/              # (planned) SVG icons
```

**Note:** The app uses a single `app.rs` file containing all widgets, screens, and logic. This makes it easier to understand and modify during development.

## Dependencies

```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }
makepad-map = { path = "../makepad-map" }
```

---

## UI Layout

Mobile-style layout with bottom tab bar:

```
┌─────────────────────────────┐
│  Search Bar (sticky top)    │
├─────────────────────────────┤
│                             │
│   Business List / Map       │
│   (scrollable content)      │
│                             │
├─────────────────────────────┤
│ [🔍 Search] [🗺 Map]        │  ← Pill-shaped Tab Bar
└─────────────────────────────┘
```

---

## Data Model

```rust
#[derive(Clone, Debug)]
pub struct Business {
    pub id: String,
    pub name: String,
    pub rating: f32,           // 1.0 - 5.0
    pub review_count: u32,
    pub price: Option<String>, // "$", "$$", "$$$", "$$$$"
    pub categories: Vec<String>,
    pub city: String,
    pub distance_meters: Option<f64>,
    pub lat: f64,
    pub lng: f64,
}
```

---

## Key Visual Features

| Feature | Implementation |
|---------|----------------|
| Star rating | 5-pointed stars using polar coordinate SDF shader |
| Tab bar | Pill/stadium shaped buttons (circle-rect-circle SDF) |
| Category tags | Pill shaped with border |
| Business card | Hover animation, 110px network-loaded photo with rounded corners |
| Map markers | Yelp red pins with labels |
| Search bar | Rounded rectangle with search icon |
| Network images | HTTP request + OnceLock static + JPEG/PNG decoding |

## Visual Style

- **Colors:** Yelp red (#d32323) as accent, white backgrounds, dark text (#1a1a1a)
- **Typography:** System fonts, 17px names, 13-14px body, 12px tags
- **Spacing:** 16px card padding, 8px element spacing
- **Corners:** 8px border radius on cards/inputs, pill shape on tabs/tags

---

## Running the App

```bash
cargo run
```

The app opens at 1280x800 with the search screen showing mock San Francisco restaurants.
