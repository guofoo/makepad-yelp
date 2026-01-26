use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    MapScreen = {{MapScreen}} {
        width: Fill
        height: Fill
        flow: Overlay

        // Map view
        map = <GeoMapView> {
            width: Fill
            height: Fill
            latitude: 37.7749
            longitude: -122.4194
            zoom: 13.0
        }

        // Floating search bar
        <View> {
            width: Fill
            height: Fit
            padding: { top: 12.0, left: 16.0, right: 16.0 }

            search_bar = <SearchBar> {}
        }
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
