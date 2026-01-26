use makepad_widgets::*;
use crate::app::Tab;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323
    GRAY = #999

    YelpTabBar = {{YelpTabBar}} {
        width: Fill
        height: 60.0

        show_bg: true
        draw_bg: {
            color: #fff
            instance border_color: #e0e0e0

            fn pixel(self) -> vec4 {
                if self.pos.y < 1.0 / self.rect_size.y {
                    return self.border_color;
                }
                return self.color;
            }
        }

        flow: Right
        padding: { bottom: 8.0 }

        search_tab = <Button> {
            width: Fill
            height: Fill
            text: "Search"
            draw_text: {
                color: (YELP_RED)
                text_style: { font_size: 12.0 }
            }
            draw_bg: { color: #0000 }
        }

        map_tab = <Button> {
            width: Fill
            height: Fill
            text: "Map"
            draw_text: {
                color: (GRAY)
                text_style: { font_size: 12.0 }
            }
            draw_bg: { color: #0000 }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct YelpTabBar {
    #[deref] view: View,
    #[rust] current_tab: Tab,
}

impl Widget for YelpTabBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(ids!(search_tab)).clicked(&actions) {
            if self.current_tab != Tab::Search {
                self.current_tab = Tab::Search;
                cx.action(YelpTabBarAction::TabChanged(Tab::Search));
            }
        }
        if self.view.button(ids!(map_tab)).clicked(&actions) {
            if self.current_tab != Tab::Map {
                self.current_tab = Tab::Map;
                cx.action(YelpTabBarAction::TabChanged(Tab::Map));
            }
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
