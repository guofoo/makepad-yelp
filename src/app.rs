use makepad_widgets::*;
use makepad_map::GeoMapViewWidgetExt;
use std::sync::{LazyLock, Mutex};
use std::collections::HashMap;

// Global storage for restaurant images by business ID
static RESTAURANT_IMAGES: LazyLock<Mutex<HashMap<String, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323
    STAR_YELLOW = #f8b84e

    // Loading Spinner - animated rotating arc using time-based animation
    LoadingSpinner = {{LoadingSpinner}} {
        width: 40.0
        height: 40.0
        show_bg: true
        draw_bg: {
            instance spin_time: 0.0
            instance opacity: 1.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let center = self.rect_size * 0.5;
                let radius = min(center.x, center.y) - 4.0;

                // Draw arc (partial circle)
                let angle = self.pos * 2.0 - 1.0;
                let a = atan(angle.y - 0.0, angle.x - 0.0);
                let pi = 3.14159265;

                // Rotate based on time (continuous rotation)
                let rotation = self.spin_time;
                let rotated_angle = a - rotation * pi * 2.0;
                let normalized = mod(rotated_angle + pi, 2.0 * pi) - pi;

                // Arc spans about 270 degrees
                let in_arc = step(-pi * 0.75, normalized) * step(normalized, pi * 0.75);

                // Draw circle outline (track)
                sdf.circle(center.x, center.y, radius);
                sdf.stroke(vec4(0.88, 0.88, 0.88, self.opacity), 3.0);

                // Draw colored arc on top
                sdf.circle(center.x, center.y, radius);
                let arc_color = vec4(0.827, 0.137, 0.137, in_arc * self.opacity); // YELP_RED with arc mask
                sdf.stroke(arc_color, 3.0);

                return sdf.result;
            }
        }
    }

    // Star Rating widget - proper 5-pointed stars
    StarRating = {{StarRating}} {
        width: Fit
        height: 20.0
        show_bg: true
        draw_bg: {
            instance rating: 0.0

            fn pixel(self) -> vec4 {
                let filled = vec4(1.0, 0.72, 0.0, 1.0);       // #FFB800 gold
                let empty = vec4(0.6, 0.6, 0.6, 1.0);          // #999 medium gray
                let outline_color = vec4(0.35, 0.35, 0.35, 1.0);

                // Each star takes 1/5 of width
                let star_idx = floor(self.pos.x * 5.0);
                let local_x = fract(self.pos.x * 5.0);

                // Center point in local star space
                let cx = local_x - 0.5;
                let cy = self.pos.y - 0.5;

                // Convert to polar coordinates
                let angle = atan(cy, cx);
                let r = length(vec2(cx, cy));

                // 5-pointed star: alternating outer and inner radius
                let pi = 3.14159265;
                let points = 5.0;
                let outer_r = 0.42;
                let inner_r = 0.17;

                // Calculate which segment we're in
                let segment_angle = pi / points;
                let a = mod(angle + pi + segment_angle / 2.0, 2.0 * segment_angle) - segment_angle;

                // Interpolate between outer and inner radius
                let t = abs(a) / segment_angle;
                let star_radius = mix(outer_r, inner_r, t);

                // Anti-aliased edges using smoothstep over ~1.5px
                let aa = 1.5 / self.rect_size.y;
                let inside = smoothstep(star_radius + aa, star_radius - aa, r);
                let outline_edge = star_radius + 0.025;
                let outline_band = smoothstep(outline_edge + aa, outline_edge - aa, r);

                let is_filled = step(star_idx + 0.5, self.rating);
                let col = mix(empty, filled, is_filled);

                // Blend: outline ring behind, star fill on top
                let ring = outline_band * (1.0 - inside);
                let final_color = mix(outline_color.rgb, col.rgb, inside);
                let alpha = max(inside, ring * 0.6);
                return vec4(final_color, alpha);
            }
        }
    }

    // Search Bar widget
    SearchBar = {{SearchBar}} {
        width: Fill
        height: 44.0
        show_bg: true
        draw_bg: {
            color: #f5f5f5
            instance border_color: #e0e0e0
            instance border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
        flow: Right
        align: { y: 0.5 }
        padding: { left: 12.0, right: 12.0 }
        spacing: 8.0

        <Label> {
            width: Fit, height: Fit
            text: "Search"
            draw_text: { text_style: { font_size: 14.0 }, color: #999 }
        }
        input = <TextInput> {
            width: Fill, height: Fit
            empty_text: "Restaurants, bars, cafes..."
            draw_bg: { color: #0000 }
            draw_text: { text_style: { font_size: 14.0 }, color: #333 }
        }
    }

    // Business Card widget with Yelp-style layout
    BusinessCard = {{BusinessCard}} {
        width: Fill
        height: Fit
        padding: { top: 16.0, bottom: 16.0, left: 16.0, right: 16.0 }
        flow: Right
        spacing: 16.0
        show_bg: true
        draw_bg: {
            color: #fff
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                return mix(self.color, #f5f5f5, self.hover);
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }

        cursor: Hand

        // Photo container with loading spinner overlay
        photo_container = <View> {
            width: 110.0, height: 110.0
            flow: Overlay
            align: { x: 0.5, y: 0.5 }

            photo = <Image> {
                width: Fill, height: Fill
                fit: Smallest
                draw_bg: {
                    instance radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
                        let color = self.get_color();
                        // Show placeholder color if no image loaded (alpha = 0)
                        let placeholder = vec4(0.93, 0.91, 0.89, 1.0);
                        let final_color = mix(placeholder, color, color.w);
                        sdf.fill(vec4(final_color.xyz, 1.0));
                        return sdf.result;
                    }
                }
            }

            // Loading spinner shown while image loads
            photo_spinner = <LoadingSpinner> {
                width: 36.0, height: 36.0
                draw_bg: {
                    instance opacity: 1.0
                }
            }
        }

        info = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4.0

            // Top row: name + distance
            name_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: { y: 0.5 }

                name_label = <Label> {
                    width: Fill, height: Fit
                    draw_text: {
                        text_style: { font_size: 17.0 }
                        color: #1a1a1a
                    }
                    text: "Business Name"
                }
                distance_label = <Label> {
                    width: Fit, height: Fit
                    draw_text: {
                        text_style: { font_size: 13.0 }
                        color: #666
                    }
                    text: "2.5 mi"
                }
            }

            // Rating row: stars + numeric + reviews
            rating_row = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 6.0
                align: { y: 0.5 }

                stars = <StarRating> {
                    width: 100.0, height: 20.0
                }
                rating_num = <Label> {
                    width: Fit, height: Fit
                    draw_text: { text_style: { font_size: 14.0 }, color: #1a1a1a }
                    text: "4.2"
                }
                review_count = <Label> {
                    width: Fit, height: Fit
                    draw_text: { text_style: { font_size: 14.0 }, color: #666 }
                    text: "(249 reviews)"
                }
            }

            // Meta row: location · price · status
            meta_label = <Label> {
                width: Fill, height: Fit
                draw_text: { text_style: { font_size: 13.0 }, color: #666 }
                text: "Milpitas · $$$ · Closed until 11:00 AM"
            }

            // Category tags
            tags_row = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8.0
                margin: { top: 8.0 }

                tag1 = <RoundedView> {
                    width: Fit, height: Fit
                    padding: { top: 6.0, bottom: 6.0, left: 14.0, right: 14.0 }
                    show_bg: true
                    draw_bg: {
                        color: #fff
                        instance border_color: #e0e0e0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let w = self.rect_size.x;
                            let h = self.rect_size.y;
                            let r = h * 0.5;
                            // Outer pill (border)
                            sdf.circle(r, r, r);
                            sdf.fill(self.border_color);
                            sdf.rect(r, 0.0, w - h, h);
                            sdf.fill(self.border_color);
                            sdf.circle(w - r, r, r);
                            sdf.fill(self.border_color);
                            // Inner pill (fill)
                            sdf.circle(r, r, r - 1.0);
                            sdf.fill(self.color);
                            sdf.rect(r, 1.0, w - h, h - 2.0);
                            sdf.fill(self.color);
                            sdf.circle(w - r, r, r - 1.0);
                            sdf.fill(self.color);
                            return sdf.result;
                        }
                    }
                    tag1_label = <Label> {
                        width: Fit, height: Fit
                        draw_text: { text_style: { font_size: 12.0 }, color: #1a1a1a }
                        text: "Italian"
                    }
                }
            }
        }
    }

    // Tab Bar widget with icons
    YelpTabBar = {{YelpTabBar}} {
        width: Fill
        height: 60.0
        show_bg: true
        draw_bg: {
            color: #fff
            instance border_color: #e0e0e0
            fn pixel(self) -> vec4 {
                if self.pos.y < 1.0 / self.rect_size.y { return self.border_color; }
                return self.color;
            }
        }
        flow: Right
        padding: { top: 8.0, bottom: 8.0, left: 16.0, right: 16.0 }
        spacing: 16.0

        search_tab = <RoundedView> {
            width: Fill, height: Fill
            flow: Right
            align: { x: 0.5, y: 0.5 }
            spacing: 8.0
            padding: { left: 16.0, right: 16.0 }
            cursor: Hand
            show_bg: true
            draw_bg: {
                instance bg_color: (YELP_RED)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let w = self.rect_size.x;
                    let h = self.rect_size.y;
                    let r = h * 0.5;
                    // Pill/stadium shape: circle-rect-circle
                    sdf.circle(r, h * 0.5, r);
                    sdf.fill(self.bg_color);
                    sdf.rect(r, 0.0, w - h, h);
                    sdf.fill(self.bg_color);
                    sdf.circle(w - r, h * 0.5, r);
                    sdf.fill(self.bg_color);
                    return sdf.result;
                }
            }

            search_icon = <RoundedView> {
                width: 20.0, height: 20.0
                show_bg: true
                draw_bg: {
                    instance icon_color: #fff
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Magnifying glass circle
                        sdf.circle(8.0, 8.0, 5.0);
                        sdf.stroke(self.icon_color, 1.8);
                        // Handle
                        sdf.move_to(12.0, 12.0);
                        sdf.line_to(17.0, 17.0);
                        sdf.stroke(self.icon_color, 2.0);
                        return sdf.result;
                    }
                }
            }
            search_label = <Label> {
                width: Fit, height: Fit
                draw_text: { text_style: { font_size: 13.0 }, color: #fff }
                text: "Search"
            }
        }

        map_tab = <RoundedView> {
            width: Fill, height: Fill
            flow: Right
            align: { x: 0.5, y: 0.5 }
            spacing: 8.0
            padding: { left: 16.0, right: 16.0 }
            cursor: Hand
            show_bg: true
            draw_bg: {
                instance bg_color: #0000
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let w = self.rect_size.x;
                    let h = self.rect_size.y;
                    let r = h * 0.5;
                    // Pill/stadium shape: circle-rect-circle
                    sdf.circle(r, h * 0.5, r);
                    sdf.fill(self.bg_color);
                    sdf.rect(r, 0.0, w - h, h);
                    sdf.fill(self.bg_color);
                    sdf.circle(w - r, h * 0.5, r);
                    sdf.fill(self.bg_color);
                    return sdf.result;
                }
            }

            map_icon = <RoundedView> {
                width: 20.0, height: 20.0
                show_bg: true
                draw_bg: {
                    instance icon_color: #666
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let cx = 10.0;
                        let cy = 8.0;
                        // Location pin using strokes (like search icon)
                        // Outer circle
                        sdf.circle(cx, cy, 5.5);
                        sdf.stroke(self.icon_color, 2.0);
                        // Inner dot
                        sdf.circle(cx, cy, 1.5);
                        sdf.fill(self.icon_color);
                        // Pin stem (line down)
                        sdf.move_to(cx, cy + 5.5);
                        sdf.line_to(cx, 18.0);
                        sdf.stroke(self.icon_color, 2.0);
                        return sdf.result;
                    }
                }
            }
            map_label = <Label> {
                width: Fit, height: Fit
                draw_text: { text_style: { font_size: 13.0 }, color: #666 }
                text: "Map"
            }
        }
    }

    // Search Screen
    SearchScreen = {{SearchScreen}} {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance opacity: 1.0
            fn pixel(self) -> vec4 {
                return vec4(0.96, 0.96, 0.96, self.opacity);
            }
        }
        animator: {
            fade = {
                default: show
                hide = {
                    from: { all: Forward { duration: 0.3 } }
                    apply: { draw_bg: { opacity: 0.0 } }
                }
                show = {
                    from: { all: Forward { duration: 0.3 } }
                    apply: { draw_bg: { opacity: 1.0 } }
                }
            }
        }

        header = <View> {
            width: Fill, height: Fit
            padding: { top: 12.0, bottom: 12.0, left: 16.0, right: 16.0 }
            show_bg: true
            draw_bg: { color: #fff }

            search_bar = <RoundedView> {
                width: Fill, height: 44.0
                show_bg: true
                draw_bg: {
                    color: #f0f0f0
                    instance radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
                        sdf.fill(self.color);
                        return sdf.result;
                    }
                }
                flow: Right
                align: { y: 0.5 }
                padding: { left: 12.0, right: 12.0 }
                spacing: 8.0

                // Search icon
                <View> {
                    width: 20.0, height: 20.0
                    show_bg: true
                    draw_bg: {
                        instance icon_color: #999
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let c = self.rect_size * 0.5;
                            sdf.circle(c.x - 1.0, c.y - 1.0, 6.0);
                            sdf.stroke(self.icon_color, 1.5);
                            sdf.move_to(c.x + 3.0, c.y + 3.0);
                            sdf.line_to(c.x + 8.0, c.y + 8.0);
                            sdf.stroke(self.icon_color, 1.5);
                            return sdf.result;
                        }
                    }
                }

                <Label> {
                    width: Fit, height: Fit
                    text: "Restaurants"
                    draw_text: { text_style: { font_size: 15.0 }, color: #1a1a1a }
                }
                <View> { width: 1.0, height: 20.0, show_bg: true, draw_bg: { color: #ccc } }
                <Label> {
                    width: Fit, height: Fit
                    text: "Current Location"
                    draw_text: { text_style: { font_size: 15.0 }, color: #999 }
                }
            }
        }

        // Divider
        <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #e0e0e0 } }

        // Pull to refresh indicator (shown when refreshing)
        refresh_indicator = <View> {
            width: Fill, height: 50.0
            align: { x: 0.5, y: 0.5 }
            show_bg: true
            draw_bg: { color: #f5f5f5 }
            visible: false

            <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 8.0
                align: { y: 0.5 }

                refresh_spinner = <LoadingSpinner> {
                    width: 24.0, height: 24.0
                }
                <Label> {
                    text: "Refreshing..."
                    draw_text: { text_style: { font_size: 13.0 }, color: #666 }
                }
            }
        }

        // Empty state (shown when no results)
        empty_state = <View> {
            width: Fill, height: Fill
            visible: false
            align: { x: 0.5, y: 0.5 }
            flow: Down
            spacing: 16.0

            // Empty search icon
            <View> {
                width: 80.0, height: 80.0
                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let c = self.rect_size * 0.5;
                        // Magnifying glass
                        sdf.circle(c.x - 8.0, c.y - 8.0, 22.0);
                        sdf.stroke(#ccc, 4.0);
                        sdf.move_to(c.x + 8.0, c.y + 8.0);
                        sdf.line_to(c.x + 28.0, c.y + 28.0);
                        sdf.stroke(#ccc, 4.0);
                        // X mark in circle
                        sdf.move_to(c.x - 16.0, c.y - 16.0);
                        sdf.line_to(c.x, c.y);
                        sdf.stroke(#ddd, 2.5);
                        sdf.move_to(c.x, c.y - 16.0);
                        sdf.line_to(c.x - 16.0, c.y);
                        sdf.stroke(#ddd, 2.5);
                        return sdf.result;
                    }
                }
            }

            <Label> {
                text: "No restaurants found"
                draw_text: { text_style: { font_size: 18.0 }, color: #666 }
            }
            <Label> {
                text: "Try adjusting your search or location"
                draw_text: { text_style: { font_size: 14.0 }, color: #999 }
            }
        }

        // Business list
        list = <PortalList> {
            width: Fill, height: Fill
            drag_scrolling: true

            // Pull-to-refresh trigger (first item)
            pull_refresh = <View> {
                width: Fill, height: 60.0
                align: { x: 0.5, y: 0.5 }
                cursor: Hand

                <View> {
                    width: Fit, height: Fit
                    flow: Right
                    spacing: 8.0
                    align: { y: 0.5 }

                    // Refresh arrow icon
                    <View> {
                        width: 20.0, height: 20.0
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let c = self.rect_size * 0.5;
                                // Circular arrow
                                sdf.circle(c.x, c.y, 7.0);
                                sdf.stroke(#999, 1.5);
                                // Arrow head
                                sdf.move_to(c.x + 5.0, c.y - 3.0);
                                sdf.line_to(c.x + 7.0, c.y);
                                sdf.line_to(c.x + 4.0, c.y);
                                sdf.stroke(#999, 1.5);
                                return sdf.result;
                            }
                        }
                    }
                    <Label> {
                        text: "Pull to refresh"
                        draw_text: { text_style: { font_size: 13.0 }, color: #999 }
                    }
                }
            }

            business_card = <BusinessCard> {}
        }
    }

    // Map Screen with markers
    MapScreen = {{MapScreen}} {
        width: Fill
        height: Fill
        flow: Overlay
        show_bg: true
        draw_bg: {
            instance opacity: 1.0
            fn pixel(self) -> vec4 {
                return vec4(1.0, 1.0, 1.0, self.opacity);
            }
        }
        animator: {
            fade = {
                default: show
                hide = {
                    from: { all: Forward { duration: 0.3 } }
                    apply: { draw_bg: { opacity: 0.0 } }
                }
                show = {
                    from: { all: Forward { duration: 0.3 } }
                    apply: { draw_bg: { opacity: 1.0 } }
                }
            }
        }

        map = <GeoMapView> {
            width: Fill, height: Fill
            center_lat: 37.7749
            center_lng: -122.4194
            zoom: 13.0
        }

        // Header with back button and search bar
        <View> {
            width: Fill, height: Fit
            flow: Down

            header = <View> {
                width: Fill, height: 56.0
                padding: { left: 8.0, right: 16.0 }
                show_bg: true
                draw_bg: { color: #fff }
                flow: Right
                align: { y: 0.5 }
                spacing: 8.0

                back_button = <RoundedView> {
                    width: 40.0, height: 40.0
                    align: { x: 0.5, y: 0.5 }
                    cursor: Hand
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let r = self.rect_size.y * 0.5;
                            sdf.circle(r, r, r);
                            let base = vec4(0.96, 0.96, 0.96, 1.0);
                            let hover = vec4(0.9, 0.9, 0.9, 1.0);
                            sdf.fill(mix(base, hover, self.hover));
                            return sdf.result;
                        }
                    }
                    // Back arrow using Label
                    <Label> {
                        text: "<"
                        draw_text: {
                            color: (YELP_RED)
                            text_style: { font_size: 18.0 }
                        }
                    }
                }

                <Label> {
                    width: Fill, height: Fit
                    text: "Map"
                    draw_text: {
                        text_style: { font_size: 18.0 }
                        color: #1a1a1a
                    }
                }
            }
        }
    }

    // Business Detail Screen
    BusinessDetailScreen = {{BusinessDetailScreen}} {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance opacity: 1.0
            fn pixel(self) -> vec4 {
                return vec4(1.0, 1.0, 1.0, self.opacity);
            }
        }
        animator: {
            fade = {
                default: show
                hide = {
                    from: { all: Forward { duration: 0.3 } }
                    apply: { draw_bg: { opacity: 0.0 } }
                }
                show = {
                    from: { all: Forward { duration: 0.3 } }
                    apply: { draw_bg: { opacity: 1.0 } }
                }
            }
        }

        // Header with back button
        header = <View> {
            width: Fill, height: 56.0
            padding: { left: 8.0, right: 16.0 }
            show_bg: true
            draw_bg: { color: #fff }
            flow: Right
            align: { y: 0.5 }

            back_button = <Button> {
                width: 44.0, height: 44.0
                text: "<"
                draw_text: {
                    color: (YELP_RED)
                    text_style: { font_size: 20.0 }
                }
                draw_bg: { color: #0000 }
            }

            title = <Label> {
                width: Fill, height: Fit
                draw_text: {
                    text_style: { font_size: 18.0 }
                    color: #1a1a1a
                }
                text: "Business Name"
            }
        }

        <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #e0e0e0 } }

        // Scrollable content
        <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16.0

                // Hero image with overlaid info
                hero_container = <View> {
                    width: Fill, height: 350.0
                    flow: Overlay

                    // Zoomed/darkened background layer for blur-like effect
                    hero_bg = <Image> {
                        width: Fill, height: Fill
                        fit: Biggest
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                let color = self.get_color();
                                if color.w < 0.01 {
                                    return vec4(0.15, 0.15, 0.15, 1.0);
                                }
                                // Darken and desaturate for background effect
                                let gray = dot(color.xyz, vec3(0.299, 0.587, 0.114));
                                let desaturated = mix(color.xyz, vec3(gray), 0.4);
                                return vec4(desaturated * 0.5, 1.0);
                            }
                        }
                    }

                    // Centered hero image with proper aspect ratio
                    <View> {
                        width: Fill, height: Fill
                        align: { x: 0.5, y: 0.5 }

                        hero_image = <Image> {
                            width: Fit, height: Fill
                            fit: Smallest
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    let color = self.get_color();
                                    if color.w < 0.01 {
                                        return vec4(0.0, 0.0, 0.0, 0.0);
                                    }
                                    return color;
                                }
                            }
                        }
                    }

                    // Loading spinner for hero image
                    hero_spinner = <LoadingSpinner> {
                        width: 48.0, height: 48.0
                    }

                    // Gradient overlay for text readability (top and bottom)
                    <View> {
                        width: Fill, height: Fill
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                // Gradient at top for title
                                let top_gradient = smoothstep(0.3, 0.0, self.pos.y);
                                // Gradient at bottom for rating/info
                                let bottom_gradient = smoothstep(0.5, 1.0, self.pos.y);
                                let alpha = max(top_gradient, bottom_gradient) * 0.7;
                                return vec4(0.0, 0.0, 0.0, alpha);
                            }
                        }
                    }

                    // Business info overlaid - title at top, rating/info at bottom
                    info_section = <View> {
                        width: Fill, height: Fill
                        flow: Down
                        padding: 16.0

                        // Name at top
                        name_label = <Label> {
                            width: Fill, height: Fit
                            draw_text: { text_style: { font_size: 28.0 }, color: #fff }
                            text: "Business Name"
                        }

                        // Spacer to push info to bottom
                        <View> { width: Fill, height: Fill }

                        // Rating and other info at bottom with dark backdrop
                        <RoundedView> {
                            width: Fit, height: Fit
                            flow: Down
                            spacing: 8.0
                            padding: { top: 8.0, bottom: 8.0, left: 12.0, right: 12.0 }
                            show_bg: true
                            draw_bg: {
                                color: #0008
                                border_radius: 8.0
                            }

                            rating_row = <View> {
                                width: Fit, height: Fit
                                flow: Right
                                spacing: 8.0
                                align: { y: 0.5 }

                                stars = <StarRating> {
                                    width: 100.0, height: 20.0
                                }
                                rating_text = <Label> {
                                    width: Fit, height: Fit
                                    draw_text: { text_style: { font_size: 14.0 }, color: #fff }
                                    text: "4.5"
                                }
                                review_count = <Label> {
                                    width: Fit, height: Fit
                                    draw_text: { text_style: { font_size: 14.0 }, color: #ddd }
                                    text: "(1,234 reviews)"
                                }
                            }

                            meta_label = <Label> {
                                width: Fill, height: Fit
                                draw_text: { text_style: { font_size: 14.0 }, color: #ddd }
                                text: "$$$ · Italian · Pizza"
                            }

                            location_label = <Label> {
                                width: Fill, height: Fit
                                draw_text: { text_style: { font_size: 14.0 }, color: #ccc }
                                text: "San Francisco · 0.5 mi"
                            }
                        }
                    }
                }

                // Action button with pill shape and hover states
                action_buttons = <View> {
                    width: Fill, height: Fit
                    padding: { left: 16.0, right: 16.0 }

                    directions_button = <RoundedView> {
                        width: Fill, height: 50.0
                        align: { x: 0.5, y: 0.5 }
                        cursor: Hand
                        show_bg: true
                        draw_bg: {
                            instance hover: 0.0
                            instance pressed: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let w = self.rect_size.x;
                                let h = self.rect_size.y;
                                let r = h * 0.5;
                                // Yelp red with hover/pressed darkening
                                let base_color = vec4(0.827, 0.137, 0.137, 1.0);
                                let hover_color = vec4(0.70, 0.10, 0.10, 1.0);
                                let pressed_color = vec4(0.55, 0.08, 0.08, 1.0);
                                let color = mix(mix(base_color, hover_color, self.hover), pressed_color, self.pressed);
                                // Pill shape
                                sdf.circle(r, h * 0.5, r);
                                sdf.fill(color);
                                sdf.rect(r, 0.0, w - h, h);
                                sdf.fill(color);
                                sdf.circle(w - r, h * 0.5, r);
                                sdf.fill(color);
                                return sdf.result;
                            }
                        }
                        <Label> {
                            text: "Directions"
                            draw_text: { color: #fff, text_style: { font_size: 16.0 } }
                        }
                    }
                }

                // Description section
                <View> {
                    width: Fill, height: 1.0
                    margin: { left: 16.0, right: 16.0 }
                    show_bg: true, draw_bg: { color: #e0e0e0 }
                }

                description_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8.0
                    padding: { left: 16.0, right: 16.0 }

                    <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 16.0 }, color: #1a1a1a }
                        text: "About"
                    }

                    description = <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 14.0, line_spacing: 1.4 }, color: #666 }
                        text: "A popular spot known for its delicious food and great atmosphere. Come visit us for an unforgettable dining experience."
                    }
                }

                // Hours section
                <View> {
                    width: Fill, height: 1.0
                    margin: { left: 16.0, right: 16.0 }
                    show_bg: true, draw_bg: { color: #e0e0e0 }
                }

                hours_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8.0
                    padding: { left: 16.0, right: 16.0, bottom: 16.0 }

                    <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 16.0 }, color: #1a1a1a }
                        text: "Hours"
                    }

                    hours_label = <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 14.0 }, color: #666 }
                        text: "Mon-Sun: 11:00 AM - 10:00 PM"
                    }
                }
            }
        }
    }

    // Main App with Stack Navigation
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { inner_size: vec2(1080, 2424) }
                show_bg: true
                width: Fill
                height: Fill
                draw_bg: { fn pixel(self) -> vec4 { return #ffffff } }

                body = <View> {
                    width: Fill
                    height: Fill
                    flow: Down

                    // Title bar
                    <View> {
                        width: Fill, height: 50.0
                        show_bg: true
                        draw_bg: { color: #d32323 }
                        align: { x: 0.5, y: 0.5 }

                        <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                text_style: { font_size: 18.0 }
                                color: #ffffff
                            }
                            text: "Makepad Yelp"
                        }
                    }

                    content = <View> {
                        width: Fill
                        height: Fill
                        flow: Overlay

                        search_screen = <SearchScreen> { visible: true }
                        map_screen = <MapScreen> { visible: false }
                        detail_screen = <BusinessDetailScreen> { visible: false }
                    }

                    tab_bar = <YelpTabBar> {}
                }
            }
        }
    }
}

