use makepad_widgets::*;
use crate::data::{Business, mock_businesses};
use crate::widgets::business_card::BusinessCard;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    SearchScreen = {{SearchScreen}} {
        width: Fill
        height: Fill
        flow: Down

        // Search header
        header = <View> {
            width: Fill
            height: Fit
            padding: { top: 12.0, bottom: 12.0, left: 16.0, right: 16.0 }
            show_bg: true
            draw_bg: { color: #fff }

            search_bar = <SearchBar> {}
        }

        // Divider
        <View> {
            width: Fill
            height: 1.0
            show_bg: true
            draw_bg: { color: #e0e0e0 }
        }

        // Business list
        list = <PortalList> {
            width: Fill
            height: Fill

            business_card = <BusinessCard> {}
        }
    }
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
        // Load mock data on first draw
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
