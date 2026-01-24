use makepad_widgets::*;
use makepad_map::GeoMapViewWidgetExt;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323
    STAR_YELLOW = #f8b84e

    // Star Rating widget - draws filled/empty stars based on rating
    StarRating = {{StarRating}} {
        width: Fit
        height: 18.0
        show_bg: true
        draw_bg: {
            instance rating: 0.0
            instance star_count: 5.0

            fn pixel(self) -> vec4 {
                let star_width = self.rect_size.y;
                let total_width = star_width * self.star_count;

                // Which star are we in?
                let star_index = floor(self.pos.x * self.star_count);
                let local_x = fract(self.pos.x * self.star_count);

                // Star shape using distance field
                let center = vec2(0.5, 0.5);
                let p = vec2(local_x, self.pos.y) - center;

                // 5-pointed star SDF
                let angle = atan(p.y, p.x);
                let r = length(p);
                let n = 5.0;
                let m = 0.5; // Inner radius ratio
                let star_angle = 3.14159 / n;
                let a = mod(angle + star_angle, 2.0 * star_angle) - star_angle;
                let outer = 0.35;
                let inner = outer * m;
                let star_r = inner / cos(a);

                let in_star = r < star_r + 0.05;

                if !in_star {
                    return vec4(0.0, 0.0, 0.0, 0.0);
                }

                // Filled or empty based on rating
                let fill_threshold = self.rating - star_index;
                if fill_threshold >= 1.0 {
                    // Fully filled star
                    return #f8b84e;
                } else if fill_threshold > 0.0 {
                    // Partially filled - use horizontal position
                    if local_x < fill_threshold {
                        return #f8b84e;
                    } else {
                        return #e0e0e0;
                    }
                } else {
                    // Empty star
                    return #e0e0e0;
                }
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

    // Business Card widget with hover animation
    BusinessCard = {{BusinessCard}} {
        width: Fill
        height: Fit
        padding: 16.0
        flow: Right
        spacing: 12.0
        show_bg: true
        draw_bg: {
            color: #fff
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                return mix(self.color, #f5f5f5, self.hover * 0.5);
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

        photo = <RoundedView> {
            width: 80.0, height: 80.0
            show_bg: true
            draw_bg: { color: #e0e0e0, border_radius: 4.0 }
        }

        info = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4.0

            name_label = <Label> {
                width: Fill, height: Fit
                draw_text: { text_style: { font_size: 16.0 }, color: #1a1a1a }
                text: "Business Name"
            }

            rating_row = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 6.0
                align: { y: 0.5 }

                stars = <StarRating> {
                    width: 90.0, height: 18.0
                }
                review_count = <Label> {
                    width: Fit, height: Fit
                    draw_text: { text_style: { font_size: 13.0 }, color: #666 }
                    text: "(100)"
                }
            }

            meta_label = <Label> {
                width: Fill, height: Fit
                draw_text: { text_style: { font_size: 13.0 }, color: #666 }
                text: "$$$ - Italian - 0.5 mi"
            }
            location_label = <Label> {
                width: Fill, height: Fit
                draw_text: { text_style: { font_size: 13.0 }, color: #999 }
                text: "San Francisco"
            }
        }
    }

    // Tab Bar widget
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
        padding: { bottom: 8.0 }

        search_tab = <Button> {
            width: Fill, height: Fill
            text: "Search"
            draw_text: { color: (YELP_RED), text_style: { font_size: 12.0 } }
            draw_bg: { color: #0000 }
        }
        map_tab = <Button> {
            width: Fill, height: Fill
            text: "Map"
            draw_text: { color: #999, text_style: { font_size: 12.0 } }
            draw_bg: { color: #0000 }
        }
    }

    // Search Screen
    SearchScreen = {{SearchScreen}} {
        width: Fill
        height: Fill
        flow: Down

        header = <View> {
            width: Fill, height: Fit
            padding: { top: 12.0, bottom: 12.0, left: 16.0, right: 16.0 }
            show_bg: true
            draw_bg: { color: #fff }
            <SearchBar> {}
        }

        <View> { width: Fill, height: 1.0, show_bg: true, draw_bg: { color: #e0e0e0 } }

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
                            border_radius: 4.0
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
                            border_radius: 4.0
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
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(_) => {
                if let Some(ref business) = self.business {
                    cx.action(BusinessCardAction::Clicked(business.clone()));
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
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(ref business) = self.business {
            self.view.label(ids!(name_label)).set_text(cx, &business.name);
            self.view.label(ids!(review_count)).set_text(cx, &format!("({})", business.review_count));
            self.view.label(ids!(meta_label)).set_text(cx, &business.price_and_categories());
            self.view.label(ids!(location_label)).set_text(cx, &business.city);

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
    #[rust] current_tab: Tab,
}

impl Widget for YelpTabBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        if self.view.button(ids!(search_tab)).clicked(&actions) && self.current_tab != Tab::Search {
            self.current_tab = Tab::Search;
            cx.action(YelpTabBarAction::TabChanged(Tab::Search));
        }
        if self.view.button(ids!(map_tab)).clicked(&actions) && self.current_tab != Tab::Map {
            self.current_tab = Tab::Map;
            cx.action(YelpTabBarAction::TabChanged(Tab::Map));
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
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
    #[rust] businesses: Vec<Business>,
}

impl Widget for SearchScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
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

#[derive(Live, LiveHook, Widget)]
pub struct MapScreen {
    #[deref] view: View,
    #[rust] markers_added: bool,
}

impl Widget for MapScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
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
                    cx.action(BusinessCardAction::Clicked(business));
                    break;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
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

#[derive(Live, LiveHook, Widget)]
pub struct BusinessDetailScreen {
    #[deref] view: View,
    #[rust] business: Option<Business>,
}

impl Widget for BusinessDetailScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        if self.view.button(ids!(back_button)).clicked(&actions) {
            cx.action(DetailScreenAction::Back);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
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
            // Handle tab changes
            if let Some(YelpTabBarAction::TabChanged(tab)) = action.downcast_ref() {
                self.switch_tab(cx, *tab);
            }

            // Handle business card clicks
            if let Some(BusinessCardAction::Clicked(business)) = action.downcast_ref() {
                self.show_detail(cx, business);
            }

            // Handle back from detail
            if let Some(DetailScreenAction::Back) = action.downcast_ref() {
                self.hide_detail(cx);
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn switch_tab(&mut self, cx: &mut Cx, tab: Tab) {
        if self.showing_detail {
            self.hide_detail(cx);
        }
        if self.current_tab == tab { return; }
        self.current_tab = tab;
        self.ui.view(ids!(search_screen)).set_visible(cx, tab == Tab::Search);
        self.ui.view(ids!(map_screen)).set_visible(cx, tab == Tab::Map);
        self.ui.redraw(cx);
    }

    fn show_detail(&mut self, cx: &mut Cx, business: &Business) {
        self.showing_detail = true;

        // Set business data on detail screen
        self.ui.business_detail_screen(ids!(detail_screen)).set_business(cx, business);

        // Show detail screen, hide others
        self.ui.view(ids!(search_screen)).set_visible(cx, false);
        self.ui.view(ids!(map_screen)).set_visible(cx, false);
        self.ui.view(ids!(detail_screen)).set_visible(cx, true);
        self.ui.view(ids!(tab_bar)).set_visible(cx, false);
        self.ui.redraw(cx);
    }

    fn hide_detail(&mut self, cx: &mut Cx) {
        self.showing_detail = false;

        // Restore previous tab
        self.ui.view(ids!(detail_screen)).set_visible(cx, false);
        self.ui.view(ids!(search_screen)).set_visible(cx, self.current_tab == Tab::Search);
        self.ui.view(ids!(map_screen)).set_visible(cx, self.current_tab == Tab::Map);
        self.ui.view(ids!(tab_bar)).set_visible(cx, true);
        self.ui.redraw(cx);
    }
}