app_main!(App);

// =====================
// Data Types
// =====================

#[derive(Clone, Debug)]
pub struct Business {
    pub id: String,
    pub name: String,
    pub rating: f32,
    pub review_count: u32,
    pub price: Option<String>,
    pub categories: Vec<String>,
    pub city: String,
    pub distance_meters: Option<f64>,
    pub lat: f64,
    pub lng: f64,
}

impl Business {
    pub fn price_and_categories(&self) -> String {
        let mut parts = Vec::new();
        if let Some(ref price) = self.price {
            parts.push(price.clone());
        }
        if !self.categories.is_empty() {
            parts.push(self.categories.join(", "));
        }
        if let Some(distance) = self.distance_meters {
            let miles = distance / 1609.34;
            parts.push(format!("{:.1} mi", miles));
        }
        parts.join(" · ")
    }

    pub fn distance_text(&self) -> String {
        if let Some(distance) = self.distance_meters {
            let miles = distance / 1609.34;
            format!("{} · {:.1} mi", self.city, miles)
        } else {
            self.city.clone()
        }
    }

    pub fn meta_line(&self) -> String {
        let mut parts = Vec::new();
        parts.push(self.city.clone());
        if let Some(ref price) = self.price {
            parts.push(price.clone());
        }
        parts.join(" · ")
    }
}

