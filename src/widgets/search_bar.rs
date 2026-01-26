use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323

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
                sdf.box(
                    1.0,
                    1.0,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                );
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }

        flow: Right
        align: { y: 0.5 }
        padding: { left: 12.0, right: 12.0 }
        spacing: 8.0

        // Search icon placeholder
        <Label> {
            width: Fit
            height: Fit
            text: "Search"
            draw_text: {
                text_style: { font_size: 14.0 }
                color: #999
            }
        }

        input = <TextInput> {
            width: Fill
            height: Fit
            empty_message: "Restaurants, bars, cafes..."
            draw_bg: { color: #0000 }
            draw_text: {
                text_style: { font_size: 14.0 }
                color: #333
            }
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
