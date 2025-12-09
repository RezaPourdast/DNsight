//! UI constants, theme configuration, and rendering functions.

use eframe::egui::{self, TextureHandle, Vec2};

use crate::domain::{AppState, DnsProvider, DnsState};

// ============================================================================
// UI CONSTANTS
// ============================================================================

/// UI spacing constants
pub mod ui_constants {
    pub const SPACING_SMALL: f32 = 10.0;
    pub const _SPACING_MEDIUM: f32 = 20.0;
    pub const _SPACING_LARGE: f32 = 30.0;
    pub const _SPACING_XLARGE: f32 = 40.0;

    pub const BUTTON_WIDTH: f32 = 200.0;
    pub const BUTTON_HEIGHT: f32 = 40.0;
    pub const BUTTON_SPACING: f32 = 3.0;

    pub const TITLE_BAR_HEIGHT: f32 = 30.0;
    pub const _WINDOW_PADDING: f32 = 4.0; // Reserved for future use
}

/// UI color constants
pub mod ui_colors {
    use eframe::egui::Color32;

    pub const BUTTON_SUCCESS: Color32 = Color32::from_rgb(60, 140, 64); // Darker #4CAF50
    pub const BUTTON_DANGER: Color32 = Color32::from_rgb(183, 46, 42); // Darker #E53935
    pub const BUTTON_TEXT: Color32 = Color32::WHITE;

    pub const STATUS_STATIC: Color32 = Color32::GREEN;
    pub const STATUS_DHCP: Color32 = Color32::YELLOW;
    pub const STATUS_NONE: Color32 = Color32::RED;

    pub const SUCCESS: Color32 = Color32::GREEN;
    pub const ERROR: Color32 = Color32::RED;
    pub const WARNING: Color32 = Color32::YELLOW;
}

use ui_colors::*;
use ui_constants::*;

// ============================================================================
// THEME CONFIGURATION
// ============================================================================

use std::sync::atomic::{AtomicBool, Ordering};

// Track if theme has been configured
static THEME_CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Configure UI theme and styling.
pub fn configure_theme(ctx: &egui::Context) {
    if !THEME_CONFIGURED.swap(true, Ordering::SeqCst) {
        let mut style = (*ctx.style()).clone();
        // Configure spacing
        style.spacing.item_spacing = egui::vec2(SPACING_SMALL, SPACING_SMALL);
        ctx.set_style(style);
    }
}

// ============================================================================
// UI RENDERING FUNCTIONS
// ============================================================================

/// Render IP address input field with validation.
pub fn render_ip_input(ui: &mut egui::Ui, ip: &mut String, label: &str) -> bool {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(format!("{}: ", label)).color(egui::Color32::WHITE));

        let field_id = egui::Id::new(label);
        let ip_clone = ip.clone();
        let is_valid = ip_clone.is_empty() || is_valid_ip(&ip_clone);

        let mut text_edit = egui::TextEdit::singleline(ip)
            .desired_width(200.0)
            .id(field_id)
            .text_color(egui::Color32::WHITE);

        if !ip_clone.is_empty() && !is_valid {
            text_edit = text_edit.text_color(egui::Color32::RED);
        }

        ui.add_sized(Vec2::new(200.0, 20.0), text_edit);
    });

    ip.is_empty() || is_valid_ip(ip)
}

/// Validate IP address format.
pub fn is_valid_ip(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    for part in parts {
        if part.parse::<u8>().is_err() {
            return false;
        }
    }
    true
}

