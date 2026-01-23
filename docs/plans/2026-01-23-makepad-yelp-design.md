# makepad-yelp Design

A Yelp clone built with Makepad framework - both a learning showcase and functional app.

## Goals

1. **Learning/Demo** - Showcase Makepad's capabilities (smooth animations, custom shaders, cross-platform)
2. **Functional Clone** - Working app with real Yelp API integration

## Platforms

- Desktop first (mobile-style UI at 390x844 iPhone size)
- Same codebase runs on iOS, Android, WASM

## Features (MVP)

| Feature | Description |
|---------|-------------|
| Search + List | Search bar, business listings with photos/ratings/distance |
| Business Details | Full page with hours, contact info, action buttons |
| Map View | Interactive map with business markers (uses makepad-map) |

**Out of scope for MVP:** Filters, user accounts, reviews submission

## Data Strategy

- **Mock data** for development and demos (works offline)
- **Yelp Fusion API** as optional feature (requires API key)

---

## Project Structure

```
makepad-yelp/
├── Cargo.toml
├── src/
│   ├── app.rs              # App shell, tab bar, navigation stack
│   ├── lib.rs              # Live design registration
│   ├── main.rs             # Entry point
│   ├── data/
│   │   ├── mod.rs
│   │   ├── business.rs     # Business struct, mock data
│   │   └── yelp_api.rs     # Optional Yelp Fusion client
│   ├── screens/
│   │   ├── mod.rs
│   │   ├── search.rs       # Search screen (list view)
│   │   ├── map.rs          # Map screen (integrates makepad-map)
│   │   └── details.rs      # Business detail screen
│   └── widgets/
│       ├── mod.rs
│       ├── business_card.rs    # List item card
│       ├── rating_stars.rs     # Star rating with shader
│       ├── search_bar.rs       # Search input
│       └── tab_bar.rs          # Bottom navigation
└── resources/
    └── icons/                  # SVG icons for tabs, UI
```

## Dependencies

```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }
makepad-map = { path = "../makepad-map" }
```

---

## Navigation

Mobile-style layout with bottom tab bar and stack navigation:

```
┌─────────────────────────────┐
│  Search Bar (sticky top)    │
├─────────────────────────────┤
│                             │
│   Business List / Map       │
│   (scrollable content)      │
│                             │
├─────────────────────────────┤
│ [🔍 Search] [🗺 Map] [👤 Me] │  ← Tab Bar
└─────────────────────────────┘
```

### App DSL Structure

```rust
live_design! {
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { inner_size: vec2(390, 844) }  // iPhone 14 size
                body = <View> {
                    width: Fill, height: Fill
                    flow: Down

                    stack_nav = <StackNavigation> {
                        root_view = <View> {
                            width: Fill, height: Fill
                            flow: Down

                            tab_content = <View> {
                                width: Fill, height: Fill
                                // Screens swap here based on tab
                            }

                            tab_bar = <TabBar> { }
                        }

                        details_view = <StackNavigationView> {
                            <DetailsScreen> { }
                        }
                    }
                }
            }
        }
    }
}
```

- **Tab switching:** Tab bar emits actions; app swaps Search/Map screen visibility
- **Details push:** Tapping business card calls `stack_nav.push()` with data via `Scope::with_data()`

---

## Data Model

```rust
#[derive(Clone, Debug)]
pub struct Business {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub rating: f32,           // 1.0 - 5.0
    pub review_count: u32,
    pub price: Option<String>, // "$", "$$", "$$$", "$$$$"
    pub categories: Vec<String>,
    pub address: String,
    pub city: String,
    pub distance_meters: Option<f64>,
    pub latitude: f64,
    pub longitude: f64,
    pub phone: Option<String>,
    pub hours: Option<Vec<DayHours>>,
    pub is_open_now: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct DayHours {
    pub day: u8,        // 0 = Monday
    pub start: String,  // "0900"
    pub end: String,    // "2100"
}
```

### App State

```rust
pub struct AppState {
    pub businesses: Vec<Business>,
    pub selected_business: Option<Business>,
    pub search_query: String,
    pub current_location: Option<(f64, f64)>,
}
```

---

## Key Widgets

### BusinessCard

```
┌─────────────────────────────────────┐
│ ┌─────────┐  Flour + Water          │
│ │  Photo  │  ★★★★☆ 4.5 (4521)       │
│ │  80x80  │  $$$ · Italian · 0.5 mi │
│ └─────────┘  Mission District       │
└─────────────────────────────────────┘
```

