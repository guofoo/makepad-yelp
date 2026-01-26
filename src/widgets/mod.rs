pub mod business_card;
pub mod search_bar;
pub mod tab_bar;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    business_card::live_design(cx);
    search_bar::live_design(cx);
    tab_bar::live_design(cx);
}