pub fn mock_businesses() -> Vec<Business> {
    vec![
        Business {
            id: "1".into(), name: "Flour + Water".into(), rating: 4.5, review_count: 4521,
            price: Some("$$$".into()), categories: vec!["Italian".into(), "Pizza".into()],
            city: "San Francisco".into(), distance_meters: Some(850.0),
            lat: 37.7599, lng: -122.4148,
        },
        Business {
            id: "2".into(), name: "Tartine Bakery".into(), rating: 4.0, review_count: 8234,
            price: Some("$$".into()), categories: vec!["Bakeries".into(), "Cafes".into()],
            city: "San Francisco".into(), distance_meters: Some(1200.0),
            lat: 37.7614, lng: -122.4241,
        },
        Business {
            id: "3".into(), name: "Burma Superstar".into(), rating: 4.0, review_count: 6712,
            price: Some("$$".into()), categories: vec!["Burmese".into()],
            city: "San Francisco".into(), distance_meters: Some(3400.0),
            lat: 37.7829, lng: -122.4589,
        },
        Business {
            id: "4".into(), name: "Zuni Cafe".into(), rating: 4.0, review_count: 3891,
            price: Some("$$$".into()), categories: vec!["American".into()],
            city: "San Francisco".into(), distance_meters: Some(2100.0),
            lat: 37.7755, lng: -122.4214,
        },
        Business {
            id: "5".into(), name: "La Taqueria".into(), rating: 4.0, review_count: 5423,
            price: Some("$".into()), categories: vec!["Mexican".into(), "Tacos".into()],
            city: "San Francisco".into(), distance_meters: Some(1800.0),
            lat: 37.7509, lng: -122.4180,
        },
    ]
}