```rust
live_design! {
    BusinessCard = {{BusinessCard}} {
        width: Fill, height: Fit
        padding: 12.0
        flow: Right
        spacing: 12.0

        animator: {
            pressed = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {opacity: 1.0}} }
                on = { from: {all: Forward {duration: 0.05}} apply: {draw_bg: {opacity: 0.7}} }
            }
        }

        photo = <Image> {
            width: 80.0, height: 80.0
            fit: Cover
            draw_bg: { border_radius: 4.0 }
        }

        info = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4.0

            name = <Label> {
                draw_text: { text_style: { font_size: 16.0 }, color: #1a1a1a }
            }
            rating_row = <View> {
                flow: Right, spacing: 4.0
                stars = <RatingStars> { }
                rating_text = <Label> { draw_text: { color: #666 } }
            }
            meta = <Label> {
                draw_text: { text_style: { font_size: 13.0 }, color: #666 }
            }
            location = <Label> {
                draw_text: { text_style: { font_size: 13.0 }, color: #999 }
            }
        }
    }
}
```

### RatingStars (Custom Shader)

GPU-rendered stars with smooth partial fills using Yelp's signature red (#d32323):

```rust
live_design! {
    RatingStars = {{RatingStars}} {
        width: 80.0, height: 16.0

        draw_stars: {
            instance rating: 0.0
            fn pixel(self) -> vec4 {
                let star_color = vec4(0.83, 0.14, 0.14, 1.0); // Yelp red
                let empty_color = vec4(0.8, 0.8, 0.8, 1.0);

                let star_idx = floor(self.pos.x * 5.0);
                let fill = clamp(self.rating - star_idx, 0.0, 1.0);

                // Star shape via SDF
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // ... star path

                return mix(empty_color, star_color, fill);
            }
        }
    }
}
```

---

## Screens

### SearchScreen

```rust
live_design! {
    SearchScreen = {{SearchScreen}} {
        width: Fill, height: Fill
        flow: Down

        header = <View> {
            width: Fill, height: Fit
            padding: {top: 8.0, bottom: 8.0, left: 12.0, right: 12.0}
            show_bg: true
            draw_bg: { color: #fff }

            search_bar = <SearchBar> { }
        }

        list = <PortalList> {
            width: Fill, height: Fill
            flow: Down
            BusinessCard = <BusinessCard> { }
        }
    }
}
```

### MapScreen

```rust
live_design! {
    MapScreen = {{MapScreen}} {
        width: Fill, height: Fill
        flow: Overlay

        map = <GeoMapView> {
            width: Fill, height: Fill
            latitude: 37.7749
            longitude: -122.4194
            zoom: 13.0
        }

        <View> {
            width: Fill, height: Fit
            padding: 12.0
            search_bar = <SearchBar> { }
        }
    }
}
```

### DetailsScreen

```
┌─────────────────────────────┐
│ ← Back        Flour + Water │
├─────────────────────────────┤
│ ┌─────────────────────────┐ │
│ │      Hero Photo         │ │
│ └─────────────────────────┘ │
│ ★★★★☆ 4.5 · 4521 reviews   │
│ $$$ · Italian · Pizza       │
├─────────────────────────────┤
│ 📍 2401 Harrison St         │
│ 📞 (415) 826-0399           │
│ 🕐 Open until 10 PM         │
├─────────────────────────────┤
│ [ Call ]  [ Directions ]    │
└─────────────────────────────┘
```

Receives `Business` via `Scope` and populates fields in `draw_walk`.

---

## Makepad Showcase Features

| Feature | Implementation |
|---------|----------------|
| Smooth scrolling | `PortalList` with 60fps virtualization |
| Tap feedback | Animator with pressed state (opacity change) |
| Star rating | Custom GPU shader with partial fills |
| Search focus | Animated border color transition |
| Tab indicator | Sliding underline animation |
| Map | `makepad-map` with pan/zoom/markers |
| Screen transitions | `StackNavigation` slide animation |

---

## Visual Style

- **Colors:** Yelp red (#d32323) as accent, white backgrounds, dark text
- **Typography:** System fonts, 16px body, 13px secondary
- **Spacing:** 12px standard padding, 8px between elements
- **Corners:** 4px border radius on cards/images

---

## Implementation Order

1. Project setup (Cargo.toml, basic app shell)
2. Data model + mock businesses
3. Tab bar + screen switching
4. BusinessCard widget
5. SearchScreen with list
6. DetailsScreen
7. MapScreen integration
8. RatingStars shader polish
9. Yelp API integration (optional)
