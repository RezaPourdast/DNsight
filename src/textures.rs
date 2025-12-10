use eframe::egui::{self, ColorImage, TextureHandle};
use image;

fn get_exe_dir() -> Option<std::path::PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
}

fn load_texture_from_path(
    ctx: &egui::Context,
    name: &str,
    filename: &str,
) -> Option<TextureHandle> {
    let mut search_paths = Vec::new();
    
    if let Some(exe_dir) = get_exe_dir() {
        search_paths.push(exe_dir.join("asset").join(filename));
        search_paths.push(exe_dir.join("assets").join(filename));
        
        if let Some(parent) = exe_dir.parent() {
            search_paths.push(parent.join("asset").join(filename));
            search_paths.push(parent.join("assets").join(filename));
            
            if let Some(grandparent) = parent.parent() {
                search_paths.push(grandparent.join("asset").join(filename));
                search_paths.push(grandparent.join("assets").join(filename));
            }
        }
    }
    
    if let Ok(cwd) = std::env::current_dir() {
        search_paths.push(cwd.join("asset").join(filename));
        search_paths.push(cwd.join("assets").join(filename));
    }
    
    search_paths.push(std::path::PathBuf::from(format!("asset/{}", filename)));
    search_paths.push(std::path::PathBuf::from(format!("assets/{}", filename)));

    for base_path in search_paths {
        let paths = vec![
            base_path.clone(),
            base_path.with_extension("jpg"),
            base_path.with_extension("jpeg"),
            base_path.with_extension("webp"),
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
    }
    
    None
}

pub fn load_background_image(ctx: &egui::Context) -> Option<TextureHandle> {
    load_texture_from_path(ctx, "background", "main-background.png")
}

pub fn load_ping_background_image(ctx: &egui::Context) -> Option<TextureHandle> {
    load_texture_from_path(ctx, "ping_background", "ping-background.png")
}

pub fn load_custom_dns_background_image(ctx: &egui::Context) -> Option<TextureHandle> {
    load_texture_from_path(ctx, "custom_dns_background", "custom-dns-bg.png")
}

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
