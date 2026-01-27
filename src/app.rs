use makepad_widgets::*;
use makepad_map::GeoMapViewWidgetExt;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323
    STAR_YELLOW = #f8b84e

    // Star Rating widget - proper 5-pointed stars
    StarRating = {{StarRating}} {
        width: Fit
        height: 20.0
        show_bg: true
        draw_bg: {
            instance rating: 0.0

            fn pixel(self) -> vec4 {
                let filled = vec4(0.957, 0.224, 0.224, 1.0);  // #f43939
                let empty = vec4(0.878, 0.878, 0.878, 1.0);   // #e0e0e0

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
                let outer_r = 0.45;
                let inner_r = 0.18;

                // Calculate which segment we're in
                let segment_angle = pi / points;
                let a = mod(angle + pi + segment_angle / 2.0, 2.0 * segment_angle) - segment_angle;

                // Interpolate between outer and inner radius
                let t = abs(a) / segment_angle;
                let star_radius = mix(outer_r, inner_r, t);

                let inside = step(r, star_radius);
                let is_filled = step(star_idx + 0.5, self.rating);
                let col = mix(empty, filled, is_filled);

                return vec4(col.rgb, col.a * inside);
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

        // Photo placeholder (network images require custom implementation)
        photo = <RoundedView> {
            width: 110.0, height: 110.0
            show_bg: true
            draw_bg: {
                color: #e8e0d8
                instance radius: 8.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
                    sdf.fill(self.color);
                    // Food icon silhouette
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y - 5.0, 20.0);
                    sdf.fill(#d0c8c0);
                    sdf.circle(c.x - 15.0, c.y + 15.0, 8.0);
                    sdf.fill(#d0c8c0);
                    sdf.circle(c.x + 15.0, c.y + 15.0, 8.0);
                    sdf.fill(#d0c8c0);
                    return sdf.result;
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
                    padding: { top: 6.0, bottom: 6.0, left: 12.0, right: 12.0 }
                    show_bg: true
                    draw_bg: {
                        color: #fff
                        instance border_color: #e0e0e0
                        instance radius: 16.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.radius);
                            sdf.fill_keep(self.color);
                            sdf.stroke(self.border_color, 1.0);
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
                instance radius: 22.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
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
                instance radius: 22.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
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
                        // Location pin head (circle with hole)
                        sdf.circle(10.0, 6.0, 4.5);
                        sdf.fill_keep(self.icon_color);
                        sdf.circle(10.0, 6.0, 1.8);
                        sdf.subtract();
                        // Pin point (triangle)
                        sdf.move_to(5.5, 8.0);
                        sdf.line_to(10.0, 18.0);
                        sdf.line_to(14.5, 8.0);
                        sdf.close_path();
                        sdf.fill(self.icon_color);
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
        draw_bg: { color: #f5f5f5 }

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

        // Business list
        list = <PortalList> {
            width: Fill, height: Fill
            business_card = <BusinessCard> {}
        }
    }

    // Map Screen with markers
    MapScreen = {{MapScreen}} {
        width: Fill
        height: Fill
        flow: Overlay

        map = <GeoMapView> {
            width: Fill, height: Fill
            center_lat: 37.7749
            center_lng: -122.4194
            zoom: 13.0
        }

        <View> {
            width: Fill, height: Fit
            padding: { top: 12.0, left: 16.0, right: 16.0 }
            <SearchBar> {}
        }
    }

    // Business Detail Screen
    BusinessDetailScreen = {{BusinessDetailScreen}} {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true
        draw_bg: { color: #fff }

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
                padding: 16.0
                spacing: 16.0

                // Hero image placeholder
                hero_image = <RoundedView> {
                    width: Fill, height: 200.0
                    show_bg: true
                    draw_bg: { color: #e0e0e0, border_radius: 8.0 }
                }

                // Business info section
                info_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8.0

                    name_label = <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 24.0 }, color: #1a1a1a }
                        text: "Business Name"
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
                            draw_text: { text_style: { font_size: 14.0 }, color: #666 }
                            text: "4.5"
                        }
                        review_count = <Label> {
                            width: Fit, height: Fit
                            draw_text: { text_style: { font_size: 14.0 }, color: #666 }
                            text: "(1,234 reviews)"
                        }
                    }

                    meta_label = <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 14.0 }, color: #666 }
                        text: "$$$ · Italian · Pizza"
                    }

                    location_label = <Label> {
                        width: Fill, height: Fit
                        draw_text: { text_style: { font_size: 14.0 }, color: #999 }
                        text: "San Francisco · 0.5 mi"
                    }
                }

                // Action buttons
                action_buttons = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 12.0

                    call_button = <Button> {
                        width: Fill, height: 44.0
                        text: "Call"
                        draw_bg: {
                            color: (YELP_RED)
                            border_radius: 6.0
                        }
                        draw_text: {
                            color: #fff
                            text_style: { font_size: 14.0 }
                        }
                    }

                    directions_button = <Button> {
                        width: Fill, height: 44.0
                        text: "Directions"
                        draw_bg: {
                            color: #f5f5f5
                            border_radius: 6.0
                        }
                        draw_text: {
                            color: #333
                            text_style: { font_size: 14.0 }
                        }
                    }
                }

                // Description section
                <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #e0e0e0 } }

                description_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8.0

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
                <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #e0e0e0 } }

                hours_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8.0

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
                window: { inner_size: vec2(1280, 800) }
                show_bg: true
                width: Fill
                height: Fill
                draw_bg: { fn pixel(self) -> vec4 { return #ffffff } }

                body = <View> {
                    width: Fill
                    height: Fill
                    flow: Down

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
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl BusinessCard {
    pub fn set_business(&mut self, business: &Business) {
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
}

#[derive(Clone, Debug, DefaultNone)]
pub enum YelpTabBarAction {
    None,
    TabChanged(Tab),
}

#[derive(Live, LiveHook, Widget)]
pub struct SearchScreen {
    #[deref] view: View,
    #[live(true)] visible: bool,
    #[rust] businesses: Vec<Business>,
}

impl Widget for SearchScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible { return; }
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible { return DrawStep::done(); }
        if self.businesses.is_empty() {
            self.businesses = mock_businesses();
        }
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.businesses.len());
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < self.businesses.len() {
                        let item = list.item(cx, item_id, live_id!(business_card));
                        if let Some(mut card) = item.borrow_mut::<BusinessCard>() {
                            card.set_business(&self.businesses[item_id]);
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
            inner.visible = visible;
            inner.redraw(cx);
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MapScreen {
    #[deref] view: View,
    #[live] visible: bool,
    #[rust] markers_added: bool,
}

impl Widget for MapScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible { return; }
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

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
            inner.visible = visible;
            inner.redraw(cx);
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BusinessDetailScreen {
    #[deref] view: View,
    #[live] visible: bool,
    #[rust] business: Option<Business>,
}

impl Widget for BusinessDetailScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible { return; }
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        if self.view.button(ids!(back_button)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                DetailScreenAction::Back,
            );
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
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl BusinessDetailScreen {
    pub fn set_business(&mut self, business: &Business) {
        self.business = Some(business.clone());
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
            inner.visible = visible;
            inner.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DetailScreenAction {
    None,
    Back,
}

// =====================
// App
// =====================

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] current_tab: Tab,
    #[rust] showing_detail: bool,
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
}
