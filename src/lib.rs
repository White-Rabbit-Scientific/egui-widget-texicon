use egui::{
    include_image, pos2, text::LayoutJob, vec2, Align, Color32, CornerRadius, FontId, Frame, Id,
    Image, ImageSource, Margin, Rect, RectAlign, Response, Sense, Stroke, Ui, Vec2, Widget,
};

#[derive(Default, PartialEq)]
pub enum TexiSense {
    #[default]
    Frame,
    ImageAndText,
}

#[derive(Default, Clone, Debug)]
pub struct TexiState {
    pub texi_being_hovered: bool,
    pub texi_selected: bool,
}

#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Texicon<'a> {
    texistate: &'a mut TexiState,
    texi_enabled: bool,
    texi_img: ImageSource<'static>,
    texi_img_size: Vec2,
    texi_text: String,
    texi_text_size: f32,
    texi_img_text_gap: f32,
    texi_top_gap: f32,
    texi_bottom_gap: f32,
    texi_sense: TexiSense,
    texi_bkgnd_col: Color32,
    texi_bkgnd_col_sel: Color32,
    texi_bkgnd_col_hov: Color32,
    texi_img_tint: Color32,
    texi_img_tint_sel: Color32,
    texi_img_tint_hov: Color32,
    texi_text_col: Color32,
    texi_text_col_sel: Color32,
    texi_text_col_hov: Color32,
    texi_frame_col: Color32,
    texi_frame_col_sel: Color32,
    texi_frame_col_hov: Color32,
    texi_frame_size: Vec2,
    texi_frame_width: f32,
    texi_inner_margin: Margin,
    texi_rounding: CornerRadius,
    texi_tooltip_text: Option<String>,
    texi_tooltip_gap: f32,
    texi_tooltip_position: RectAlign,
}

/// Default values for the Texicon struct and
/// a Builder Pattern implementation for customization.
impl<'a> Texicon<'a> {
    pub fn new(texistate: &'a mut TexiState) -> Self {
        // Set some default colors.
        // egui gamma_multiply_u8:
        //    "Multiply with 127 to make color
        //     half as opaque, perceptually."
        let stroke_col_def = Color32::LIGHT_GRAY.gamma_multiply_u8(160);
        let stroke_col_def_sel = Color32::LIGHT_GRAY.gamma_multiply_u8(220);
        let stroke_col_def_hov = Color32::LIGHT_GRAY.gamma_multiply_u8(255);
        let bkgnd_col_def = Color32::DARK_GRAY.gamma_multiply_u8(32);
        let bkgnd_col_sel = Color32::DARK_GRAY.gamma_multiply_u8(100);
        let bkgnd_col_hov = Color32::DARK_GRAY.gamma_multiply_u8(148);
        Texicon {
            texi_enabled: true,
            texistate,
            texi_img: include_image!("../assets/question.svg"),
            texi_img_size: vec2(32.0, 32.0),
            texi_text: "Missing image".to_string(),
            texi_text_size: 13.0,
            texi_img_text_gap: 0.0,
            texi_top_gap: 10.0,
            texi_bottom_gap: 10.0,
            texi_sense: Default::default(),
            texi_bkgnd_col: bkgnd_col_def,
            texi_bkgnd_col_sel: bkgnd_col_sel,
            texi_bkgnd_col_hov: bkgnd_col_hov,
            texi_img_tint: stroke_col_def,
            texi_img_tint_sel: stroke_col_def_sel,
            texi_img_tint_hov: stroke_col_def_hov,
            texi_text_col: stroke_col_def,
            texi_text_col_sel: stroke_col_def_sel,
            texi_text_col_hov: stroke_col_def_hov,
            texi_frame_col: stroke_col_def,
            texi_frame_col_sel: stroke_col_def_sel,
            texi_frame_col_hov: stroke_col_def_hov,
            texi_frame_size: vec2(70.0, 70.0),
            texi_frame_width: 1.0,
            texi_inner_margin: Margin::same(6),
            texi_rounding: CornerRadius::same(6),
            texi_tooltip_text: None,
            texi_tooltip_gap: 10.0,
            texi_tooltip_position: RectAlign::RIGHT,
        }
    }
    /// Set the enable/disable flag for the Texicon.
    #[inline]
    pub fn texi_enabled(mut self, texi_enabled: bool) -> Self {
        self.texi_enabled = texi_enabled;
        self
    }

