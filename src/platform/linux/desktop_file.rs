use std::path::PathBuf;
use freedesktop_entry_parser::{AttrSelector, parse_entry};

pub(crate) struct ApplicationDesktopFile {
    pub name: String,
    pub icon: Option<String>,
    pub keywords: Vec<String>
}

pub(crate) enum DesktopFileError {
    FileNotFound,
    NoDesktopEntry,
    InvalidFormat,
    HiddenFile,
}

impl TryFrom<&PathBuf> for ApplicationDesktopFile {
    type Error = DesktopFileError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let entry = parse_entry(value)
            .map_err(|_| DesktopFileError::InvalidFormat)?;

        let content_section: AttrSelector<&str> = entry.section("Desktop Entry");
        let name = content_section.attr("Name")
            .ok_or(DesktopFileError::NoDesktopEntry)?
            .to_string();

        let icon = content_section.attr("Icon")
            .map(|s| s.to_string());

        let keywords = content_section.attr("Keywords")
            .map(|s| s.split(';').map(|s| s.to_string()).collect())
            .unwrap_or_default();

        let no_display = content_section.attr("NoDisplay")
            .map_or(Ok(false), |s| s.parse::<bool>())
            .map_err(|_| DesktopFileError::InvalidFormat)?;

        if no_display {
            // Hidden files are typically used for window managers and other system utilities
            // so this is not an application we can start
            return Err(DesktopFileError::HiddenFile);
        }

        Ok(ApplicationDesktopFile {
            name,
            icon,
            keywords
        })
    }
}