/// Render status section showing current DNS configuration.
pub fn render_status_section(
    ui: &mut egui::Ui,
    dns_state: &DnsState,
    app_state: &AppState,
    on_test_click: impl FnOnce(),
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Current Status")
                    .color(egui::Color32::WHITE)
                    .size(18.0),
            );
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                let test_btn = ui
                    .add_sized(
                        Vec2::new(22.0, 22.0),
                        egui::Button::new(egui::RichText::new("🔄").size(16.0)).frame(false),
                    )
                    .on_hover_text("Test DNS")
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                if test_btn.clicked() {
                    on_test_click();
                }
            });
        });
    });

    match dns_state {
        DnsState::Static(servers) => {
            ui.colored_label(STATUS_STATIC, "Static DNS Configuration 🔒");
            let fallback = String::from("None");
            let primary = servers.first().unwrap_or(&fallback);
            ui.label(
                egui::RichText::new(format!("Primary: {}", primary)).color(egui::Color32::WHITE),
            );
            if servers.len() > 1 {
                let secondary = servers.get(1).unwrap_or(&fallback);
                ui.label(
                    egui::RichText::new(format!("Secondary: {}", secondary))
                        .color(egui::Color32::WHITE),
                );
            }
        }
        DnsState::Dhcp => {
            ui.colored_label(STATUS_DHCP, "🔄 DHCP DNS Configuration");
        }
        DnsState::None => {
            ui.colored_label(STATUS_NONE, "❌ No DNS Configuration");

            // Show DNSIGHT only when app is Idle or Processing
            let show_app_name = matches!(app_state, AppState::Idle | AppState::Processing);
            if show_app_name {
                ui.add_space(50.0);
                ui.label(
                    egui::RichText::new("welcome to DNSIGHT!  💙")
                        .color(egui::Color32::LIGHT_BLUE)
                        .size(16.0),
                );
            }
        }
    }
}

/// Render provider selection dropdown.
pub fn render_provider_selection(
    ui: &mut egui::Ui,
    selected_provider: &DnsProvider,
    custom_primary: &str,
    custom_secondary: &str,
    saved_entries: &[crate::domain::SavedDnsEntry],
    mut on_provider_change: impl FnMut(DnsProvider),
    mut on_custom_selected: impl FnMut(),
    mut on_add_new: impl FnMut(),
) {
    let mut providers: Vec<(&str, DnsProvider)> = vec![
        ("Electro", DnsProvider::electro()),
        ("Radar", DnsProvider::radar()),
        ("Shekan", DnsProvider::shekan()),
        ("Bogzar", DnsProvider::bogzar()),
        ("Quad9", DnsProvider::quad9()),
        (
            "Custom",
            DnsProvider::custom(custom_primary.to_string(), custom_secondary.to_string()),
        ),
    ];

    // Add saved entries
    for entry in saved_entries {
        providers.push((
            &entry.name,
            DnsProvider::saved(
                entry.name.clone(),
                entry.primary.clone(),
                entry.secondary.clone(),
            ),
        ));
    }

    // Add "+" option at the end
    providers.push(("+", DnsProvider::custom(String::new(), String::new())));

    let current_index = providers
        .iter()
        .position(|(_, provider)| match (provider, selected_provider) {
            (DnsProvider::Saved { name: n1, .. }, DnsProvider::Saved { name: n2, .. }) => n1 == n2,
            _ => std::mem::discriminant(provider) == std::mem::discriminant(selected_provider),
        })
        .unwrap_or(0);

    let selected_provider_opt = std::cell::Cell::new(None::<DnsProvider>);
    let should_open_custom = std::cell::Cell::new(false);

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        // Store original styles
        let original_padding = ui.style().spacing.button_padding;
        let original_bg_fill = ui.style().visuals.widgets.inactive.bg_fill;
        let original_corner_radius = ui.style().visuals.widgets.inactive.corner_radius;

        // Calculate padding to achieve button height
        let text_size = ui.style().text_styles[&egui::TextStyle::Body].size;
        let vertical_padding = ((BUTTON_HEIGHT / 2.0 + 5.0) - text_size) / 2.0;
        ui.style_mut().spacing.button_padding = egui::vec2(8.0, vertical_padding.max(0.0));

        // Make combobox semi-transparent with rounded corners
        let bg_opacity = 45;
        let dark_gray = 255;
        ui.style_mut().visuals.widgets.inactive.bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.hovered.bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, 80);
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, 80);
        ui.style_mut().visuals.widgets.active.bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.active.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.noninteractive.bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.noninteractive.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.open.bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);
        ui.style_mut().visuals.widgets.open.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(dark_gray, dark_gray, dark_gray, bg_opacity);

        let corner_radius = egui::CornerRadius {
            nw: 6,
            ne: 6,
            sw: 6,
            se: 6,
        };
        ui.style_mut().visuals.widgets.inactive.corner_radius = corner_radius;
        ui.style_mut().visuals.widgets.hovered.corner_radius = corner_radius;
        ui.style_mut().visuals.widgets.active.corner_radius = corner_radius;
        ui.style_mut().visuals.widgets.noninteractive.corner_radius = corner_radius;
        ui.style_mut().visuals.widgets.open.corner_radius = corner_radius;

        egui::ComboBox::from_id_salt("dns_provider")
            .selected_text(
                egui::RichText::new(providers[current_index].0).color(egui::Color32::WHITE),
            )
            .width(BUTTON_WIDTH)
            .show_ui(ui, |ui| {
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);

                for (name, provider) in &providers {
                    // Handle "+" option separately
                    if *name == "+" {
                        if ui.selectable_label(false, "➕ Add New").clicked() {
                            on_add_new();
                        }
                        continue;
                    }

                    let was_selected = if *name == "Custom"
                        && matches!(selected_provider, DnsProvider::Custom { .. })
                    {
                        true
                    } else if let DnsProvider::Saved { name: sn, .. } = selected_provider {
                        *name == sn.as_str()
                    } else {
                        std::mem::discriminant(provider)
                            == std::mem::discriminant(selected_provider)
                    };

                    if ui.selectable_label(was_selected, *name).clicked() {
                        let is_custom = matches!(provider, DnsProvider::Custom { .. });
                        selected_provider_opt.set(Some(provider.clone()));
                        if is_custom {
                            should_open_custom.set(true);
                        }
                    }
                }
            });

        // Restore original styles
        ui.style_mut().spacing.button_padding = original_padding;
        ui.style_mut().visuals.widgets.inactive.bg_fill = original_bg_fill;
        ui.style_mut().visuals.widgets.inactive.corner_radius = original_corner_radius;
    });

    if let Some(provider) = selected_provider_opt.take() {
        on_provider_change(provider);
        if should_open_custom.get() {
            on_custom_selected();
        }
    }
}

