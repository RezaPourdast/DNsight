use ping;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, TextureHandle, Vec2};

use crate::domain::{
    AppState, DnsOperation, DnsProvider, DnsState, OperationResult, SavedDnsEntry,
};
use crate::storage::{add_saved_dns, delete_saved_dns, load_saved_dns};
use crate::system::{
    clear_dns_with_result, get_active_adapter, get_current_dns, set_dns_with_result,
};
use crate::textures::{
    load_background_image, load_custom_dns_background_image, load_ping_background_image,
    load_social_logos,
};
use crate::ui::{
    self, render_action_buttons, render_add_dns_window_content, render_app_state,
    render_custom_dns_window_content, render_footer, render_ping_window_content,
    render_provider_selection, render_status_section, ui_colors, ui_constants,
};
use crate::windows::{custom_window_frame, simple_window_frame};

#[derive(Default)]
pub struct MyApp {
    adapter: Option<String>,
    dns: Vec<String>,
    app_state: AppState,
    selected_provider: DnsProvider,
    dns_state: DnsState,
    custom_primary: String,
    custom_secondary: String,
    operation_sender: Option<mpsc::Sender<OperationResult>>,
    operation_receiver: Option<mpsc::Receiver<OperationResult>>,
    show_second_window: bool,
    ping_value: f64,
    ping_history: VecDeque<f64>,
    ping_sender: Option<mpsc::Sender<f64>>,
    ping_receiver: Option<mpsc::Receiver<f64>>,
    show_clear_confirmation: bool,
    show_custom_dns_window: bool,
    show_add_dns_window: bool,
    saved_dns_entries: Vec<SavedDnsEntry>,
    new_dns_name: String,
    new_dns_primary: String,
    new_dns_secondary: String,
    add_dns_error: Option<String>,
    background_texture: Option<TextureHandle>,
    ping_background_texture: Option<TextureHandle>,
    custom_dns_background_texture: Option<TextureHandle>,
    social_logos: std::collections::HashMap<String, TextureHandle>,
}

static PING_REQUEST: AtomicBool = AtomicBool::new(false);

impl MyApp {
    pub fn new() -> Self {
        let saved_dns_entries = load_saved_dns();

        let app = Self {
            dns_state: DnsState::None,
            ping_value: 0.0,
            ping_history: VecDeque::with_capacity(15),
            ping_sender: None,
            ping_receiver: None,
            background_texture: None,
            ping_background_texture: None,
            custom_dns_background_texture: None,
            social_logos: std::collections::HashMap::new(),
            saved_dns_entries,
            ..Default::default()
        };

        app
    }

    fn handle_operation(&mut self, operation: DnsOperation) {
        self.app_state = AppState::Processing;

        let adapter = get_active_adapter();
        self.adapter = adapter.clone();

        let (sender, receiver) = mpsc::channel();
        self.operation_sender = Some(sender);
        self.operation_receiver = Some(receiver);

        let adapter_for_thread = adapter;
        let sender_clone = self.operation_sender.clone();

        thread::spawn(move || {
            let result = match operation {
                DnsOperation::Set(provider) => {
                    if let Some(adapter) = &adapter_for_thread {
                        let (primary, secondary) = provider.get_servers();
                        set_dns_with_result(adapter, &primary, &secondary)
                    } else {
                        OperationResult::Error("No Internet Connection Found".to_string())
                    }
                }
                DnsOperation::Clear => {
                    if let Some(adapter) = &adapter_for_thread {
                        clear_dns_with_result(adapter)
                    } else {
                        OperationResult::Error("No Internet Connection Found".to_string())
                    }
                }
                DnsOperation::Test => {
                    if let Some(adapter) = &adapter_for_thread {
                        let dns = get_current_dns(adapter);
                        if dns.is_empty() {
                            OperationResult::Warning("No DNS servers configured".to_string())
                        } else {
                            OperationResult::Success(format!(
                                "DNS test successful: {}",
                                dns.join(", ")
                            ))
                        }
                    } else {
                        OperationResult::Error("No Internet Connection Found".to_string())
                    }
                }
            };

            if let Some(s) = sender_clone {
                let _ = s.send(result);
            }
        });
    }