    /// Set the image for the Texicon.
    #[inline]
    pub fn texi_img(mut self, texi_img: ImageSource<'static>) -> Self {
        self.texi_img = texi_img;
        self
    }

    /// Set the img_size for the Texicon.
    #[inline]
    pub fn texi_img_size(mut self, texi_img_size: Vec2) -> Self {
        self.texi_img_size = texi_img_size;
        self
    }

    /// Set the text for the Texicon.
    #[inline]
    pub fn texi_text(mut self, texi_text: String) -> Self {
        self.texi_text = texi_text;
        self
    }

    /// Set the text_size for the Texicon.
    #[inline]
    pub fn texi_text_size(mut self, texi_text_size: f32) -> Self {
        self.texi_text_size = texi_text_size;
        self
    }

    /// Set the img_text_gap for the Texicon.
    #[inline]
    pub fn texi_img_text_gap(mut self, texi_img_text_gap: f32) -> Self {
        self.texi_img_text_gap = texi_img_text_gap;
        self
    }

    /// Set the top_gap for the Texicon.
    #[inline]
    pub fn texi_top_gap(mut self, texi_top_gap: f32) -> Self {
        self.texi_top_gap = texi_top_gap;
        self
    }

    /// Set the bottom_gap for the Texicon.
    #[inline]
    pub fn texi_bottom_gap(mut self, texi_bottom_gap: f32) -> Self {
        self.texi_bottom_gap = texi_bottom_gap;
        self
    }

    /// Set the frame vs img+text sensing for the Texicon.
    #[inline]
    pub fn texi_sense(mut self, texi_sense: TexiSense) -> Self {
        self.texi_sense = texi_sense;
        self
    }

    /// Set the non-selected background color for the Texicon.
    #[inline]
    pub fn texi_bkgnd_col(mut self, texi_bkgnd_col: Color32) -> Self {
        self.texi_bkgnd_col = texi_bkgnd_col;
        self
    }

    /// Set the selected background color for the Texicon.
    #[inline]
    pub fn texi_bkgnd_col_sel(mut self, texi_bkgnd_col_sel: Color32) -> Self {
        self.texi_bkgnd_col_sel = texi_bkgnd_col_sel;
        self
    }

    /// Set the hovering background color for the Texicon.
    #[inline]
    pub fn texi_bkgnd_col_hov(mut self, texi_bkgnd_col_hov: Color32) -> Self {
        self.texi_bkgnd_col_hov = texi_bkgnd_col_hov;
        self
    }

    /// Set the non-selected image tint color for the Texicon.
    #[inline]
    pub fn texi_img_tint(mut self, texi_img_tint: Color32) -> Self {
        self.texi_img_tint = texi_img_tint;
        self
    }

    /// Set the selected image tint color for the Texicon.
    #[inline]
    pub fn texi_img_tint_sel(mut self, texi_img_tint_sel: Color32) -> Self {
        self.texi_img_tint_sel = texi_img_tint_sel;
        self
    }

    /// Set the hovering image tint color for the Texicon.
    #[inline]
    pub fn texi_img_tint_hov(mut self, texi_img_tint_hov: Color32) -> Self {
        self.texi_img_tint_hov = texi_img_tint_hov;
        self
    }

    /// Set the non-selected text color for the Texicon.
    #[inline]
    pub fn texi_text_col(mut self, texi_txt_col: Color32) -> Self {
        self.texi_text_col = texi_txt_col;
        self
    }

    /// Set the selected text color for the Texicon.
    #[inline]
    pub fn texi_text_col_sel(mut self, texi_txt_col_sel: Color32) -> Self {
        self.texi_text_col_sel = texi_txt_col_sel;
        self
    }