/// Render application state (idle, processing, success, error, warning).
pub fn render_app_state(ui: &mut egui::Ui, app_state: &AppState) {
    match app_state {
        AppState::Idle => {}
        AppState::Processing => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Processing DNS operation...");
            });
        }
        AppState::Success(message) => {
            ui.colored_label(SUCCESS, format!("✅ {}", message));
        }
        AppState::Error(message) => {
            ui.colored_label(ERROR, format!("❌ {}", message));
        }
        AppState::Warning(message) => {
            ui.colored_label(WARNING, format!("⚠️ {}", message));
        }
    }
}

/// Render action buttons (Set DNS, Clear DNS, and optionally Delete for saved entries).
pub fn render_action_buttons(
    ui: &mut egui::Ui,
    provider_name: &str,
    on_set_dns: impl FnOnce(),
    on_clear_dns: impl FnOnce(),
    on_delete: Option<impl FnOnce()>,
) {
    ui.vertical_centered(|ui| {
        // Set DNS button
        if ui
            .add_sized(
                Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT),
                egui::Button::new(
                    egui::RichText::new(format!("Set {} DNS", provider_name))
                        .color(BUTTON_TEXT)
                        .strong()
                        .size(14.0),
                )
                .fill(BUTTON_SUCCESS)
                .corner_radius(6),
            )
            .clicked()
        {
            on_set_dns();
        }

        ui.add_space(BUTTON_SPACING);

        // Clear DNS button
        if ui
            .add_sized(
                Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT),
                egui::Button::new(
                    egui::RichText::new("Clear DNS")
                        .color(BUTTON_TEXT)
                        .strong()
                        .size(14.0),
                )
                .fill(BUTTON_DANGER)
                .corner_radius(6),
            )
            .clicked()
        {
            on_clear_dns();
        }

        // Delete button for saved entries (only shown when on_delete is Some)
        if let Some(delete_callback) = on_delete {
            ui.add_space(BUTTON_SPACING);
            if ui
                .add_sized(
                    Vec2::new(BUTTON_WIDTH / 2.0, 30.0), // Half width, smaller height
                    egui::Button::new(
                        egui::RichText::new(format!("Delete ({})", provider_name))
                            .color(egui::Color32::WHITE)
                            .size(12.0),
                    )
                    .fill(egui::Color32::from_rgba_unmultiplied(100, 100, 100, 100)) // Gray transparent
                    .corner_radius(6),
                )
                .clicked()
            {
                delete_callback();
            }
        }
    });
}

