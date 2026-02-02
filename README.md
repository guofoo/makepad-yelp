# Makepad Yelp Clone

A Yelp mobile app clone built with [Makepad](https://github.com/makepad/makepad) - a cross-platform UI framework written in Rust.

## Features

- **Business Listings** - Scrollable list of restaurants with photos, ratings, and metadata
- **Star Ratings** - Custom 5-pointed star shader using polar coordinate SDF
- **Business Details** - Full detail screen with hero image, ratings, and action buttons
- **Interactive Map** - Map view with business markers using makepad-map
- **Tab Navigation** - Pill-shaped tab bar with Search and Map tabs
- **Network Images** - Async image loading from network with per-business unique photos
- **Hover Animations** - Smooth hover and press states on interactive elements

## Tech Stack

- **Rust** - Systems programming language
- **Makepad Widgets** - Cross-platform UI framework
- **Makepad Map** - Map component for geographic visualization
- **Custom Sdf2d Shaders** - GPU-accelerated shapes (stars, pills, rounded corners)

## Running the App

```bash
# Clone the repository
git clone <repo-url>
cd makepad-yelp

# Run the app
cargo run
```

The app opens at 1280x800 with the search screen showing mock San Francisco restaurants.

## Project Structure

```
makepad-yelp/
├── Cargo.toml              # Dependencies
├── README.md               # This file
├── src/
│   ├── app.rs              # All widgets, screens, and app logic
│   ├── lib.rs              # Library exports
│   └── main.rs             # Entry point
└── docs/
    └── plans/              # Design documents
        ├── 2026-01-23-makepad-yelp-design.md
        └── 2026-01-26-yelp-ui-redesign.md
```

## Architecture

The app uses a single-file architecture (`src/app.rs`) containing:

### Widgets
- `StarRating` - 5-pointed star rating display
- `SearchBar` - Rounded search input
- `BusinessCard` - Restaurant card with photo, info, and tags
- `YelpTabBar` - Bottom tab navigation with pill-shaped buttons

### Screens
- `SearchScreen` - Business list with PortalList for efficient scrolling
- `MapScreen` - Interactive map with business markers
- `BusinessDetailScreen` - Full business details with Call/Directions buttons

### Data
- `Business` - Restaurant data model
- `mock_businesses()` - Sample SF restaurants

## Visual Style

| Element | Implementation |
|---------|----------------|
| Primary Color | Yelp Red `#d32323` |
| Star Rating | Polar coordinate SDF shader |
| Tab/Button Shape | Pill/stadium (circle-rect-circle SDF) |
| Category Tags | Pill shape with border |
| Business Photos | Network-loaded with rounded corners |

## Key Interactions

1. **Search Tab** - Browse restaurant list, click card to view details
2. **Map Tab** - View restaurants on map, tap marker to view details
3. **Detail Screen** - View full info, tap "Directions" to open map
4. **Navigation** - Back buttons and tab bar for screen navigation

## Dependencies

```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }
makepad-map = { path = "../makepad-map" }
```

## Platform Support

Built with Makepad's cross-platform capabilities:
- Desktop (macOS, Windows, Linux)
- Mobile (iOS, Android)
- Web (WASM)

## License

MIT