    /// Set the hovering text color for the Texicon.
    #[inline]
    pub fn texi_text_col_hov(mut self, texi_txt_col_hov: Color32) -> Self {
        self.texi_text_col_hov = texi_txt_col_hov;
        self
    }

    /// Set the non-selected frame color for the Texicon.
    #[inline]
    pub fn texi_frame_col(mut self, texi_frame_col: Color32) -> Self {
        self.texi_frame_col = texi_frame_col;
        self
    }

    /// Set the selected frame color for the Texicon.
    #[inline]
    pub fn texi_frame_col_sel(mut self, texi_frame_col_sel: Color32) -> Self {
        self.texi_frame_col_sel = texi_frame_col_sel;
        self
    }

    /// Set the hovering frame color for the Texicon.
    #[inline]
    pub fn texi_frame_col_hov(mut self, texi_frame_col_hov: Color32) -> Self {
        self.texi_frame_col_hov = texi_frame_col_hov;
        self
    }

    /// Set the frame_size for the Texicon.
    #[inline]
    pub fn texi_frame_size(mut self, texi_frame_size: Vec2) -> Self {
        self.texi_frame_size = texi_frame_size;
        self
    }

    /// Set the frame_width for the Texicon.
    #[inline]
    pub fn texi_frame_width(mut self, texi_frame_width: f32) -> Self {
        self.texi_frame_width = texi_frame_width;
        self
    }

    /// Set the inner_margin for the Texicon.
    #[inline]
    pub fn texi_inner_margin(mut self, texi_inner_margin: Margin) -> Self {
        self.texi_inner_margin = texi_inner_margin;
        self
    }

    /// Set the rounding for the Texicon.
    #[inline]
    pub fn texi_rounding(mut self, texi_rounding: CornerRadius) -> Self {
        self.texi_rounding = texi_rounding;
        self
    }

    /// Set the tooltip text for the Texicon.
    #[inline]
    pub fn texi_tooltip_text(mut self, texi_tooltip_text: Option<String>) -> Self {
        self.texi_tooltip_text = texi_tooltip_text;
        self
    }

    /// Set the tooltip gap for the Texicon.
    #[inline]
    pub fn texi_tooltip_gap(mut self, texi_tooltip_gap: f32) -> Self {
        self.texi_tooltip_gap = texi_tooltip_gap;
        self
    }

    /// Set the tooltip position/alignment for the Texicon.
    #[inline]
    pub fn texi_tooltip_position(mut self, texi_tooltip_position: RectAlign) -> Self {
        self.texi_tooltip_position = texi_tooltip_position;
        self
    }
}

