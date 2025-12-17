use egui::{
    pos2, text::LayoutJob, vec2, Align, Color32, CornerRadius, FontId, Frame, Image, ImageSource,
    Margin, Rect, RectAlign, Response, Sense, Stroke, Ui, Vec2, Widget,
};

#[derive(Default, PartialEq)]
pub enum TexiSense {
    #[default]
    Frame,
    ImageAndText,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon {
    uid: String,
    img: ImageSource<'static>,
    enabled: bool,
    is_selected: bool,
    is_hovered: bool,
    img_size: Vec2,
    img_scale_hov: f32,
    text: Option<String>, // Texicon text is optional
    text_size: f32,
    img_text_gap: f32,
    top_gap: f32,
    bottom_gap: f32,
    sense: TexiSense,
    bkgnd_col: Option<Color32>,
    bkgnd_col_sel: Option<Color32>,
    bkgnd_col_hov: Option<Color32>,
    img_tint_col: Option<Color32>,
    img_tint_col_sel: Option<Color32>,
    img_tint_col_hov: Option<Color32>,
    text_col: Option<Color32>,
    text_col_sel: Option<Color32>,
    text_col_hov: Option<Color32>,
    frame_col: Option<Color32>,
    frame_col_sel: Option<Color32>,
    frame_col_hov: Option<Color32>,
    frame_size: Vec2,
    frame_width: f32,
    inner_margin: Margin,
    radius: CornerRadius,
    tooltip_text: Option<String>,
    tooltip_gap: f32,
    tooltip_position: RectAlign,
}

/// Default values for the Texicon struct and a
/// Builder Pattern implementation for customization.
impl Texicon {
    pub fn new(texi_tid: &str, img: ImageSource<'static>) -> Self {
        Texicon {
            uid: texi_tid.to_string(),
            img,
            enabled: true,
            is_selected: false,
            is_hovered: false,
            img_size: vec2(32.0, 32.0),
            img_scale_hov: 1.0,
            text: Some("Missing text".to_string()),
            text_size: 13.0,
            img_text_gap: 4.0,
            top_gap: 10.0,
            bottom_gap: 10.0,
            sense: Default::default(),
            bkgnd_col: None,
            bkgnd_col_sel: None,
            bkgnd_col_hov: None,
            img_tint_col: None,
            img_tint_col_sel: None,
            img_tint_col_hov: None,
            text_col: None,
            text_col_sel: None,
            text_col_hov: None,
            frame_col: None,
            frame_col_sel: None,
            frame_col_hov: None,
            frame_size: vec2(70.0, 70.0),
            frame_width: 1.0,
            inner_margin: Margin::same(6),
            radius: CornerRadius::same(6),
            tooltip_text: None,
            tooltip_gap: 10.0,
            tooltip_position: RectAlign::RIGHT,
        }
    }
    /// Set the enable/disable flag for the Texicon.
    #[inline]
    pub fn texi_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the selected flag for the Texicon.
    #[inline]
    pub fn texi_is_selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    /// Set the enabled flag for the Texicon.
    #[inline]
    pub fn texi_is_hovered(mut self, is_hovered: bool) -> Self {
        self.is_hovered = is_hovered;
        self
    }

    /// Set the image for the Texicon.
    #[inline]
    pub fn texi_img(mut self, img: ImageSource<'static>) -> Self {
        self.img = img;
        self
    }

    /// Set the img_size for the Texicon.
    #[inline]
    pub fn texi_img_size(mut self, img_size: Vec2) -> Self {
        self.img_size = img_size;
        self
    }

    /// Set the img_size for the Texicon.
    #[inline]
    pub fn texi_img_scale_hov(mut self, img_scale_hov: f32) -> Self {
        self.img_scale_hov = img_scale_hov;
        self
    }

    /// Set the text for the Texicon.
    #[inline]
    pub fn texi_text(mut self, text: Option<String>) -> Self {
        self.text = text;
        self
    }

