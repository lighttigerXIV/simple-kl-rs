
use crate::paths::{get_settings_path, get_autostart_path, get_local_dir, get_resources_directory};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{fs, env};
use std::fs::File;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_general_settings")]
    pub general: GeneralSettings,
    #[serde(default = "default_search_settings")]
    pub search: SearchSettings,
    #[serde(default = "default_results_settings")]
    pub results: ResultsSettings,
    #[serde(default = "default_theme_settings")]
    pub theme: ThemeSettings,
    #[serde(default = "default_search_engines")]
    pub search_engines: Vec<SearchEngineSettings>,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<ExtensionsSettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralSettings {
    #[serde(default = "default_general_first_key")]
    pub first_key: String,
    #[serde(default = "default_general_second_key")]
    pub second_key: String,
    #[serde(default = "default_general_third_key")]
    pub third_key: String,
    #[serde(default = "default_general_auto_start")]
    pub auto_start: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchSettings {
    #[serde(default = "default_search_show_settings_icon")]
    pub show_settings_icon: bool,
    #[serde(default = "default_search_show_search_icon")]
    pub show_search_icon: bool,
    #[serde(default = "default_search_show_settings_icon")]
    pub show_placeholder: bool,
    #[serde(default = "default_search_border_radius")]
    pub border_radius: usize,
    #[serde(default = "default_search_border_width")]
    pub border_width: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultsSettings {
    #[serde(default = "default_results_count")]
    pub results_count: usize,
    #[serde(default = "default_results_split_ui")]
    pub split_ui: bool,
    #[serde(default = "default_results_layout")]
    pub layout: LayoutSetting,
    #[serde(default = "default_results_blacklist")]
    pub blacklist: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LayoutSetting {
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeSettings {
    #[serde(default = "default_theme_background")]
    pub background: String,
    #[serde(default = "default_theme_secondary_background")]
    pub secondary_background: String,
    #[serde(default = "default_theme_tertiary_background")]
    pub tertiary_background: String,
    #[serde(default = "default_theme_accent")]
    pub accent: String,
    #[serde(default = "default_theme_on_accent")]
    pub on_accent: String,
    #[serde(default = "default_theme_danger")]
    pub danger: String,
    #[serde(default = "default_theme_on_danger")]
    pub on_danger: String,
    #[serde(default = "default_theme_text")]
    pub text: String,
    #[serde(default = "default_theme_secondary_text")]
    pub secondary_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEngineSettings {
    pub keyword: String,
    pub icon: Option<String>,
    #[serde(default = "default_search_engine_tint_icon")]
    pub tint_icon: bool,
    pub name: String,
    pub query: String,
    #[serde(default = "default_search_engine_default")]
    pub default: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionsSettings {
    pub id: String,
    pub keyword: String,
    #[serde(default = "default_extension_setting")]
    pub settings: ExtensionSetting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionSetting {
    #[serde(default = "default_extension_option_setting")]
    pub any: Vec<ExtensionOptionSetting>,
    #[serde(default = "default_extension_option_setting")]
    pub linux: Vec<ExtensionOptionSetting>,
    #[serde(default = "default_extension_option_setting")]
    pub windows: Vec<ExtensionOptionSetting>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionOptionSetting {
    pub id: String,
    pub current_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Setting {
    GeneralFirstKey,
    GeneralSecondKey,
    GeneralThirdKey,
    GeneralAutoStart,
    SearchShowSettingsIcon,
    SearchShowSearchIcon,
    SearchShowPlaceholder,
    SearchBorderRadius,
    SearchBorderWidth,
    ResultsCount,
    ResultsSplitUI,
    ResultsLayout,
    ThemeBackground,
    ThemeSecondaryBackground,
    ThemeTertiaryBackground,
    ThemeAccent,
    ThemeOnAccent,
    ThemeDanger,
    ThemeOnDanger,
    ThemeText,
    ThemeSecondaryText,
    SearchEngines,
    Extensions,
}

//Default Settings
fn default_general_settings() -> GeneralSettings {
    return GeneralSettings {
        first_key: default_general_first_key(),
        second_key: default_general_second_key(),
        third_key: default_general_third_key(),
        auto_start: default_general_auto_start(),
    };
}

fn default_general_first_key() -> String {
    return "ctrl".to_owned();
}

fn default_general_second_key() -> String {
    return "-".to_owned();
}

fn default_general_third_key() -> String {
    return "space".to_owned();
}

fn default_general_auto_start() -> bool {
    return true;
}

fn default_search_settings() -> SearchSettings {
    return SearchSettings {
        show_settings_icon: default_search_show_settings_icon(),
        show_search_icon: default_search_show_search_icon(),
        show_placeholder: default_search_show_placeholder(),
        border_radius: default_search_border_radius(),
        border_width: default_search_border_width(),
    };
}

fn default_search_show_settings_icon() -> bool {
    return true;
}

fn default_search_show_search_icon() -> bool {
    return true;
}

fn default_search_show_placeholder() -> bool {
    return true;
}

fn default_search_border_radius() -> usize {
    return 20;
}

fn default_search_border_width() -> usize {
    return 1;
}

fn default_results_settings() -> ResultsSettings {
    return ResultsSettings {
        results_count: default_results_count(),
        split_ui: default_results_split_ui(),
        layout: default_results_layout(),
        blacklist: default_results_blacklist(),
    };
}

fn default_results_count() -> usize {
    return 6;
}

fn default_results_split_ui() -> bool {
    return false;
}

fn default_results_layout() -> LayoutSetting {
    return match env::consts::OS {
        "linux" => LayoutSetting::Medium,
        _=> LayoutSetting::Small
    };
}

fn default_results_blacklist() -> Vec<String> {
    return vec![];
}

fn default_theme_settings() -> ThemeSettings {
    return ThemeSettings {
        background: default_theme_background(),
        secondary_background: default_theme_secondary_background(),
        tertiary_background: default_theme_tertiary_background(),
        accent: default_theme_accent(),
        on_accent: default_theme_on_accent(),
        danger: default_theme_danger(),
        on_danger: default_theme_on_danger(),
        text: default_theme_text(),
        secondary_text: default_theme_secondary_text(),
    };
}

fn default_theme_background() -> String {
    return "#24273a".to_owned();
}

fn default_theme_secondary_background() -> String {
    return "#181926".to_owned();
}

fn default_theme_tertiary_background() -> String {
    return "#1e2030".to_owned();
}

fn default_theme_accent() -> String {
    return "#8aadf4".to_owned();
}

fn default_theme_on_accent() -> String {
    return "#181926".to_owned();
}

fn default_theme_danger() -> String {
    return "#ed8796".to_owned();
}

fn default_theme_on_danger() -> String {
    return "#181926".to_owned();
}

fn default_theme_text() -> String {
    return "#cad3f5".to_owned();
}

fn default_theme_secondary_text() -> String {
    return "#b8c0e0".to_owned();
}

fn default_search_engines() -> Vec<SearchEngineSettings> {

    let mut search_engines: Vec<SearchEngineSettings> = vec![];
    let mut icons_dir = get_resources_directory().unwrap();
    icons_dir.push("icons");

    let mut google_icon = icons_dir.to_owned();
    google_icon.push("google.svg");

    let mut duckduckgo_icon = icons_dir.to_owned();
    duckduckgo_icon.push("duckduckgo.svg");

    let mut brave_icon = icons_dir.to_owned();
    brave_icon.push("brave.svg");

    search_engines.push(SearchEngineSettings{
        keyword: String::from("gg"),
        icon: Some(google_icon.into_os_string().into_string().unwrap()),
        tint_icon: true,
        name: String::from("Google"),
        query: String::from("https://www.google.com/search?q=%s"),
        default: true
    });

    search_engines.push(SearchEngineSettings{
        keyword: String::from("dd"),
        icon: Some(duckduckgo_icon.into_os_string().into_string().unwrap()),
        tint_icon: true,
        name: String::from("DuckDuckGo"),
        query: String::from("https://duckduckgo.com/?q=%s"),
        default: false
    });

    search_engines.push(SearchEngineSettings{
        keyword: String::from("bs"),
        icon: Some(brave_icon.into_os_string().into_string().unwrap()),
        tint_icon: true,
        name: String::from("Brave Search"),
        query: String::from("https://search.brave.com/search?q=%s"),
        default: false
    });

    return search_engines;
}

fn default_search_engine_tint_icon() -> bool {
    return false;
}

fn default_search_engine_default() -> bool {
    return false;
}

fn default_extensions() -> Vec<ExtensionsSettings> {
    return vec![];
}

fn default_extension_setting() -> ExtensionSetting {
    return ExtensionSetting {
        any: vec![],
        linux: vec![],
        windows: vec![],
    };
}

fn default_extension_option_setting() -> Vec<ExtensionOptionSetting> {
    return vec![];
}

//Functions

/// Returns the app settings
pub fn get_settings() -> Settings {
    let settings_yaml = fs::read_to_string(&get_settings_path().unwrap()).unwrap();

    let settings: Settings = serde_yaml::from_str(&settings_yaml).unwrap();

    return settings;
}

/// Creates a settings file if it doesn't exist
pub fn init_settings() {
    let settings_file_path = get_settings_path().unwrap();
    let settings_folder = settings_file_path.parent().unwrap();

    if !settings_folder.exists() {
        fs::create_dir_all(&settings_folder).expect("Error creating settings folder");
    }

    if !&settings_file_path.exists() {
        File::create(&settings_file_path).expect("Error creating settings file");
        let settings_yaml = serde_yaml::to_string(&get_settings()).unwrap();
        fs::write(&settings_file_path, &settings_yaml).expect("Error writing settings file");
    }
}

/// Updates the app settings
pub fn update_settings(settings: &Settings) {
    let settings_yaml = serde_yaml::to_string(settings).unwrap();
    fs::write(&get_settings_path().unwrap(), &settings_yaml).expect("Error writing settings file");
}


/// Creates/Removes a shortcut to the autostart folder
pub fn update_auto_start() {
    let path = get_autostart_path().unwrap();
    let settings = get_settings();
    let auto_start = settings.general.auto_start;

    if !path.exists() && auto_start {
        fs::create_dir_all(&path).expect("Error creating autostart folder");
    }

    match env::consts::OS {
        "linux" => {
            let desktop_file_content = include_str!("../assets/files/simple-kl-service.desktop");
            let mut desktop_file_path = path.to_owned();
            desktop_file_path.push("simple-kl-service.desktop");

            if auto_start {
                fs::write(&desktop_file_path, &desktop_file_content)
                    .expect("Error creating autostart file");
            } else {
                if desktop_file_path.exists() {
                    fs::remove_file(&desktop_file_path).expect("Error removing autostart file");
                }
            }
        }
        #[cfg(target_os = "windows")]
        "windows" => {
            let script = if auto_start { "enable-autostart.ps1" } else { "disable-autostart.ps1" };

            let mut path = get_local_dir().unwrap();
            path.push("resources\\ps-scripts");
            path.push(script);

            let script_content = fs::read_to_string(&path).unwrap();
            powershell_script::run(&script_content).expect("Error running autostart script");
        }
        _ => {}
    }
}