/// Widget trait to enable the Texicon widget to be displayed
impl Widget for Texicon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        // Texicon colors
        let texi_bkgnd_color;
        let texi_text_color;
        let texi_img_tint_color;
        let texi_frame_color;
        // Update Txicon colors depending on state
        if self.texistate.texi_being_hovered {
            texi_bkgnd_color = self.texi_bkgnd_col_hov;
            texi_text_color = self.texi_text_col_hov;
            texi_img_tint_color = self.texi_img_tint_hov;
            texi_frame_color = self.texi_frame_col_hov;
        } else if self.texistate.texi_selected {
            texi_bkgnd_color = self.texi_bkgnd_col_sel;
            texi_text_color = self.texi_text_col_sel;
            texi_img_tint_color = self.texi_img_tint_sel;
            texi_frame_color = self.texi_frame_col_sel;
        } else {
            texi_bkgnd_color = self.texi_bkgnd_col;
            texi_text_color = self.texi_text_col;
            texi_img_tint_color = self.texi_img_tint;
            texi_frame_color = self.texi_frame_col;
        }

        // Texicon frame width and color
        let stroke = Stroke {
            width: self.texi_frame_width,
            color: texi_frame_color,
        };

        // Texicon frame
        let frame = Frame::default()
            .outer_margin(Margin::ZERO)
            .inner_margin(Margin::ZERO)
            .corner_radius(self.texi_rounding)
            .fill(texi_bkgnd_color)
            .stroke(stroke);

        // Add space before the frame
        ui.add_space(self.texi_top_gap);

        // Display Texicon as enabled or disabled
        let frame_output = ui.add_enabled_ui(self.texi_enabled, |ui| {
            // Show Texicon
            frame.show(ui, |ui| {
                // Set Texicon frame size
                ui.set_min_size(self.texi_frame_size);
                ui.set_max_size(self.texi_frame_size);

                // Allocate the full rect with interaction sense
                let (rect, response) = ui.allocate_exact_size(self.texi_frame_size, Sense::click());

                // Calculate center positions
                let center_x = rect.center().x;
                let center_y = rect.center().y;

                // Calculate maximum width for text wrapping
                let wrap_width = self.texi_frame_size.x
                    - (self.texi_inner_margin.left + self.texi_inner_margin.right) as f32;

                // LayoutJob for Texicon text
                let mut layout_job = LayoutJob::simple(
                    self.texi_text.to_string(),
                    FontId {
                        size: self.texi_text_size,
                        ..Default::default()
                    },
                    texi_text_color,
                    wrap_width,
                );
                // Center each line
                layout_job.halign = Align::Center;

                // Use painter's layout_job method
                let galley = ui.painter().layout_job(layout_job);

                // Calculate text area (this could be one or more lines)
                let galley_text_x = galley.size().x;
                let galley_text_y = galley.size().y;

                // Calculate y starting position
                let total_height = self.texi_img_size.y + self.texi_img_text_gap + galley_text_y;
                let start_y = center_y - (total_height / 2.0);

                // Image rect
                let img_pos = pos2(center_x - (self.texi_img_size.x / 2.0), start_y);
                let img_rect = Rect::from_min_size(img_pos, self.texi_img_size);

                // Paint the Texicon image, but...
                // First convert ImageSource in Image (= cheap)
                let image = Image::new(self.texi_img).tint(texi_img_tint_color);
                image.paint_at(ui, img_rect);

                // Text position
                let text_y = start_y + self.texi_img_size.y + self.texi_img_text_gap;

                // Final text position, now paint...
                let text_pos = pos2(center_x, text_y);
                ui.painter().galley(text_pos, galley, texi_text_color);

                // Create a unique base ID from the state reference
                let base_id = Id::new(self.texistate as *const _ as usize);

                // Create specific interaction areas
                let img_resp = ui.interact(img_rect, base_id.with("img"), Sense::click());

                // Get text rect for response
                let mut text_rect = Rect::from_center_size(
                    pos2(center_x, text_y + galley_text_y / 2.0),
                    vec2(galley_text_x, galley_text_y),
                );

                // Increase text_rect sense area to include the texi_img_text_gap
                text_rect.min.y = text_rect.min.y - self.texi_img_text_gap;

                // Final text response area (including img/text gap)
                let text_resp = ui.interact(text_rect, base_id.with("text"), Sense::click());

                (img_resp, text_resp, response)
            })
        });

        // Add space after the frame
        ui.add_space(self.texi_bottom_gap);

        // Aggregate responses
        let (img_resp, text_resp, frame_resp) = frame_output.inner.inner;

        // If ignoring outer frame response,
        // union the image and text responses
        let mut resp = img_resp.union(text_resp);

        // If including the outer frame response,
        // union frame with the image and text
        if self.texi_sense == TexiSense::Frame {
            resp = resp.union(frame_resp);
        }

        // Texicon hover depends upon response
        self.texistate.texi_being_hovered = resp.hovered();

        if let Some(text) = self.texi_tooltip_text {
            let mut tooltip = egui::Tooltip::for_enabled(&resp);
            let options = tooltip
                .popup
                .align(self.texi_tooltip_position)
                .gap(self.texi_tooltip_gap)
                .close_behavior(egui::PopupCloseBehavior::CloseOnClick);
            tooltip.popup = options;
            tooltip.show(|ui| {
                ui.label(text);
            });
        };

        resp
    }
}
