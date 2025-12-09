//! Texture loading and management for background images and logos.

use eframe::egui::{self, ColorImage, TextureHandle};
use image;

/// Load a texture from an image file, trying multiple formats and paths.
fn load_texture_from_path(
    ctx: &egui::Context,
    name: &str,
    filename: &str,
) -> Option<TextureHandle> {
    let image_path = if let Ok(dir) = std::env::current_dir() {
        dir.join("asset").join(filename)
    } else {
        std::path::PathBuf::from(format!("asset/{}", filename))
    };

    // Try multiple formats: PNG, JPG, JPEG, WEBP
    let paths = vec![
        image_path.clone(),
        image_path.with_extension("jpg"),
        image_path.with_extension("jpeg"),
        image_path.with_extension("webp"),
    ];

    for path in paths {
        if path.exists() {
            if let Ok(img) = image::open(&path) {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let pixels = rgba.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                let texture = ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR);
                return Some(texture);
            }
        }
    }
    None
}

/// Load the main background image.
pub fn load_background_image(ctx: &egui::Context) -> Option<TextureHandle> {
    load_texture_from_path(ctx, "background", "main-background.png")
}

/// Load the ping window background image.
pub fn load_ping_background_image(ctx: &egui::Context) -> Option<TextureHandle> {
    load_texture_from_path(ctx, "ping_background", "ping-background.png")
}

/// Load the custom DNS window background image.
pub fn load_custom_dns_background_image(ctx: &egui::Context) -> Option<TextureHandle> {
    load_texture_from_path(ctx, "custom_dns_background", "custom-dns-bg.png")
}

/// Load all social media logos.
pub fn load_social_logos(ctx: &egui::Context) -> std::collections::HashMap<String, TextureHandle> {
    let mut logos = std::collections::HashMap::new();
    let logo_files = vec![
        ("cup-of-drink", "cup-of-drink.png"),
        ("email", "email.png"),
        ("github", "github.png"),
    ];

    for (name, filename) in logo_files {
        if let Some(texture) = load_texture_from_path(ctx, &format!("logo_{}", name), filename) {
            logos.insert(name.to_string(), texture);
        }
    }

    logos
}
