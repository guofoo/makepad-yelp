# Yelp UI Redesign Implementation Plan

**Status:** COMPLETED

**Goal:** Redesign the Makepad Yelp clone to match the authentic Yelp mobile app visual style.

**Architecture:** All widgets implemented in `src/app.rs` with custom shaders for star ratings, pill-shaped buttons, and UI elements.

**Tech Stack:** Rust, Makepad (makepad-widgets), custom Sdf2d shaders.

---

## Reference: Yelp Visual Style

- **Colors:** Yelp Red `#d32323`, Star Red `#f43939`, Text Black `#1a1a1a`, Gray `#666666`, Light Gray `#999999`
- **Tab Bar:** Pill-shaped buttons with icons + text, active = red background with white text, inactive = transparent with gray text
- **Business Card:** Large photo (110px square), bold name, 5-pointed star rating, numeric rating, review count, metadata row, pill-shaped category tags
- **Star Rating:** 5-pointed stars using polar coordinate SDF, filled = red (#f43939), empty = light gray (#e0e0e0)

---

## Completed Tasks

### Task 1: Star Shape Shader
- Implemented 5-pointed star rating using polar coordinate SDF formula
- Stars fill based on rating value (0-5)
- Location: `src/app.rs` StarRating widget

### Task 2: Business Card Layout
- 110x110px photo placeholder with food icon silhouette
- Name + distance row
- Star rating + numeric rating + review count row
- Meta info (city, price)
- Pill-shaped category tags with border
- Hover animation effect
- Location: `src/app.rs` BusinessCard widget

### Task 3: Tab Bar with Pill Buttons
- Pill/stadium shaped buttons (circle-rect-circle SDF pattern from makepad-badger)
- Search icon (magnifying glass) and Map icon (location pin) using Sdf2d
- Active tab: Yelp red background, white icon/text
- Inactive tab: Transparent background, gray icon/text
- Dynamic color updates on tab switch
- Location: `src/app.rs` YelpTabBar widget

### Task 4: Search Screen Header
- Yelp-style search bar with rounded corners
- Search icon + "Restaurants" + divider + "Current Location"
- White header background
- Location: `src/app.rs` SearchScreen widget

### Task 5: Additional Polish
- Category tags updated to pill shape matching tab bar style
- Consistent use of stadium/pill shape across UI (inspired by makepad-badger)
- All interactions working: tab switching, business card clicks, detail view navigation

### Task 6: Network Image Loading
- HTTP request on app startup to fetch restaurant image from `picsum.photos/320/240`
- Global `OnceLock<Vec<u8>>` static to store image bytes across widgets
- BusinessCard loads JPEG/PNG data into Image widget with rounded corners
- Custom `draw_bg` shader on Image for rounded corner display with placeholder fallback
- Location: `src/app.rs` App (handle_startup, handle_network_responses), BusinessCard (draw_walk)

---

## File Structure

```
src/
  app.rs          # Main implementation (all widgets, screens, app logic)
  lib.rs          # Library exports
  main.rs         # Entry point
```

**Note:** Legacy directories (`widgets/`, `screens/`, `data/`) have been cleaned up and removed.

---

## Summary

| Task | Status | Key Implementation |
|------|--------|-------------------|
| Star Rating | Done | 5-pointed star SDF using polar coordinates |
| Business Card | Done | Yelp-style layout with photo, ratings, tags |
| Tab Bar | Done | Pill-shaped buttons with SDF icons |
| Search Header | Done | Rounded search bar with icon |
| Category Tags | Done | Pill shape with border |
| Network Images | Done | HTTP image loading with OnceLock static |