/// Render footer with social media links.
pub fn render_footer(
    ui: &mut egui::Ui,
    social_logos: &std::collections::HashMap<String, TextureHandle>,
    has_delete_button: bool,
) {
    let icon_size = 28.0;
    let icon_spacing = 15.0;
    let light_gray = egui::Color32::from_rgb(180, 180, 180);

    ui.vertical(|ui| {
        // Reduce spacing when delete button is shown to keep icons in place
        let top_spacing = if has_delete_button { 10.0 } else { 40.0 };
        ui.add_space(top_spacing);
        ui.horizontal(|ui| {
            ui.add_space(52.5);
            let logos = vec![
                ("cup-of-drink", "https://www.coffeete.ir/rezapourdast"),
                ("email", "mailto:s.rezapourdast@gmail.com"),
                ("github", "https://github.com/RezaPourdast"),
            ];

            for (logo_name, url) in logos {
                if let Some(texture) = social_logos.get(logo_name) {
                    let (rect, response) = ui
                        .allocate_exact_size(Vec2::new(icon_size, icon_size), egui::Sense::click());

                    let painter = ui.painter();
                    painter.image(
                        texture.id(),
                        rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        light_gray,
                    );

                    if response.clicked() {
                        let _ = open::that(url);
                    }

                    if response.hovered() {
                        painter.rect_filled(
                            rect,
                            0.0,
                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30),
                        );
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    ui.add_space(icon_spacing);
                }
            }
        });
    });
}

/// Render ping monitor window content.
pub fn render_ping_window_content(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    ping_value: f64,
    ping_history: &[f64],
) {
    // Draw ping background image with low opacity if available
    if let Some(texture) =
        ctx.data(|d| d.get_temp::<Option<TextureHandle>>(egui::Id::new("ping_background_texture")))
    {
        if let Some(ref tex) = texture {
            let painter = ui.painter();
            let viewport_rect = ui.ctx().viewport_rect();
            let tint = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * 0.3) as u8);
            painter.image(
                tex.id(),
                viewport_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                tint,
            );
        }
    }

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.vertical_centered(|ui| {
            ui.heading(" Ping Monitor");
            ui.add_space(10.0);

            let ping_text = format!("{} ms", ping_value);
            let ping_color = if ping_value == 0.0 {
                egui::Color32::LIGHT_GRAY
            } else if ping_value < 100.0 {
                egui::Color32::GREEN
            } else if ping_value < 200.0 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };

            ui.label(egui::RichText::new(ping_text).size(28.0).color(ping_color));
        });

        ui.add_space(10.0);

        // Ping history chart
        if !ping_history.is_empty() {
            let line_color = if ping_value == 0.0 {
                egui::Color32::LIGHT_GRAY
            } else if ping_value < 100.0 {
                egui::Color32::GREEN
            } else if ping_value < 200.0 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };

            let chart_height = 150.0;
            let chart_margin = 40.0;
            let chart_width = ui.available_width() - (chart_margin * 2.0);
            let (chart_rect, _) =
                ui.allocate_exact_size(egui::vec2(chart_width, chart_height), egui::Sense::hover());
            let chart_rect = chart_rect.translate(egui::vec2(chart_margin, 0.0));

            let painter = ui.painter();

            // Draw background
            painter.rect_filled(
                chart_rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(20, 20, 20, 100),
            );

            // Find min/max for scaling
            let min_val = ping_history
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min)
                .max(0.0);
            let max_val = ping_history
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
                .max(100.0);
            let range = (max_val - min_val).max(1.0);

            // Draw grid lines
            let grid_color = egui::Color32::from_rgba_unmultiplied(150, 150, 150, 30);
            for i in 0..=4 {
                let y = chart_rect.min.y + (chart_rect.height() / 4.0) * i as f32;
                painter.line_segment(
                    [
                        egui::pos2(chart_rect.min.x, y),
                        egui::pos2(chart_rect.max.x, y),
                    ],
                    egui::Stroke::new(1.0, grid_color),
                );
            }

            if ping_history.len() > 1 {
                let num_vertical_lines = (ping_history.len() - 1).min(10);
                for i in 0..=num_vertical_lines {
                    let x = chart_rect.min.x
                        + (chart_rect.width() / num_vertical_lines as f32) * i as f32;
                    painter.line_segment(
                        [
                            egui::pos2(x, chart_rect.min.y),
                            egui::pos2(x, chart_rect.max.y),
                        ],
                        egui::Stroke::new(1.0, grid_color),
                    );
                }
            }

            // Draw ping line
            if ping_history.len() > 1 {
                let points: Vec<egui::Pos2> = ping_history
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| {
                        let x = chart_rect.min.x
                            + (chart_rect.width() / (ping_history.len() - 1).max(1) as f32)
                                * i as f32;
                        let normalized = (value - min_val) / range;
                        let y = chart_rect.max.y - (chart_rect.height() * normalized as f32);
                        egui::pos2(x, y)
                    })
                    .collect();

                for i in 0..points.len() - 1 {
                    painter.line_segment(
                        [points[i], points[i + 1]],
                        egui::Stroke::new(2.0, line_color),
                    );
                }

                for point in &points {
                    painter.circle_filled(*point, 3.0, line_color);
                }
            }

            // Draw Y-axis labels
            let label_color = egui::Color32::WHITE;
            for i in 0..=4 {
                let value = max_val - (range / 4.0) * i as f64;
                let y = chart_rect.min.y + (chart_rect.height() / 4.0) * i as f32;
                painter.text(
                    egui::pos2(chart_rect.min.x - 5.0, y),
                    egui::Align2::RIGHT_CENTER,
                    format!("{:.0}", value),
                    egui::FontId::monospace(10.0),
                    label_color,
                );
            }

            // Show average of last 15 pings
            let avg = if ping_history.is_empty() {
                0.0
            } else {
                ping_history.iter().copied().sum::<f64>() / ping_history.len() as f64
            };
            ui.add_space(6.0);
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "Avg (last {}): {:.1} ms",
                        ping_history.len(),
                        avg
                    ))
                    .color(egui::Color32::from_rgb(180, 180, 180))
                    .size(12.0),
                );
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new("Waiting for ping data...").color(egui::Color32::GRAY),
                );
            });
        }
    });
}