// =====================
// Widget Implementations
// =====================

#[derive(Live, LiveHook, Widget)]
pub struct StarRating {
    #[deref] view: View,
    #[live] rating: f32,
}

impl Widget for StarRating {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.apply_over(cx, live! { draw_bg: { rating: (self.rating) } });
        self.view.draw_walk(cx, scope, walk)
    }
}

impl StarRatingRef {
    pub fn set_rating(&self, cx: &mut Cx, rating: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.rating = rating;
            inner.redraw(cx);
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct LoadingSpinner {
    #[deref] view: View,
    #[rust] start_time: f64,
    #[rust] next_frame: NextFrame,
    #[rust] animating: bool,
}

impl Widget for LoadingSpinner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Request continuous animation frames
        if let Some(nf) = self.next_frame.is_event(event) {
            // Initialize start time on first frame
            if self.start_time == 0.0 {
                self.start_time = nf.time;
            }
            // Update time uniform for rotation
            let elapsed = nf.time - self.start_time;
            let rotation = (elapsed % 1.0) as f32; // 1 second per rotation
            self.view.apply_over(cx, live! { draw_bg: { spin_time: (rotation) } });
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Start animation on first draw
        if !self.animating {
            self.animating = true;
            self.next_frame = cx.new_next_frame();
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SearchBar {
    #[deref] view: View,
}

impl Widget for SearchBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BusinessCard {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[rust] business: Option<Business>,
    #[rust] loaded_image_id: Option<String>, // Track which business's image is loaded
}

impl Widget for BusinessCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Forward to view first so area is set up
        self.view.handle_event(cx, event, scope);

        // Handle animator
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        // Handle hits on our area
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    if let Some(ref business) = self.business {
                        log!("BusinessCard clicked: {}", business.name);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            BusinessCardAction::Clicked(business.clone()),
                        );
                    }
                }
            }
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, &[live_id!(hover), live_id!(on)]);
                cx.set_cursor(MouseCursor::Hand);
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, &[live_id!(hover), live_id!(off)]);
                cx.set_cursor(MouseCursor::Default);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(ref business) = self.business {
            self.view.label(ids!(name_label)).set_text(cx, &business.name);
            self.view.label(ids!(distance_label)).set_text(cx, &format!("{:.1} mi", business.distance_meters.unwrap_or(0.0) / 1609.34));
            self.view.label(ids!(rating_num)).set_text(cx, &format!("{:.1}", business.rating));
            self.view.label(ids!(review_count)).set_text(cx, &format!("({} reviews)", business.review_count));
            self.view.label(ids!(meta_label)).set_text(cx, &business.meta_line());

            // Set first category tag
            if !business.categories.is_empty() {
                self.view.label(ids!(tag1_label)).set_text(cx, &business.categories[0]);
            }

            // Set star rating
            self.view.star_rating(ids!(stars)).set_rating(cx, business.rating);

            // Load restaurant image from network if available
            let needs_load = self.loaded_image_id.as_ref() != Some(&business.id);
            let mut image_loaded = !needs_load;

            if needs_load {
                // Get image data from cache (clone to release lock quickly)
                let image_data = RESTAURANT_IMAGES.lock().ok()
                    .and_then(|images| images.get(&business.id).cloned());

                if let Some(data) = image_data {
                    let photo = self.view.image(ids!(photo));
                    // Try loading as JPEG (picsum returns JPEG)
                    if photo.load_jpg_from_data(cx, &data).is_ok() {
                        self.loaded_image_id = Some(business.id.clone());
                        image_loaded = true;
                    }
                }
            }

            // Show/hide spinner based on image loading state
            // Set size to 0 when hidden to completely remove it from layout and rendering
            let spinner = self.view.view(ids!(photo_spinner));
            if image_loaded {
                spinner.apply_over(cx, live! {
                    width: 0.0, height: 0.0
                    draw_bg: { opacity: 0.0 }
                });
            } else {
                spinner.apply_over(cx, live! {
                    width: 36.0, height: 36.0
                    draw_bg: { opacity: 1.0 }
                });
            }
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl BusinessCard {
    pub fn set_business(&mut self, business: &Business) {
        // Reset loaded_image_id if business changed to force image reload check
        if self.business.as_ref().map(|b| &b.id) != Some(&business.id) {
            self.loaded_image_id = None;
        }
        self.business = Some(business.clone());
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum BusinessCardAction {
    None,
    Clicked(Business),
}

#[derive(Live, LiveHook, Widget)]
pub struct YelpTabBar {
    #[deref] view: View,
    #[live(true)] visible: bool,
    #[rust] current_tab: Tab,
}

impl Widget for YelpTabBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible { return; }
        self.view.handle_event(cx, event, scope);

        // Check hits on search tab area
        let search_area = self.view.view(ids!(search_tab)).area();
        if let Hit::FingerUp(fe) = event.hits(cx, search_area) {
            if fe.is_over {
                log!("Search tab clicked, current: {:?}", self.current_tab);
                if self.current_tab != Tab::Search {
                    self.current_tab = Tab::Search;
                    self.update_tab_colors(cx);
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        YelpTabBarAction::TabChanged(Tab::Search),
                    );
                }
            }
        }

        // Check hits on map tab area
        let map_area = self.view.view(ids!(map_tab)).area();
        if let Hit::FingerUp(fe) = event.hits(cx, map_area) {
            if fe.is_over {
                log!("Map tab clicked, current: {:?}", self.current_tab);
                if self.current_tab != Tab::Map {
                    self.current_tab = Tab::Map;
                    self.update_tab_colors(cx);
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        YelpTabBarAction::TabChanged(Tab::Map),
                    );
                }
            }
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible { return DrawStep::done(); }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl YelpTabBar {
    fn update_tab_colors(&mut self, cx: &mut Cx) {
        let yelp_red = vec4(0.827, 0.137, 0.137, 1.0);
        let white = vec4(1.0, 1.0, 1.0, 1.0);
        let gray = vec4(0.4, 0.4, 0.4, 1.0);
        let transparent = vec4(0.0, 0.0, 0.0, 0.0);

        let (search_bg, search_fg, map_bg, map_fg) = match self.current_tab {
            Tab::Search => (yelp_red, white, transparent, gray),
            Tab::Map => (transparent, gray, yelp_red, white),
        };

        // Update search tab background and colors
        self.view.view(ids!(search_tab)).apply_over(cx, live! {
            draw_bg: { bg_color: (search_bg) }
        });
        self.view.view(ids!(search_icon)).apply_over(cx, live! {
            draw_bg: { icon_color: (search_fg) }
        });
        self.view.label(ids!(search_label)).apply_over(cx, live! {
            draw_text: { color: (search_fg) }
        });

        // Update map tab background and colors
        self.view.view(ids!(map_tab)).apply_over(cx, live! {
            draw_bg: { bg_color: (map_bg) }
        });
        self.view.view(ids!(map_icon)).apply_over(cx, live! {
            draw_bg: { icon_color: (map_fg) }
        });
        self.view.label(ids!(map_label)).apply_over(cx, live! {
            draw_text: { color: (map_fg) }
        });

        self.redraw(cx);
    }
}

impl YelpTabBarRef {
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }

    pub fn set_active_tab(&self, cx: &mut Cx, tab: Tab) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.current_tab = tab;
            inner.update_tab_colors(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum YelpTabBarAction {
    None,
    TabChanged(Tab),
}

#[derive(Live, LiveHook, Widget)]
pub struct SearchScreen {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[live(true)] visible: bool,
    #[live(true)] is_active: bool, // Whether to process events
    #[rust] businesses: Vec<Business>,
    #[rust] is_refreshing: bool,
    #[rust] show_empty_state: bool,
}

impl Widget for SearchScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        if !self.is_active { return; }
        self.view.handle_event(cx, event, scope);

        // Handle pull-to-refresh tap
        let refresh_view = self.view.portal_list(ids!(list)).view(ids!(pull_refresh));
        if let Hit::FingerUp(fe) = event.hits(cx, refresh_view.area()) {
            if fe.is_over && !self.is_refreshing {
                self.is_refreshing = true;
                cx.widget_action(self.widget_uid(), &scope.path, SearchScreenAction::Refresh);
                self.redraw(cx);
            }
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible { return DrawStep::done(); }
        if self.businesses.is_empty() && !self.show_empty_state {
            self.businesses = mock_businesses();
        }

        // Show/hide refresh indicator
        self.view.view(ids!(refresh_indicator)).set_visible(cx, self.is_refreshing);

        // Show/hide empty state vs list
        let has_results = !self.businesses.is_empty() || !self.show_empty_state;
        self.view.view(ids!(empty_state)).set_visible(cx, self.show_empty_state && self.businesses.is_empty());
        self.view.portal_list(ids!(list)).set_visible(cx, has_results);

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // +1 for pull_refresh item at the top
                list.set_item_range(cx, 0, self.businesses.len() + 1);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id == 0 {
                        // Pull to refresh item (hidden when refreshing)
                        let item = list.item(cx, item_id, live_id!(pull_refresh));
                        if !self.is_refreshing {
                            item.draw_all_unscoped(cx);
                        }
                    } else if item_id <= self.businesses.len() {
                        let item = list.item(cx, item_id, live_id!(business_card));
                        if let Some(mut card) = item.borrow_mut::<BusinessCard>() {
                            card.set_business(&self.businesses[item_id - 1]);
                        }
                        item.draw_all_unscoped(cx);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl SearchScreenRef {
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_active = visible;
            if visible {
                inner.visible = true;
                // Start with opacity 0 and animate to 1
                inner.view.apply_over(cx, live! { draw_bg: { opacity: 0.0 } });
                inner.animator_play(cx, &[live_id!(fade), live_id!(show)]);
            } else {
                inner.visible = false;
            }
            inner.redraw(cx);
        }
    }

    pub fn set_refreshing(&self, cx: &mut Cx, refreshing: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_refreshing = refreshing;
            inner.redraw(cx);
        }
    }

    pub fn set_businesses(&self, cx: &mut Cx, businesses: Vec<Business>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.businesses = businesses;
            inner.show_empty_state = true; // Enable empty state checking
            inner.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum SearchScreenAction {
    None,
    Refresh,
}

#[derive(Live, LiveHook, Widget)]
pub struct MapScreen {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[live] visible: bool,
    #[rust] is_active: bool,
    #[rust] markers_added: bool,
}

impl Widget for MapScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        if !self.is_active { return; }
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        // Handle back button
        let back_btn = self.view.view(ids!(back_button));
        let back_area = back_btn.area();
        match event.hits(cx, back_area) {
            Hit::FingerDown(_) => {
                back_btn.apply_over(cx, live!{ draw_bg: { hover: 1.0 } });
                back_btn.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                back_btn.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
                back_btn.redraw(cx);
                if fe.is_over {
                    cx.widget_action(self.widget_uid(), &scope.path, MapScreenAction::Back);
                }
            }
            Hit::FingerHoverIn(_) => {
                back_btn.apply_over(cx, live!{ draw_bg: { hover: 1.0 } });
                back_btn.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                back_btn.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
                back_btn.redraw(cx);
            }
            _ => {}
        }

        // Handle marker taps
        let map = self.view.geo_map_view(ids!(map));
        if let Some(marker_id) = map.marker_tapped(&actions) {
            log!("Marker tapped: {:?}", marker_id);
            // Find the business and emit action
            let businesses = mock_businesses();
            for business in businesses {
                let business_live_id = LiveId::from_str(&business.id);
                if marker_id == business_live_id {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        BusinessCardAction::Clicked(business),
                    );
                    break;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible { return DrawStep::done(); }
        // Add markers on first draw
        if !self.markers_added {
            self.markers_added = true;
            let businesses = mock_businesses();
            let map = self.view.geo_map_view(ids!(map));

            for business in &businesses {
                let marker_id = LiveId::from_str(&business.id);
                map.add_marker_with_label(
                    cx,
                    marker_id,
                    business.lng,
                    business.lat,
                    &business.name,
                    vec4(0.827, 0.137, 0.137, 1.0), // YELP_RED
                );
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl MapScreenRef {
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_active = visible;
            if visible {
                inner.visible = true;
                inner.view.apply_over(cx, live! { draw_bg: { opacity: 0.0 } });
                inner.animator_play(cx, &[live_id!(fade), live_id!(show)]);
            } else {
                inner.visible = false;
            }
            inner.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MapScreenAction {
    None,
    Back,
}

#[derive(Live, LiveHook, Widget)]
pub struct BusinessDetailScreen {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[live] visible: bool,
    #[rust] is_active: bool,
    #[rust] business: Option<Business>,
    #[rust] image_loaded: bool,
}

impl Widget for BusinessDetailScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        if !self.is_active { return; }
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        if self.view.button(ids!(back_button)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                DetailScreenAction::Back,
            );
        }

        // Handle directions button
        let dir_btn = self.view.view(ids!(directions_button));
        let dir_area = dir_btn.area();
        match event.hits(cx, dir_area) {
            Hit::FingerDown(_) => {
                dir_btn.apply_over(cx, live!{ draw_bg: { pressed: 1.0 } });
                dir_btn.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                // Keep pressed state visible briefly before action
                if fe.is_over {
                    cx.widget_action(self.widget_uid(), &scope.path, DetailScreenAction::Directions);
                } else {
                    dir_btn.apply_over(cx, live!{ draw_bg: { pressed: 0.0 } });
                    dir_btn.redraw(cx);
                }
            }
            Hit::FingerHoverIn(_) => {
                dir_btn.apply_over(cx, live!{ draw_bg: { hover: 1.0 } });
                dir_btn.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                dir_btn.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
                dir_btn.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible { return DrawStep::done(); }
        if let Some(ref business) = self.business {
            // Header title
            self.view.label(ids!(title)).set_text(cx, &business.name);

            // Info section
            self.view.label(ids!(name_label)).set_text(cx, &business.name);
            self.view.label(ids!(rating_text)).set_text(cx, &format!("{:.1}", business.rating));
            self.view.label(ids!(review_count)).set_text(cx, &format!("({} reviews)", business.review_count));
            self.view.label(ids!(meta_label)).set_text(cx, &business.price_and_categories());
            self.view.label(ids!(location_label)).set_text(cx, &business.distance_text());

            // Set star rating
            self.view.star_rating(ids!(stars)).set_rating(cx, business.rating);

            // Load hero image from network if available
            if !self.image_loaded {
                if let Ok(images) = RESTAURANT_IMAGES.lock() {
                    if let Some(image_data) = images.get(&business.id) {
                        let hero = self.view.image(ids!(hero_image));
                        let hero_bg = self.view.image(ids!(hero_bg));
                        if hero.load_jpg_from_data(cx, image_data).is_ok()
                            || hero.load_png_from_data(cx, image_data).is_ok() {
                            // Also load into background for blur effect
                            let _ = hero_bg.load_jpg_from_data(cx, image_data)
                                .or_else(|_| hero_bg.load_png_from_data(cx, image_data));
                            self.image_loaded = true;
                        }
                    }
                }
            }

            // Show/hide spinner based on image loading state
            // Set size to 0 when hidden to completely remove it
            let spinner = self.view.view(ids!(hero_spinner));
            if self.image_loaded {
                spinner.apply_over(cx, live! {
                    width: 0.0, height: 0.0
                    draw_bg: { opacity: 0.0 }
                });
            } else {
                spinner.apply_over(cx, live! {
                    width: 48.0, height: 48.0
                    draw_bg: { opacity: 1.0 }
                });
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl BusinessDetailScreen {
    pub fn set_business(&mut self, business: &Business) {
        self.business = Some(business.clone());
        self.image_loaded = false; // Reset so image loads for new business
    }
}

impl BusinessDetailScreenRef {
    pub fn set_business(&self, cx: &mut Cx, business: &Business) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_business(business);
            inner.redraw(cx);
        }
    }

    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_active = visible;
            if visible {
                inner.visible = true;
                inner.view.apply_over(cx, live! { draw_bg: { opacity: 0.0 } });
                inner.animator_play(cx, &[live_id!(fade), live_id!(show)]);
                // Reset back button visual state
                let back_btn = inner.view.button(ids!(back_button));
                back_btn.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
            } else {
                inner.visible = false;
            }
            inner.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DetailScreenAction {
    None,
    Back,
    Call,
    Directions,
}

// =====================
// App
// =====================

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] current_tab: Tab,
    #[rust] showing_detail: bool,
    #[rust] images_pending: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Tab {
    #[default]
    Search,
    Map,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        makepad_map::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.request_images(cx);
    }

    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        let businesses = mock_businesses();
        for event in responses {
            // Check if this response is for one of our business images
            for business in &businesses {
                if event.request_id == LiveId::from_str(&business.id) {
                    match &event.response {
                        NetworkResponse::HttpResponse(response) => {
                            if let Some(body) = &response.body {
                                log!("Received image for {}: {} bytes", business.name, body.len());
                                // Store in global map by business ID
                                if let Ok(mut images) = RESTAURANT_IMAGES.lock() {
                                    images.insert(business.id.clone(), body.clone());
                                }
                                // Track pending images
                                if self.images_pending > 0 {
                                    self.images_pending -= 1;
                                    if self.images_pending == 0 {
                                        // All images loaded, stop refreshing
                                        self.ui.search_screen(ids!(search_screen)).set_refreshing(cx, false);
                                    }
                                }
                                // Redraw to trigger image loading in cards
                                self.ui.redraw(cx);
                            }
                        }
                        NetworkResponse::HttpRequestError(err) => {
                            log!("Image request error for {}: {:?}", business.name, err);
                            // Still decrement pending on error
                            if self.images_pending > 0 {
                                self.images_pending -= 1;
                                if self.images_pending == 0 {
                                    self.ui.search_screen(ids!(search_screen)).set_refreshing(cx, false);
                                }
                            }
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions.iter() {
            // Handle tab changes (widget action pattern)
            if let YelpTabBarAction::TabChanged(tab) = action.as_widget_action().cast() {
                log!("App received TabChanged: {:?}", tab);
                self.switch_tab(cx, &tab);
                continue;
            }

            // Handle business card clicks (widget action pattern)
            if let BusinessCardAction::Clicked(business) = action.as_widget_action().cast() {
                log!("App received BusinessCardClicked: {}", business.name);
                self.show_detail(cx, &business);
                continue;
            }

            // Handle back from detail (widget action pattern)
            if let DetailScreenAction::Back = action.as_widget_action().cast() {
                log!("App received Back action");
                self.hide_detail(cx);
                continue;
            }

            // Handle directions button - switch to map
            if let DetailScreenAction::Directions = action.as_widget_action().cast() {
                log!("App received Directions action");
                self.hide_detail(cx);
                self.switch_tab(cx, &Tab::Map);
                continue;
            }

            // Handle call button (placeholder - just log for now)
            if let DetailScreenAction::Call = action.as_widget_action().cast() {
                log!("App received Call action - would open phone dialer");
                continue;
            }

            // Handle back from map screen - switch to search
            if let MapScreenAction::Back = action.as_widget_action().cast() {
                log!("App received Map Back action");
                self.switch_tab(cx, &Tab::Search);
                continue;
            }

            // Handle search screen refresh
            if let SearchScreenAction::Refresh = action.as_widget_action().cast() {
                log!("App received Refresh action");
                self.refresh_images(cx);
                continue;
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Capture actions generated by UI event handling
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        // Handle the captured actions
        self.handle_actions(cx, &actions);

        // Also handle system events like Startup
        self.match_event(cx, event);
    }
}

impl App {
    fn switch_tab(&mut self, cx: &mut Cx, tab: &Tab) {
        if self.showing_detail {
            self.hide_detail(cx);
        }
        if self.current_tab == *tab { return; }
        self.current_tab = *tab;
        self.ui.search_screen(ids!(search_screen)).set_visible(cx, *tab == Tab::Search);
        self.ui.map_screen(ids!(map_screen)).set_visible(cx, *tab == Tab::Map);
        // Update tab bar highlighting
        self.ui.yelp_tab_bar(ids!(tab_bar)).set_active_tab(cx, *tab);
        self.ui.redraw(cx);
    }

    fn show_detail(&mut self, cx: &mut Cx, business: &Business) {
        self.showing_detail = true;

        // Set business data on detail screen
        self.ui.business_detail_screen(ids!(detail_screen)).set_business(cx, business);

        // Show detail screen, hide others
        self.ui.search_screen(ids!(search_screen)).set_visible(cx, false);
        self.ui.map_screen(ids!(map_screen)).set_visible(cx, false);
        self.ui.business_detail_screen(ids!(detail_screen)).set_visible(cx, true);
        self.ui.yelp_tab_bar(ids!(tab_bar)).set_visible(cx, false);
        self.ui.redraw(cx);
    }

    fn hide_detail(&mut self, cx: &mut Cx) {
        self.showing_detail = false;

        // Restore previous tab
        self.ui.business_detail_screen(ids!(detail_screen)).set_visible(cx, false);
        self.ui.search_screen(ids!(search_screen)).set_visible(cx, self.current_tab == Tab::Search);
        self.ui.map_screen(ids!(map_screen)).set_visible(cx, self.current_tab == Tab::Map);
        self.ui.yelp_tab_bar(ids!(tab_bar)).set_visible(cx, true);
        self.ui.redraw(cx);
    }

    fn request_images(&mut self, cx: &mut Cx) {
        let businesses = mock_businesses();
        self.images_pending = businesses.len();
        for business in &businesses {
            let url = format!("https://picsum.photos/seed/{}/320/240", business.id);
            let request = HttpRequest::new(url, HttpMethod::GET);
            cx.http_request(LiveId::from_str(&business.id), request);
        }
        log!("Requesting {} restaurant images...", businesses.len());
    }

    fn refresh_images(&mut self, cx: &mut Cx) {
        // Clear cached images and re-request
        if let Ok(mut images) = RESTAURANT_IMAGES.lock() {
            images.clear();
        }
        self.request_images(cx);
    }
}