    fn handle_operation_result(&mut self, result: OperationResult) {
        match result {
            OperationResult::Success(message) => {
                self.app_state = AppState::Success(message);
                if let Some(adapter) = &self.adapter {
                    self.dns = get_current_dns(adapter);
                    self.update_dns_state();
                }
            }
            OperationResult::Error(message) => {
                self.app_state = AppState::Error(message);
            }
            OperationResult::Warning(message) => {
                self.app_state = AppState::Warning(message);
            }
        }
    }

    fn update_dns_state(&mut self) {
        if self.dns.is_empty() {
            self.dns_state = DnsState::None;
        } else if self.dns.len() == 1 && self.dns[0].contains("dhcp") {
            self.dns_state = DnsState::Dhcp;
        } else {
            self.dns_state = DnsState::Static(self.dns.clone());
        }
    }

    fn render_secondary_viewport(&mut self, ctx: &egui::Context) {
        if !self.show_second_window {
            return;
        }

        let ping_value = self.ping_value;
        let ping_history: Vec<f64> = self.ping_history.iter().copied().collect();

        let keep_open = std::cell::Cell::new(true);
        let window_size = egui::vec2(400.0, 300.0);
        let screen_center = ctx.input(|i| {
            let info = i.viewport();
            info.outer_rect
                .or(info.inner_rect)
                .map(|rect| rect.center())
                .unwrap_or_else(|| egui::pos2(0.0, 0.0))
        });
        let position = screen_center - window_size / 2.0;
        let viewport_id = egui::ViewportId::from_hash_of("ping");

        ctx.show_viewport_immediate(
            viewport_id,
            egui::ViewportBuilder::default()
                .with_title("Ping-Monitor")
                .with_inner_size(window_size)
                .with_position(position)
                .with_resizable(true)
                .with_decorations(false),
            {
                let keep_open = &keep_open;
                move |ctx, _class| {
                    if ctx.input(|i| i.viewport().close_requested()) {
                        keep_open.set(false);
                    }

                    simple_window_frame(ctx, |ui| {
                        render_ping_window_content(ui, ctx, ping_value, &ping_history);
                    });
                }
            },
        );

        self.show_second_window = keep_open.get();
        if !self.show_second_window {
            let _ = self.ping_sender.take();
            self.ping_receiver = None;
            self.ping_value = 0.0;
            self.ping_history.clear();
        }
    }

    fn render_custom_dns_window(&mut self, ctx: &egui::Context) {
        if !self.show_custom_dns_window {
            return;
        }

        let keep_open = std::cell::Cell::new(true);
        let window_size = egui::vec2(300.0, 250.0);
        let screen_center = ctx.input(|i| {
            let info = i.viewport();
            info.outer_rect
                .or(info.inner_rect)
                .map(|rect| rect.center())
                .unwrap_or_else(|| egui::pos2(0.0, 0.0))
        });
        let position = screen_center - window_size / 2.0;
        let viewport_id = egui::ViewportId::from_hash_of("custom_dns");

        ctx.show_viewport_immediate(
            viewport_id,
            egui::ViewportBuilder::default()
                .with_title("Custom DNS Settings")
                .with_inner_size(window_size)
                .with_position(position)
                .with_resizable(false)
                .with_decorations(false),
            {
                let keep_open = &keep_open;
                let custom_primary = &mut self.custom_primary;
                let custom_secondary = &mut self.custom_secondary;
                let clear_requested = std::cell::Cell::new(false);

                move |ctx, _class| {
                    if ctx.input(|i| i.viewport().close_requested()) {
                        keep_open.set(false);
                    }

                    simple_window_frame(ctx, |ui| {
                        render_custom_dns_window_content(
                            ui,
                            ctx,
                            custom_primary,
                            custom_secondary,
                            || {
                                keep_open.set(false);
                            },
                            || {
                                clear_requested.set(true);
                            },
                        );
                    });

                    if clear_requested.get() {
                        *custom_primary = String::new();
                        *custom_secondary = String::new();
                    }
                }
            },
        );

        self.show_custom_dns_window = keep_open.get();

        if matches!(self.selected_provider, DnsProvider::Custom { .. }) {
            self.selected_provider =
                DnsProvider::custom(self.custom_primary.clone(), self.custom_secondary.clone());
        }
    }

