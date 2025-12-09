//! Custom window frame implementations with title bars and controls.

use eframe::egui::{
    self, CentralPanel, Id, PointerButton, Sense, TextureHandle, UiBuilder, ViewportCommand,
};

use crate::ui::ui_constants::TITLE_BAR_HEIGHT;

/// Custom window frame with title bar, background image support, and controls.
pub fn custom_window_frame(
    ctx: &egui::Context,
    _title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
    on_ping_click: impl FnOnce(),
) {
    let panel_frame = egui::Frame::new()
        .fill(ctx.style().visuals.window_fill())
        .corner_radius(10)
        .outer_margin(1);

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        // Draw background image with low opacity if available
        if let Some(texture) =
            ctx.data(|d| d.get_temp::<Option<TextureHandle>>(egui::Id::new("background_texture")))
        {
            if let Some(ref tex) = texture {
                let painter = ui.painter();
                // Increased opacity for more visible background (0.3 = 30% opacity)
                let tint =
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * 0.3) as u8);
                painter.image(
                    tex.id(),
                    app_rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    tint,
                );
            }
        }

        let title_bar_height = TITLE_BAR_HEIGHT;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, _title, on_ping_click);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y + 4.0; // Add 8px margin from top of header
            rect
        }
        .shrink(4.0);

        let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
        add_contents(&mut content_ui);
    });
}

/// Title bar UI with ping button, minimize, and close controls.
pub fn title_bar_ui(
    ui: &mut egui::Ui,
    title_bar_rect: eframe::epaint::Rect,
    _title: &str,
    on_ping_click: impl FnOnce(),
) {
    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );

    // Left-side (top-left) controls: ping button
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(6.0);

            let button_height = 20.0;
            let ping_btn = ui
                .add(egui::Button::new(
                    egui::RichText::new("📶").size(button_height),
                ))
                .on_hover_text("Ping Monitor (8.8.8.8)")
                .on_hover_cursor(egui::CursorIcon::PointingHand);

            if ping_btn.clicked() {
                on_ping_click();
            }

            // keep remaining left-side space empty
            ui.add_space(4.0);
        },
    );

    if title_bar_response.drag_started_by(PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.scope_builder(
        UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_button(ui);
            ui.add_space(6.0);
            minimize_button(ui);
        },
    );
}

/// Show a minimize button for the native window.
pub fn minimize_button(ui: &mut egui::Ui) {
    let button_height = 20.0;

    let minimize_resp = ui
        .add(egui::Button::new(
            egui::RichText::new("➖").size(button_height),
        ))
        .on_hover_text("Minimize the window")
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    if minimize_resp.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}

/// Show a close button for the native window.
pub fn close_button(ui: &mut egui::Ui) {
    let button_height = 20.0;

    let close_resp = ui
        .add(egui::Button::new(
            egui::RichText::new("❌").size(button_height),
        ))
        .on_hover_text("Close the window")
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    if close_resp.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
    }
}

/// Simple window frame with title bar that only has a close button.
pub fn simple_window_frame(ctx: &egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
    let panel_frame = egui::Frame::new()
        .fill(ctx.style().visuals.window_fill())
        .outer_margin(0.0)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 80)));

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = TITLE_BAR_HEIGHT;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        simple_title_bar_ui(ui, title_bar_rect);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(1.0);

        let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
        add_contents(&mut content_ui);
    });
}

/// Title bar UI with only close button and drag functionality.
pub fn simple_title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect) {
    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("simple_title_bar"),
        Sense::click_and_drag(),
    );

    if title_bar_response.drag_started_by(PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.scope_builder(
        UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_button(ui);
            ui.add_space(6.0);
        },
    );
}
