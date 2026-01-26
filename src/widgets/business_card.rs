use makepad_widgets::*;
use crate::data::Business;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    YELP_RED = #d32323

    BusinessCard = {{BusinessCard}} {
        width: Fill
        height: Fit
        padding: 16.0
        flow: Right
        spacing: 12.0

        show_bg: true
        draw_bg: { color: #fff }

        // Photo placeholder
        photo = <RoundedView> {
            width: 80.0
            height: 80.0
            show_bg: true
            draw_bg: {
                color: #e0e0e0
                border_radius: 4.0
            }
        }

        // Business info
        info = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4.0

            name_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: { font_size: 16.0 }
                    color: #1a1a1a
                }
                text: "Business Name"
            }

            // Rating row
            rating_row = <View> {
                width: Fill
                height: Fit
                flow: Right
                spacing: 4.0
                align: { y: 0.5 }

                stars = <RatingStars> {
                    rating: 4.5
                }

                rating_label = <Label> {
                    width: Fit
                    height: Fit
                    draw_text: {
                        text_style: { font_size: 13.0 }
                        color: #666
                    }
                    text: "4.5 (100)"
                }
            }

            meta_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: { font_size: 13.0 }
                    color: #666
                }
                text: "$$$ - Italian - 0.5 mi"
            }

            location_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: { font_size: 13.0 }
                    color: #999
                }
                text: "Mission District"
            }
        }
    }

    // Star rating widget
    RatingStars = {{RatingStars}} {
        width: 72.0
        height: 14.0
        show_bg: true

        draw_bg: {
            instance rating: 4.5
            instance star_color: #d32323
            instance empty_color: #ccc

            fn pixel(self) -> vec4 {
                let uv = self.pos;
                let star_width = 1.0 / 5.0;
                let star_idx = floor(uv.x / star_width);
                let local_x = fract(uv.x / star_width);
                let local_y = uv.y;

                // Star shape (simplified 5-pointed star)
                let cx = 0.5;
                let cy = 0.5;
                let px = local_x - cx;
                let py = local_y - cy;

                let angle = atan(py, px);
                let r = length(vec2(px, py));

                // Star function
                let n = 5.0;
                let m = 0.4;
                let star = cos(3.14159 / n) / cos(mod(angle, 2.0 * 3.14159 / n) - 3.14159 / n);
                let star_r = star * m;

                let in_star = step(r, star_r);

                // Fill based on rating
                let fill = clamp(self.rating - star_idx, 0.0, 1.0);
                let is_filled = step(local_x, fill);

                let color = mix(self.empty_color, self.star_color, is_filled);
                return mix(vec4(0.0), color, in_star);
            }
        }
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
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.action(BusinessCardAction::Clicked(self.business_id.clone()));
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
            }
            _ => {}
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update labels before drawing
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

#[derive(Clone, Debug, DefaultNone)]
pub enum BusinessCardAction {
    None,
    Clicked(String),
}

#[derive(Live, LiveHook, Widget)]
pub struct RatingStars {
    #[deref] view: View,
    #[live] rating: f64,
}

impl Widget for RatingStars {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