    fn render_add_dns_window(&mut self, ctx: &egui::Context) {
        if !self.show_add_dns_window {
            return;
        }

        let keep_open = std::cell::Cell::new(true);
        let window_size = egui::vec2(300.0, 300.0);
        let screen_center = ctx.input(|i| {
            let info = i.viewport();
            info.outer_rect
                .or(info.inner_rect)
                .map(|rect| rect.center())
                .unwrap_or_else(|| egui::pos2(0.0, 0.0))
        });
        let position = screen_center - window_size / 2.0;
        let viewport_id = egui::ViewportId::from_hash_of("add_dns");

        let save_requested = std::cell::Cell::new(false);
        let should_close = std::cell::Cell::new(false);

        ctx.show_viewport_immediate(
            viewport_id,
            egui::ViewportBuilder::default()
                .with_title("Add New DNS Entry")
                .with_inner_size(window_size)
                .with_position(position)
                .with_resizable(false)
                .with_decorations(false),
            {
                let keep_open = &keep_open;
                let name = &mut self.new_dns_name;
                let primary = &mut self.new_dns_primary;
                let secondary = &mut self.new_dns_secondary;
                let save_requested = &save_requested;
                let should_close = &should_close;
                let add_dns_error = &self.add_dns_error;

                move |ctx, _class| {
                    if ctx.input(|i| i.viewport().close_requested()) {
                        keep_open.set(false);
                    }

                    simple_window_frame(ctx, |ui| {
                        render_add_dns_window_content(
                            ui,
                            ctx,
                            name,
                            primary,
                            secondary,
                            add_dns_error.clone(),
                            || {
                                save_requested.set(true);
                            },
                            || {
                                keep_open.set(false);
                            },
                        );
                    });

                    if should_close.get() {
                        keep_open.set(false);
                    }
                }
            },
        );

        if save_requested.get() {
            let name_valid = !self.new_dns_name.trim().is_empty();
            let primary_valid = !self.new_dns_primary.trim().is_empty()
                && crate::ui::is_valid_ip(&self.new_dns_primary);
            let secondary_valid = !self.new_dns_secondary.trim().is_empty()
                && crate::ui::is_valid_ip(&self.new_dns_secondary);

            let name_trimmed = self.new_dns_name.trim();
            let name_exists = self
                .saved_dns_entries
                .iter()
                .any(|e| e.name.trim().eq_ignore_ascii_case(name_trimmed));

            if name_exists {
                self.add_dns_error = Some(format!(
                    "A DNS entry with the name '{}' already exists",
                    name_trimmed
                ));
            } else if name_valid && primary_valid && secondary_valid {
                self.add_dns_error = None;
                let entry = SavedDnsEntry {
                    name: name_trimmed.to_string(),
                    primary: self.new_dns_primary.trim().to_string(),
                    secondary: self.new_dns_secondary.trim().to_string(),
                };

                if let Err(e) = add_saved_dns(entry.clone()) {
                    self.add_dns_error = Some(format!("Failed to save DNS: {}", e));
                } else {
                    self.saved_dns_entries.push(entry.clone());
                    self.selected_provider = DnsProvider::saved(
                        entry.name.clone(),
                        entry.primary.clone(),
                        entry.secondary.clone(),
                    );
                    self.app_state = AppState::Success("DNS saved successfully!".to_string());
                    self.new_dns_name.clear();
                    self.new_dns_primary.clear();
                    self.new_dns_secondary.clear();
                    should_close.set(true);
                }
            } else {
                self.add_dns_error =
                    Some("Please enter a valid name and DNS IP addresses".to_string());
            }
        }

        if should_close.get() {
            self.show_add_dns_window = false;
        } else {
            self.show_add_dns_window = keep_open.get();
        }

        if !self.show_add_dns_window {
            self.new_dns_name.clear();
            self.new_dns_primary.clear();
            self.new_dns_secondary.clear();
            self.add_dns_error = None;
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::configure_theme(ctx);

        if self.background_texture.is_none() {
            self.background_texture = load_background_image(ctx);
        }
        if self.ping_background_texture.is_none() {
            self.ping_background_texture = load_ping_background_image(ctx);
        }
        if self.custom_dns_background_texture.is_none() {
            self.custom_dns_background_texture = load_custom_dns_background_image(ctx);
        }
        if self.social_logos.is_empty() {
            self.social_logos = load_social_logos(ctx);
        }

        if let Some(ref texture) = self.background_texture {
            ctx.data_mut(|d| {
                d.insert_temp(egui::Id::new("background_texture"), Some(texture.clone()));
            });
        }
        if let Some(ref texture) = self.ping_background_texture {
            ctx.data_mut(|d| {
                d.insert_temp(
                    egui::Id::new("ping_background_texture"),
                    Some(texture.clone()),
                );
            });
        }
        if let Some(ref texture) = self.custom_dns_background_texture {
            ctx.data_mut(|d| {
                d.insert_temp(
                    egui::Id::new("custom_dns_background_texture"),
                    Some(texture.clone()),
                );
            });
        }

        if let Some(receiver) = &self.operation_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.handle_operation_result(result);
                self.operation_receiver = None;
                self.operation_sender = None;
                ctx.request_repaint();
            } else if matches!(self.app_state, AppState::Processing) {
                ctx.request_repaint();
            }
        }

