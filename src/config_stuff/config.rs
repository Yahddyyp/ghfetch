use crate::config_stuff::colors::ColorsConfig;
use crate::config_stuff::default::write_default_config;
use crate::config_stuff::fields::Field;
use crate::config_stuff::image::ImageConfig;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub fields: Option<Vec<Field>>,
    #[serde(default)]
    pub colors: ColorsConfig,
    #[serde(default)]
    pub image: ImageConfig,
}

/// Load the config from file.
pub fn load_config() -> Result<Config> {
    let path = match dirs::home_dir() {
        Some(home) => home.join(".config").join("ghfetch").join("config.toml"),
        None => anyhow::bail!("could not find home directory"),
    };

    if !path.exists() {
        write_default_config(&path)?;
    }

    // Show Error if the config fails to read or parse
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let config = toml::from_str::<Config>(&contents)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    Ok(config)
}