/// Render custom DNS window content.
pub fn render_custom_dns_window_content(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    custom_primary: &mut String,
    custom_secondary: &mut String,
    on_save: impl FnOnce(),
    on_clear: impl FnOnce(),
) {
    // Draw custom DNS background image with low opacity if available
    if let Some(texture) = ctx.data(|d| {
        d.get_temp::<Option<TextureHandle>>(egui::Id::new("custom_dns_background_texture"))
    }) {
        if let Some(ref tex) = texture {
            let painter = ui.painter();
            let viewport_rect = ui.ctx().viewport_rect();
            let tint = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * 0.3) as u8);
            painter.image(
                tex.id(),
                viewport_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                tint,
            );
        }
    }

    ui.vertical(|ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("Custom DNS Settings")
                    .color(egui::Color32::WHITE)
                    .size(18.0),
            );
        });
        ui.add_space(SPACING_SMALL);

        let left_margin = 8.0;
        let right_margin = 16.0;
        ui.horizontal(|ui| {
            ui.add_space(left_margin);
            let available_width = ui.available_width() - left_margin - right_margin;
            let frame = egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgba_unmultiplied(60, 60, 65, 45))
                .corner_radius(12.0);
            frame.show(ui, |ui| {
                ui.set_width(available_width);
                ui.vertical(|ui| {
                    ui.add_space(12.0);
                    render_ip_input(ui, custom_primary, "1st DNS ");
                    ui.add_space(5.0);
                    render_ip_input(ui, custom_secondary, "2nd DNS");

                    ui.add_space(3.0);
                    ui.label(
                        egui::RichText::new("Example: 8.8.8.8, 1.1.1.1")
                            .color(egui::Color32::from_rgba_unmultiplied(150, 150, 150, 150))
                            .size(11.0),
                    );

                    ui.add_space(5.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui
                            .add_sized(
                                Vec2::new(70.0, 30.0),
                                egui::Button::new(
                                    egui::RichText::new("Save")
                                        .color(egui::Color32::WHITE)
                                        .size(12.0),
                                )
                                .fill(BUTTON_SUCCESS)
                                .corner_radius(6.0),
                            )
                            .clicked()
                        {
                            on_save();
                        }

                        ui.add_space(5.0);

                        if ui
                            .add_sized(
                                Vec2::new(70.0, 30.0),
                                egui::Button::new(
                                    egui::RichText::new("Clear")
                                        .color(egui::Color32::WHITE)
                                        .size(12.0),
                                )
                                .fill(egui::Color32::from_rgba_unmultiplied(100, 100, 100, 100))
                                .corner_radius(6.0),
                            )
                            .clicked()
                        {
                            on_clear();
                        }
                    });
                    ui.add_space(5.0);
                });
            });
            ui.add_space(right_margin);
        });
    });
}

