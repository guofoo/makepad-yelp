use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323

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

    // Business Card widget
    BusinessCard = {{BusinessCard}} {
        width: Fill
        height: Fit
        padding: 16.0
        flow: Right
        spacing: 12.0
        show_bg: true
        draw_bg: { color: #fff }

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
            rating_label = <Label> {
                width: Fit, height: Fit
                draw_text: { text_style: { font_size: 13.0 }, color: #666 }
                text: "4.5 (100)"
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

    // Map Screen
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

    // Main App
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { inner_size: vec2(390, 844) }
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
                        search_screen = <SearchScreen> { visible: true }
                        map_screen = <MapScreen> { visible: false }
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
}

pub fn mock_businesses() -> Vec<Business> {
    vec![
        Business {
            id: "1".into(), name: "Flour + Water".into(), rating: 4.5, review_count: 4521,
            price: Some("$$$".into()), categories: vec!["Italian".into(), "Pizza".into()],
            city: "San Francisco".into(), distance_meters: Some(850.0),
        },
        Business {
            id: "2".into(), name: "Tartine Bakery".into(), rating: 4.0, review_count: 8234,
            price: Some("$$".into()), categories: vec!["Bakeries".into(), "Cafes".into()],
            city: "San Francisco".into(), distance_meters: Some(1200.0),
        },
        Business {
            id: "3".into(), name: "Burma Superstar".into(), rating: 4.0, review_count: 6712,
            price: Some("$$".into()), categories: vec!["Burmese".into()],
            city: "San Francisco".into(), distance_meters: Some(3400.0),
        },
        Business {
            id: "4".into(), name: "Zuni Cafe".into(), rating: 4.0, review_count: 3891,
            price: Some("$$$".into()), categories: vec!["American".into()],
            city: "San Francisco".into(), distance_meters: Some(2100.0),
        },
        Business {
            id: "5".into(), name: "La Taqueria".into(), rating: 4.0, review_count: 5423,
            price: Some("$".into()), categories: vec!["Mexican".into(), "Tacos".into()],
            city: "San Francisco".into(), distance_meters: Some(1800.0),
        },
    ]
}

// =====================
// Widget Implementations
// =====================

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
    #[rust] business_id: String,
    #[rust] business_name: String,
    #[rust] rating_text: String,
    #[rust] meta_text: String,
    #[rust] location_text: String,
}

impl Widget for BusinessCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) if fe.is_over => {
                log!("Clicked business: {}", self.business_id);
            }
            Hit::FingerHoverIn(_) => cx.set_cursor(MouseCursor::Hand),
            Hit::FingerHoverOut(_) => cx.set_cursor(MouseCursor::Default),
            _ => {}
        }
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.label(ids!(name_label)).set_text(cx, &self.business_name);
        self.view.label(ids!(rating_label)).set_text(cx, &self.rating_text);
        self.view.label(ids!(meta_label)).set_text(cx, &self.meta_text);
        self.view.label(ids!(location_label)).set_text(cx, &self.location_text);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl BusinessCard {
    pub fn set_business(&mut self, business: &Business) {
        self.business_id = business.id.clone();
        self.business_name = business.name.clone();
        self.rating_text = format!("{:.1} ({})", business.rating, business.review_count);
        self.meta_text = business.price_and_categories();
        self.location_text = business.city.clone();
    }
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
}

impl Widget for MapScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// =====================
// App
// =====================

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] current_tab: Tab,
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
            if let Some(YelpTabBarAction::TabChanged(tab)) = action.downcast_ref() {
                self.switch_tab(cx, *tab);
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
        if self.current_tab == tab { return; }
        self.current_tab = tab;
        self.ui.view(ids!(search_screen)).set_visible(cx, tab == Tab::Search);
        self.ui.view(ids!(map_screen)).set_visible(cx, tab == Tab::Map);
        self.ui.redraw(cx);
    }
}