    /// Set the text_size for the Texicon.
    #[inline]
    pub fn texi_text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    /// Set the img_text_gap for the Texicon.
    #[inline]
    pub fn texi_img_text_gap(mut self, img_text_gap: f32) -> Self {
        self.img_text_gap = img_text_gap;
        self
    }

    /// Set the top_gap for the Texicon.
    #[inline]
    pub fn texi_top_gap(mut self, top_gap: f32) -> Self {
        self.top_gap = top_gap;
        self
    }

    /// Set the bottom_gap for the Texicon.
    #[inline]
    pub fn texi_bottom_gap(mut self, bottom_gap: f32) -> Self {
        self.bottom_gap = bottom_gap;
        self
    }

    /// Set the frame vs img+text sensing for the Texicon.
    #[inline]
    pub fn texi_sense(mut self, sense: TexiSense) -> Self {
        self.sense = sense;
        self
    }

    /// Set the non-selected background color for the Texicon.
    #[inline]
    pub fn texi_bkgnd_col(mut self, bkgnd_col: Color32) -> Self {
        self.bkgnd_col = Some(bkgnd_col);
        self
    }

    /// Set the selected background color for the Texicon.
    #[inline]
    pub fn texi_bkgnd_col_sel(mut self, bkgnd_col_sel: Color32) -> Self {
        self.bkgnd_col_sel = Some(bkgnd_col_sel);
        self
    }

    /// Set the hovering background color for the Texicon.
    #[inline]
    pub fn texi_bkgnd_col_hov(mut self, bkgnd_col_hov: Color32) -> Self {
        self.bkgnd_col_hov = Some(bkgnd_col_hov);
        self
    }

    /// Set the non-selected image tint color for the Texicon.
    #[inline]
    pub fn texi_img_tint_col(mut self, img_tint_col: Color32) -> Self {
        self.img_tint_col = Some(img_tint_col);
        self
    }

    /// Set the selected image tint color for the Texicon.
    #[inline]
    pub fn texi_img_tint_col_sel(mut self, img_tint_col_sel: Color32) -> Self {
        self.img_tint_col_sel = Some(img_tint_col_sel);
        self
    }

    /// Set the hovering image tint color for the Texicon.
    #[inline]
    pub fn texi_img_tint_col_hov(mut self, img_tint_col_hov: Color32) -> Self {
        self.img_tint_col_hov = Some(img_tint_col_hov);
        self
    }

    /// Set the non-selected text color for the Texicon.
    #[inline]
    pub fn texi_text_col(mut self, text_col: Color32) -> Self {
        self.text_col = Some(text_col);
        self
    }

    /// Set the selected text color for the Texicon.
    #[inline]
    pub fn texi_text_col_sel(mut self, text_col_sel: Color32) -> Self {
        self.text_col_sel = Some(text_col_sel);
        self
    }

    /// Set the hovering text color for the Texicon.
    #[inline]
    pub fn texi_text_col_hov(mut self, text_col_hov: Color32) -> Self {
        self.text_col_hov = Some(text_col_hov);
        self
    }

    /// Set the non-selected frame color for the Texicon.
    #[inline]
    pub fn texi_frame_col(mut self, frame_col: Color32) -> Self {
        self.frame_col = Some(frame_col);
        self
    }

    /// Set the selected frame color for the Texicon.
    #[inline]
    pub fn texi_frame_col_sel(mut self, frame_col_sel: Color32) -> Self {
        self.frame_col_sel = Some(frame_col_sel);
        self
    }

    /// Set the hovering frame color for the Texicon.
    #[inline]
    pub fn texi_frame_col_hov(mut self, frame_col_hov: Color32) -> Self {
        self.frame_col_hov = Some(frame_col_hov);
        self
    }

    /// Set the frame_size for the Texicon.
    #[inline]
    pub fn texi_frame_size(mut self, frame_size: Vec2) -> Self {
        self.frame_size = frame_size;
        self
    }

