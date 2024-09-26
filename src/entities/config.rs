use iced_anim::{Spring, SpringEvent};
use serde::{Deserialize, Serialize};

use crate::entities::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub theme: Spring<Theme>,
    #[serde(default = "Config::default_path")]
    pub directory: String,
}

#[derive(Debug)]
pub struct ConfigureWindow {
    pub config: Config,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum ConfigEvent {
    UpdateFolderPath,
    OpenFolder,
    UpdateTheme(SpringEvent<Theme>),
    RequestExit,
}
