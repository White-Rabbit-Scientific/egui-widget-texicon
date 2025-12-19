use egui::{
    pos2, text::LayoutJob, vec2, Align, Color32, CornerRadius, FontId, Image, ImageSource, Margin,
    Rect, RectAlign, Response, Sense, Stroke, Ui, Vec2, Widget,
};

#[derive(Default, PartialEq)]
pub enum TexiSense {
    #[default]
    Frame,
    ImageAndText,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon {
    img: ImageSource<'static>,
    enabled: bool,
    is_selected: bool,
    img_size: Vec2,
    img_scale_hov: f32,
    text: Option<String>, // Texicon text is optional
    text_size: f32,
    img_text_gap: f32,
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
    pub fn new(img: ImageSource<'static>) -> Self {
        Texicon {
            img,
            enabled: true,
            is_selected: false,
            img_size: vec2(32.0, 32.0),
            img_scale_hov: 1.0,
            text: None,
            text_size: 13.0,
            img_text_gap: 4.0,
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
    #[rustfmt::skip] #[inline]    pub fn enabled(mut self, enabled: bool) -> Self { self.enabled = enabled; self }
    #[rustfmt::skip] #[inline]    pub fn selected(mut self, is_selected: bool) -> Self { self.is_selected = is_selected; self }
    #[rustfmt::skip] #[inline]    pub fn img_size(mut self, img_size: Vec2) -> Self { self.img_size = img_size; self }
    #[rustfmt::skip] #[inline]    pub fn img_scale_hov(mut self, scale: f32) -> Self { self.img_scale_hov = scale; self }
    #[rustfmt::skip] #[inline]    pub fn text(mut self, text: impl Into<String>) -> Self { self.text = Some(text.into()); self }
    #[rustfmt::skip] #[inline]    pub fn text_size(mut self, size: f32) -> Self { self.text_size = size; self }
    #[rustfmt::skip] #[inline]    pub fn img_text_gap(mut self, img_text_gap: f32) -> Self { self.img_text_gap = img_text_gap; self }
    #[rustfmt::skip] #[inline]    pub fn sense(mut self, sense: TexiSense) -> Self { self.sense = sense; self }

    // Colors
    #[rustfmt::skip] #[inline]    pub fn bkgnd_col(mut self, col: Color32) -> Self { self.bkgnd_col = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn bkgnd_col_sel(mut self, col: Color32) -> Self { self.bkgnd_col_sel = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn bkgnd_col_hov(mut self, col: Color32) -> Self { self.bkgnd_col_hov = Some(col); self }

    #[rustfmt::skip] #[inline]    pub fn img_tint_col(mut self, col: Color32) -> Self { self.img_tint_col = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn img_tint_col_sel(mut self, col: Color32) -> Self { self.img_tint_col_sel = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn img_tint_col_hov(mut self, col: Color32) -> Self { self.img_tint_col_hov = Some(col); self }

    #[rustfmt::skip] #[inline]    pub fn text_col(mut self, col: Color32) -> Self { self.text_col = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn text_col_sel(mut self, col: Color32) -> Self { self.text_col_sel = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn text_col_hov(mut self, col: Color32) -> Self { self.text_col_hov = Some(col); self }

    #[rustfmt::skip] #[inline]    pub fn frame_col(mut self, col: Color32) -> Self { self.frame_col = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn frame_col_sel(mut self, col: Color32) -> Self { self.frame_col_sel = Some(col); self }
    #[rustfmt::skip] #[inline]    pub fn frame_col_hov(mut self, col: Color32) -> Self { self.frame_col_hov = Some(col); self }

    // Frame Config
    #[rustfmt::skip] #[inline]    pub fn frame_size(mut self, size: Vec2) -> Self { self.frame_size = size; self }
    #[rustfmt::skip] #[inline]    pub fn frame_width(mut self, width: f32) -> Self { self.frame_width = width; self }
    #[rustfmt::skip] #[inline]    pub fn inner_margin(mut self, inner_margin: Margin) -> Self { self.inner_margin = inner_margin; self }
    #[rustfmt::skip] #[inline]    pub fn radius(mut self, radius: u8) -> Self { self.radius = CornerRadius::same(radius); self }

    // Tooltip
    #[rustfmt::skip] #[inline]    pub fn tooltip_text(mut self, text: impl Into<String>) -> Self { self.tooltip_text = Some(text.into()); self }
    #[rustfmt::skip] #[inline]    pub fn tooltip_gap(mut self, tooltip_gap: f32) -> Self { self.tooltip_gap = tooltip_gap; self }
    #[rustfmt::skip] #[inline]    pub fn tooltip_position(mut self, pos: RectAlign) -> Self { self.tooltip_position = pos; self }
}

// Widget trait to enable the Texicon widget to be displayed
impl Widget for Texicon {
    fn ui(self, ui: &mut Ui) -> Response {
        // -------------------------------------------
        // Disable response interactions when disabled
        // There are 3 interaction sense sources:
        //   1. Outer widget frame
        //   2. Image
        //   3. Text gallery
        // -------------------------------------------
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::empty()
        };

        // ----------------------------------------------
        // Allocate the frame rect with interaction sense
        // ----------------------------------------------
        let (frame_rect, frame_resp) = ui.allocate_exact_size(self.frame_size, sense);

        // -------------------------------------------
        // Calculate image size (including up-scaling)
        // -------------------------------------------
        let image_size_scaled = self.img_size * self.img_scale_hov;
        let image_size_y = image_size_scaled.y;

        // -------------------
        // Calculate text size
        // -------------------
        // None means no text was supplied
        let mut galley = None;
        // Is there any text to display?
        if let Some(text) = self.text {
            // Calculate maximum width for text wrapping
            let inner_margin_total = (self.inner_margin.left + self.inner_margin.right) as f32;
            let wrap_width = self.frame_size.x - inner_margin_total;
            // Measure galley with text content
            let mut measure_job = LayoutJob::simple(
                text,
                FontId {
                    size: self.text_size,
                    ..Default::default()
                },
                Color32::PLACEHOLDER, // irrelevant for size
                wrap_width,
            );
            // Set text alignment
            measure_job.halign = Align::Center;
            // Lay out text in a galley
            galley = Some(ui.painter().layout_job(measure_job));
        }

        // -----------------------------------
        // Calculate image + gap + text height
        // -----------------------------------
        let total_img_gap_text_y =
            image_size_y + self.img_text_gap + galley.as_ref().map_or(0.0, |g| g.size().y);

        // --------------------------------
        // Calculate image center position
        // --------------------------------
        let frame_center = frame_rect.center();
        let img_center = pos2(
            frame_center.x,
            frame_center.y - (total_img_gap_text_y - image_size_y) / 2.0,
        );

        // --------------------------------------------------
        // Calculate image rect for original and scaled image
        // --------------------------------------------------
        let img_rect = Rect::from_center_size(img_center, self.img_size);
        let img_rect_scaled = Rect::from_center_size(img_center, image_size_scaled);

        // -------------------------------------
        // Create response for image interaction
        // -------------------------------------
        // We always respond to the scaled image size
        // since hovering cannot be known. This is ok so
        // long as the scaling is only a few percent.
        let img_resp = ui.interact(img_rect_scaled, frame_resp.id.with("img"), sense);

        // -------------------------------
        // Calculate text center position
        // -------------------------------
        let text_center = pos2(
            frame_center.x,
            frame_center.y
                + (total_img_gap_text_y - galley.as_ref().map_or(0.0, |g| g.size().y)) / 2.0,
        );

        // ----------------------------
        // Calculate text response rect
        // ----------------------------
        let text_rect = Rect::from_center_size(
            text_center,
            galley.as_ref().map_or(Vec2::ZERO, |g| g.size()),
        );

        // --------------------------------------
        // Increase sense area to include the gap
        // --------------------------------------
        let mut increased = text_rect;
        increased.min.y -= self.img_text_gap;

        // ------------------------------------
        // Create response for text interaction
        // ------------------------------------
        let text_resp = ui.interact(increased, frame_resp.id.with("txt"), sense);

        // ------------------------
        // Create union-ed response
        // ------------------------
        let response = match self.sense {
            TexiSense::Frame => img_resp.union(text_resp).union(frame_resp),
            TexiSense::ImageAndText => img_resp.union(text_resp),
        };

        // ------------------------
        // The colors heavy lifting
        // ------------------------
        let visuals = ui.visuals();
        let style_ina = visuals.widgets.inactive;
        let style_hov = visuals.widgets.hovered;
        let style_act = visuals.widgets.active;
        // Texicon colors
        let texi_bkgnd_col;
        let texi_text_col;
        let texi_img_tint;
        let texi_frame_col;
        // Greyed out colors when disabled
        if !self.enabled {
            texi_bkgnd_col = Color32::TRANSPARENT;
            texi_text_col = style_ina.bg_stroke.color.gamma_multiply_u8(128);
            texi_img_tint = style_ina.bg_stroke.color.gamma_multiply_u8(128);
            texi_frame_col = style_ina.bg_stroke.color.gamma_multiply_u8(128);
        } else if response.hovered() {
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

        // -----------------------------
        // Texicon frame width and color
        // -----------------------------
        let stroke = Stroke {
            width: self.frame_width,
            color: texi_frame_col,
        };

        // --------------------------------------------------
        // Add functionality for enabling / disabling Texicon
        // --------------------------------------------------
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.painter().rect(
                frame_rect,
                self.radius,
                texi_bkgnd_col,
                stroke,
                egui::StrokeKind::Inside,
            );

            // ------------------------------------
            // Convert ImageSource in Image (cheap)
            // ------------------------------------
            let image = Image::new(self.img).tint(texi_img_tint);

            // ---------------------------------------------------------------------
            // Paint the Texicon image; size depends upon scaling and hovering state
            // ---------------------------------------------------------------------
            let paint_rect = if response.hovered() {
                img_rect_scaled
            } else {
                img_rect
            };
            image.paint_at(ui, paint_rect);

            // --------------
            // Paint the text
            // --------------
            if let Some(g) = galley {
                ui.painter()
                    .galley(text_rect.center_top(), g, texi_text_col);
            }

            // -------------------------------------------
            // Show tooltip if Texicon enabled and hovered
            // -------------------------------------------
            if self.enabled && response.hovered() {
                if let Some(text) = self.tooltip_text {
                    let mut tooltip = egui::Tooltip::for_enabled(&response);
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
            }
        });
        response
    }
}