    /// Set the frame_width for the Texicon.
    #[inline]
    pub fn texi_frame_width(mut self, frame_width: f32) -> Self {
        self.frame_width = frame_width;
        self
    }

    /// Set the inner_margin for the Texicon.
    #[inline]
    pub fn texi_inner_margin(mut self, inner_margin: Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    /// Set the rounding for the Texicon.
    #[inline]
    pub fn texi_radius(mut self, radius: u8) -> Self {
        self.radius = CornerRadius::same(radius);
        self
    }

    /// Set the tooltip text for the Texicon.
    #[inline]
    pub fn texi_tooltip_text(mut self, tooltip_text: Option<String>) -> Self {
        self.tooltip_text = tooltip_text;
        self
    }

    /// Set the tooltip gap for the Texicon.
    #[inline]
    pub fn texi_tooltip_gap(mut self, tooltip_gap: f32) -> Self {
        self.tooltip_gap = tooltip_gap;
        self
    }

    /// Set the tooltip position/alignment for the Texicon.
    #[inline]
    pub fn texi_tooltip_position(mut self, tooltip_position: RectAlign) -> Self {
        self.tooltip_position = tooltip_position;
        self
    }
}

/// Widget trait to enable the Texicon widget to be displayed
///
///
impl Widget for Texicon {
    fn ui(self, ui: &mut Ui) -> Response {
        let visuals = ui.visuals();
        let style_ina = visuals.widgets.inactive;
        let style_hov = visuals.widgets.hovered;
        let style_act = visuals.widgets.active;

        // Texicon colors
        let texi_bkgnd_col;
        let texi_text_col;
        let texi_img_tint;
        let texi_frame_col;
        // Update Texicon colors depending on state
        if self.is_hovered {
            texi_bkgnd_col = self.bkgnd_col_hov.unwrap_or(style_hov.bg_fill);
            texi_text_col = self.text_col_hov.unwrap_or(visuals.strong_text_color());
            texi_img_tint = self.img_tint_col_hov.unwrap_or(visuals.strong_text_color());
            texi_frame_col = self.frame_col_hov.unwrap_or(visuals.strong_text_color());
        } else if self.is_selected {
            texi_bkgnd_col = self.bkgnd_col_sel.unwrap_or(style_act.bg_fill);
            texi_text_col = self.text_col_sel.unwrap_or(style_act.text_color());
            texi_img_tint = self.img_tint_col_sel.unwrap_or(style_act.text_color());
            texi_frame_col = self.frame_col_sel.unwrap_or(style_act.text_color());
        } else {
            texi_bkgnd_col = self.bkgnd_col.unwrap_or(style_ina.weak_bg_fill);
            texi_text_col = self.text_col.unwrap_or(visuals.weak_text_color());
            texi_img_tint = self.img_tint_col.unwrap_or(visuals.weak_text_color());
            texi_frame_col = self.frame_col.unwrap_or(visuals.weak_text_color());
        }

        // Scale image size if hovered
        let mut image_size = self.img_size;
        if self.is_hovered {
            image_size.x *= self.img_scale_hov;
            image_size.y *= self.img_scale_hov;
        }
        // Texicon frame width and color
        let stroke = Stroke {
            width: self.frame_width,
            color: texi_frame_col,
        };

        // Texicon frame
        let frame = Frame::default()
            .outer_margin(Margin::ZERO)
            .inner_margin(Margin::ZERO)
            .corner_radius(self.radius)
            .fill(texi_bkgnd_col)
            .stroke(stroke);

        // Add space before the frame
        ui.add_space(self.top_gap);

        // Display Texicon as enabled or disabled
        let frame_resp = ui.add_enabled_ui(self.enabled, |ui| {
            // Show Texicon
            frame.show(ui, |ui| {
                // Create a unique base ID
                let base_id = ui.id().with(self.uid);

                // Set Texicon frame size
                ui.set_min_size(self.frame_size);
                ui.set_max_size(self.frame_size);

                // Allocate the full rect with interaction sense
                let (rect, response) = ui.allocate_exact_size(self.frame_size, Sense::click());

                // Calculate center positions
                let center_x = rect.center().x;
                let center_y = rect.center().y;

                // Initialize start_y
                let start_y;

                // If Texicon contains text
                let text_resp = if let Some(text) = &self.text {
                    // Calculate maximum width for text wrapping
                    let wrap_width = self.frame_size.x
                        - (self.inner_margin.left + self.inner_margin.right) as f32;

                    // LayoutJob for Texicon text
                    let mut layout_job = LayoutJob::simple(
                        text.to_string(),
                        FontId {
                            size: self.text_size,
                            ..Default::default()
                        },
                        texi_text_col,
                        wrap_width,
                    );
                    // Center each line
                    layout_job.halign = Align::Center;
                    // layout_job.sections[0].format.color = texi_text_col;

                    // Use painter's layout_job method
                    let galley = ui.painter().layout_job(layout_job);

                    // Calculate text area (this could be one or more lines)
                    let galley_text_x = galley.size().x;
                    let galley_text_y = galley.size().y;

                    // Calculate y starting position
                    let total_height = image_size.y + self.img_text_gap + galley_text_y;
                    start_y = center_y - (total_height / 2.0);

                    let text_adjustment = (image_size.y - self.img_size.y) / 2.0;

                    // Text position, does not change with image scale on hover
                    let text_y = start_y + image_size.y + self.img_text_gap - text_adjustment;

                    // Final text position, now paint...
                    let text_pos = pos2(center_x, text_y);
                    ui.painter().galley(text_pos, galley, texi_text_col);

                    // Get text rect for response
                    let mut text_rect = Rect::from_center_size(
                        pos2(center_x, text_y + galley_text_y / 2.0),
                        vec2(galley_text_x, galley_text_y),
                    );

                    // Increase text_rect sense area to include the texi_img_text_gap
                    text_rect.min.y = text_rect.min.y - self.img_text_gap;

                    // Final text response area (including img/text gap)
                    ui.interact(text_rect, base_id.with("text"), Sense::click())
                } else {
                    // No text, just image
                    start_y = center_y - image_size.y / 2.0;
                    ui.response()
                };

                // Image rect
                let img_pos = pos2(center_x - (image_size.x / 2.0), start_y);
                let img_rect = Rect::from_min_size(img_pos, image_size);

                // Paint the Texicon image, but...
                // First convert ImageSource in Image (= cheap)
                let image = Image::new(self.img).tint(texi_img_tint);
                image.paint_at(ui, img_rect);

                // Create specific interaction areas
                let img_resp = ui.interact(img_rect, base_id.with("img"), Sense::click());

                (img_resp, text_resp, response)
            })
        });

        // Add space after the frame
        ui.add_space(self.bottom_gap);

        // Aggregate responses
        let (img_resp, text_resp, frame_resp) = frame_resp.inner.inner;

        // If ignoring outer frame response,
        // union the image and text responses
        let mut resp = img_resp.union(text_resp);

        // If including the outer frame response,
        // union frame with the image and text
        if self.sense == TexiSense::Frame {
            resp = resp.union(frame_resp);
        }

        // Tooltip
        if let Some(text) = self.tooltip_text {
            let mut tooltip = egui::Tooltip::for_enabled(&resp);
            let options = tooltip
                .popup
                .align(self.tooltip_position)
                .gap(self.tooltip_gap)
                .close_behavior(egui::PopupCloseBehavior::CloseOnClick);
            tooltip.popup = options;
            tooltip.show(|ui| {
                ui.label(text);
            });
        };

        // The response comes from one of two possible sources:
        // 1. The image + text responses (ignores the outer frame response)
        // 2. The outer frame response (includes image + text responses)
        resp
    }
}