        if let Some(ping_rx) = &self.ping_receiver {
            if let Ok(ping) = ping_rx.try_recv() {
                self.ping_value = ping;
                if self.ping_history.len() >= 15 {
                    self.ping_history.pop_front();
                }
                self.ping_history.push_back(ping);
                ctx.request_repaint();
            }
        }

        custom_window_frame(
            ctx,
            "",
            |ui| {
                use ui_constants::*;

                ui.horizontal(|ui| {
                    ui.set_max_width(230.0);
                    ui.set_max_height(165.0);
                    let frame = egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgba_unmultiplied(60, 60, 65, 45))
                        .corner_radius(12.0);
                    frame.show(ui, |ui| {
                        ui.set_width(225.0);
                        ui.set_height(165.0);
                        ui.add_space(12.0);
                        let dns_state = self.dns_state.clone();
                        ui.vertical(|ui| {
                            ui.add_space(12.0);
                            let mut test_dns = false;
                            render_status_section(ui, &dns_state, &self.app_state, || {
                                test_dns = true;
                            });
                            if test_dns {
                                self.handle_operation(DnsOperation::Test);
                            }
                            render_app_state(ui, &self.app_state);
                        });
                        if ui
                            .ctx()
                            .input(|i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl)
                        {
                            self.handle_operation(DnsOperation::Test);
                        }
                    });
                });

                ui.horizontal(|ui| {
                    ui.set_max_width(230.0);
                    let frame = egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgba_unmultiplied(60, 60, 65, 45))
                        .corner_radius(12.0);
                    frame.show(ui, |ui| {
                        ui.set_width(225.0);
                        ui.vertical(|ui| {
                            ui.add_space(12.0);
                            ui.horizontal(|ui| {
                                ui.add_space(13.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new("DNS List")
                                            .color(egui::Color32::WHITE)
                                            .size(18.0),
                                    );
                                    let selected_provider = self.selected_provider.clone();
                                    let custom_primary = self.custom_primary.clone();
                                    let custom_secondary = self.custom_secondary.clone();
                                    let mut provider_changed = None;
                                    let mut open_custom = false;

                                    let saved_entries = self.saved_dns_entries.clone();
                                    let mut open_add_new = false;
                                    render_provider_selection(
                                        ui,
                                        &selected_provider,
                                        &custom_primary,
                                        &custom_secondary,
                                        &saved_entries,
                                        |provider| {
                                            provider_changed = Some(provider);
                                        },
                                        || {
                                            open_custom = true;
                                        },
                                        || {
                                            open_add_new = true;
                                        },
                                    );

                                    if open_add_new {
                                        self.show_add_dns_window = true;
                                    }

                                    if let Some(provider) = provider_changed {
                                        self.selected_provider = provider;
                                    }
                                    if open_custom {
                                        self.show_custom_dns_window = true;
                                    }
                                });
                            });
                            ui.add_space(BUTTON_SPACING);
                            let selected_provider = self.selected_provider.clone();
                            let mut set_dns = false;
                            let mut clear_dns = false;
                            let delete_entry_name = std::cell::Cell::new(None::<String>);

                            let provider_name = selected_provider.display_name();

                            let delete_callback =
                                if let DnsProvider::Saved { name, .. } = &selected_provider {
                                    let entry_name = name.clone();
                                    let delete_entry_name = &delete_entry_name;
                                    Some(move || {
                                        delete_entry_name.set(Some(entry_name));
                                    })
                                } else {
                                    None
                                };

                            render_action_buttons(
                                ui,
                                &provider_name,
                                || {
                                    set_dns = true;
                                },
                                || {
                                    clear_dns = true;
                                },
                                delete_callback,
                            );

                            if let Some(name) = delete_entry_name.take() {
                                if let Err(e) = delete_saved_dns(&name) {
                                    self.app_state =
                                        AppState::Error(format!("Failed to delete DNS: {}", e));
                                } else {
                                    self.saved_dns_entries.retain(|e| e.name != name);
                                    if let DnsProvider::Saved {
                                        name: selected_name,
                                        ..
                                    } = &self.selected_provider
                                    {
                                        if selected_name == &name {
                                            self.selected_provider = DnsProvider::electro();
                                        }
                                    }
                                    self.app_state =
                                        AppState::Success("DNS entry deleted".to_string());
                                }
                            }

                            if set_dns {
                                self.handle_operation(DnsOperation::Set(
                                    self.selected_provider.clone(),
                                ));
                            }
                            if clear_dns {
                                self.show_clear_confirmation = true;
                            }
                        });
                        });
                    });

                let has_delete_button = matches!(self.selected_provider, DnsProvider::Saved { .. });
                render_footer(ui, &self.social_logos, has_delete_button);
            },
            || {
                PING_REQUEST.store(true, Ordering::SeqCst);
            },
        );

        if PING_REQUEST.swap(false, Ordering::SeqCst) {
            if self.ping_sender.is_none() {
                let (tx, rx) = mpsc::channel::<f64>();
                self.ping_sender = Some(tx.clone());
                self.ping_receiver = Some(rx);

                thread::spawn(move || {
                    loop {
                        let value = get_ping();
                        if tx.send(value).is_err() {
                            break;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }
            self.show_second_window = true;
        }

        self.render_secondary_viewport(ctx);
        self.render_custom_dns_window(ctx);
        self.render_add_dns_window(ctx);

        if self.show_clear_confirmation {
            use ui_colors::{BUTTON_SUCCESS, BUTTON_TEXT};

            egui::Window::new("Confirm Clear DNS")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new(
                            "Are you sure you want to clear the DNS configuration?",
                        )
                        .color(egui::Color32::WHITE),
                    );
                    ui.label(
                        egui::RichText::new("This will reset DNS to DHCP/automatic.")
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(10.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        ui.add_space(10.0);

                        if ui
                            .add_sized(
                                Vec2::new(80.0, 30.0),
                                egui::Button::new(egui::RichText::new("Cancel").color(BUTTON_TEXT))
                                    .fill(egui::Color32::from_rgba_unmultiplied(100, 100, 100, 100))
                                    .corner_radius(6),
                            )
                            .clicked()
                        {
                            self.show_clear_confirmation = false;
                        }

                        ui.add_space(3.0);

                        if ui
                            .add_sized(
                                Vec2::new(80.0, 30.0),
                                egui::Button::new(
                                    egui::RichText::new("Clear DNS").color(BUTTON_TEXT),
                                )
                                .fill(BUTTON_SUCCESS)
                                .corner_radius(6),
                            )
                            .clicked()
                        {
                            self.show_clear_confirmation = false;
                            self.handle_operation(DnsOperation::Clear);
                        }
                    });
                });
        }

        ctx.request_repaint_after(Duration::from_millis(1000));
    }
}

fn get_ping() -> f64 {
    let target_ip = match "8.8.8.8".parse::<std::net::IpAddr>() {
        Ok(ip) => ip,
        Err(_) => return 0.0,
    };

    let mut p = ping::new(target_ip);
    p.timeout(Duration::from_secs(1)).ttl(128);

    let start = Instant::now();
    match p.send() {
        Ok(_) => start.elapsed().as_millis() as f64,
        Err(_) => 0.0,
    }
}
