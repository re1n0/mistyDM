use gtk::gdk::{Key, ModifierType};
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, de};
use serde_inline_default::serde_inline_default;
use std::sync::Arc;

fn deserialize_key<'de, D>(deserializer: D) -> Result<Key, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Key::from_name(&s).ok_or_else(|| de::Error::custom(format!("Unknown key: {}", s)))
}

fn deserialize_modifier<'de, D>(deserializer: D) -> Result<ModifierType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "No" => Ok(ModifierType::NO_MODIFIER_MASK),
        "Ctrl" => Ok(ModifierType::CONTROL_MASK),
        "Alt" => Ok(ModifierType::ALT_MASK),
        other => Err(de::Error::custom(format!("Unknown modifier: {}", other))),
    }
}

#[serde_inline_default]
#[derive(Deserialize)]
pub struct Config {
    #[serde_inline_default(400)]
    pub width: i32,
    #[serde_inline_default(800)]
    pub height: i32,
    #[serde_inline_default(Margins::default())]
    pub margin: Margins,
    #[serde_inline_default(Anchors::default())]
    pub anchor: Anchors,
}

#[serde_inline_default]
#[derive(Deserialize, Default)]
pub struct Margins {
    #[serde_inline_default(0)]
    pub left: i32,
    #[serde_inline_default(0)]
    pub right: i32,
    #[serde_inline_default(0)]
    pub top: i32,
    #[serde_inline_default(0)]
    pub bottom: i32,
}

#[serde_inline_default]
#[derive(Deserialize, Default)]
pub struct Anchors {
    #[serde_inline_default(true)]
    pub left: bool,
    #[serde_inline_default(true)]
    pub right: bool,
    #[serde_inline_default(true)]
    pub top: bool,
    #[serde_inline_default(true)]
    pub bottom: bool,
}

impl Config {
    pub fn load() -> Config {
        let config_file = crate::utils::get_config_file(crate::CONFIG_FILE);

        let config_str = if config_file.exists() {
            std::fs::read_to_string(&config_file).expect("Failed to read the config file")
        } else {
            "".to_owned()
        };
        let config: Config = toml::from_str(&config_str).expect("Cannot parse config option: {}");

        config
    }
}

#[serde_inline_default]
#[derive(Deserialize)]
pub struct Keymap {
    #[serde(deserialize_with = "deserialize_modifier")]
    #[serde_inline_default(ModifierType::ALT_MASK)]
    pub modifier: ModifierType,
    #[serde(deserialize_with = "deserialize_key")]
    #[serde_inline_default(Key::q)]
    pub close: Key,
    #[serde(deserialize_with = "deserialize_key")]
    #[serde_inline_default(Key::k)]
    pub up: Key,
}

impl Keymap {
    pub fn load() -> Keymap {
        let keymap_file = crate::utils::get_config_file(crate::KEYMAP_FILE);

        let keymap_str = if keymap_file.exists() {
            std::fs::read_to_string(&keymap_file).expect("Failed to read the keymap file")
        } else {
            "".to_owned()
        };
        let keymap: Keymap = toml::from_str(&keymap_str).expect("Cannot parse keymap option: {}");

        keymap
    }
}

lazy_static! {
    static ref CONFIG: Arc<Config> = Arc::new(Config::load());
    static ref KEYMAP: Arc<Keymap> = Arc::new(Keymap::load());
}

pub fn get() -> Arc<Config> {
    CONFIG.clone()
}

pub fn keymap_get() -> Arc<Keymap> {
    KEYMAP.clone()
}