/// Render add DNS window content (similar to custom DNS but with name field).
pub fn render_add_dns_window_content(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    name: &mut String,
    primary: &mut String,
    secondary: &mut String,
    error_message: Option<String>,
    on_save: impl FnOnce(),
    on_cancel: impl FnOnce(),
) {
    // Draw custom DNS background image with low opacity if available
    if let Some(texture) = ctx.data(|d| {
        d.get_temp::<Option<TextureHandle>>(egui::Id::new("custom_dns_background_texture"))
    }) {
        if let Some(ref tex) = texture {
            let painter = ui.painter();
            let viewport_rect = ui.ctx().viewport_rect();
            let tint = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * 0.3) as u8);
            painter.image(
                tex.id(),
                viewport_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                tint,
            );
        }
    }

    ui.vertical(|ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("Add New DNS Entry")
                    .color(egui::Color32::WHITE)
                    .size(18.0),
            );
        });
        ui.add_space(SPACING_SMALL);

        let left_margin = 8.0;
        let right_margin = 16.0;
        ui.horizontal(|ui| {
            ui.add_space(left_margin);
            let available_width = ui.available_width() - left_margin - right_margin;
            let frame = egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgba_unmultiplied(60, 60, 65, 45))
                .corner_radius(12.0);
            frame.show(ui, |ui| {
                ui.set_width(available_width);
                ui.vertical(|ui| {
                    ui.add_space(12.0);

                    // Name input field
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Name:      ").color(egui::Color32::WHITE));
                        ui.add_sized(
                            Vec2::new(200.0, 20.0),
                            egui::TextEdit::singleline(name).text_color(egui::Color32::WHITE),
                        );
                    });

                    // Show error message under name input if present
                    if let Some(error) = error_message {
                        ui.label(
                            egui::RichText::new(error)
                                .color(egui::Color32::RED)
                                .size(11.0),
                        );
                        ui.add_space(1.0);
                    } else {
                        ui.add_space(5.0);
                    }
                    render_ip_input(ui, primary, "1st DNS ");
                    ui.add_space(5.0);
                    render_ip_input(ui, secondary, "2nd DNS");

                    ui.add_space(3.0);
                    ui.label(
                        egui::RichText::new("Example: 8.8.8.8, 1.1.1.1")
                            .color(egui::Color32::from_rgba_unmultiplied(150, 150, 150, 150))
                            .size(11.0),
                    );

                    ui.add_space(5.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui
                            .add_sized(
                                Vec2::new(70.0, 30.0),
                                egui::Button::new(
                                    egui::RichText::new("Save")
                                        .color(egui::Color32::WHITE)
                                        .size(12.0),
                                )
                                .fill(BUTTON_SUCCESS)
                                .corner_radius(6.0),
                            )
                            .clicked()
                        {
                            on_save();
                        }

                        ui.add_space(5.0);

                        if ui
                            .add_sized(
                                Vec2::new(70.0, 30.0),
                                egui::Button::new(
                                    egui::RichText::new("Cancel")
                                        .color(egui::Color32::WHITE)
                                        .size(12.0),
                                )
                                .fill(egui::Color32::from_rgba_unmultiplied(100, 100, 100, 100))
                                .corner_radius(6.0),
                            )
                            .clicked()
                        {
                            on_cancel();
                        }
                    });
                    ui.add_space(5.0);
                });
            });
            ui.add_space(right_margin);
        });
    });
}
