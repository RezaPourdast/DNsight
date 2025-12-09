//! Storage module for persisting saved DNS entries.

use std::fs;
use std::path::PathBuf;

use crate::domain::SavedDnsEntry;

/// Get the path to the storage file.
fn get_storage_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("dnsight");
    fs::create_dir_all(&path).ok();
    path.push("saved_dns.json");
    path
}

/// Load saved DNS entries from storage.
pub fn load_saved_dns() -> Vec<SavedDnsEntry> {
    let path = get_storage_path();

    if !path.exists() {
        return Vec::new();
    }

    match fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<Vec<SavedDnsEntry>>(&content) {
            Ok(entries) => entries,
            Err(_) => Vec::new(),
        },
        Err(_) => Vec::new(),
    }
}

/// Save DNS entries to storage.
pub fn save_dns_entries(entries: &[SavedDnsEntry]) -> Result<(), String> {
    let path = get_storage_path();

    let json =
        serde_json::to_string_pretty(entries).map_err(|e| format!("Failed to serialize: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

/// Add a new DNS entry to storage.
pub fn add_saved_dns(entry: SavedDnsEntry) -> Result<(), String> {
    let mut entries = load_saved_dns();
    entries.push(entry);
    save_dns_entries(&entries)
}

/// Delete a DNS entry from storage by name.
pub fn delete_saved_dns(name: &str) -> Result<(), String> {
    let mut entries = load_saved_dns();
    entries.retain(|e| e.name != name);
    save_dns_entries(&entries)
}
