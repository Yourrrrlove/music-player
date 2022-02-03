use exitfailure::ExitFailure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitConfig {
    pub music_database: String,
    pub theme: InitTheme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitTheme {
    pub list_title_color: String,
    pub list_title_page_color: String,
    pub list_border_color: String,
    pub list_music_color: String,
    pub list_folder_color: String,
    pub list_icon_color: String,
    pub list_selected_color: String,
    pub search_border_color: String,
    pub search_icon_color: String,
    pub info_title_color: String,
    pub info_border_color: String,
    pub music_pic_color1: String,
    pub music_pic_color2: String,
    pub usage_color_left: String,
    pub usage_color_right: String,
}

pub fn init() -> Result<InitConfig, ExitFailure> {
    match dirs::home_dir() {
        Some(home_path) => {
            let config_path = format!(
                "{}\\.config\\music_player\\config.yml",
                home_path.to_str().unwrap()
            );

            let file = std::fs::File::open(config_path)?;
            let init_config: InitConfig = serde_yaml::from_reader(file)?;

            Ok(init_config)
        }
        None => panic!("The path error"),
    }
}
