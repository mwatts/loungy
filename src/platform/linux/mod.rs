mod desktop_file;

use crate::components::shared::{Icon, Img};
use crate::paths::paths;

use std::fs;
use std::path::PathBuf;

use super::AppData;

pub fn get_app_data(path: &PathBuf) -> Option<AppData> {
    let cache_dir = paths().cache.join("apps");
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir.clone()).unwrap();
    }
    let cache = cache_dir.to_string_lossy().to_string();
    let file_name = path
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .to_string();

    let file = desktop_file::ApplicationDesktopFile::try_from(path).ok()?;
    let icon_url: Option<PathBuf> = file.resolve_icon();

    let icon_img = if let Some(icon) = icon_url.clone() {
        if (icon.extension().unwrap_or_default().eq("svg")) {
            // TODO: Add support for SVG icons
            Img::list_icon(Icon::AppWindow, None)
        } else {
            Img::list_file(icon)
        }
    } else {
        Img::list_icon(Icon::AppWindow, None)
    };

    Some(AppData {
        id: file_name.clone(),
        name: file.name.clone(),
        icon: icon_img,
        icon_path: icon_url.unwrap_or_else(|| PathBuf::new()),
        keywords: file.keywords,
        tag: "Application".to_string(),
    })
}
