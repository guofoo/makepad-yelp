pub mod search;
pub mod map;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    search::live_design(cx);
    map::live_design(cx);
}
