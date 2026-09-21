use crate::config_stuff::config::Config;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct ImageConfig {
    // Size of the image
    pub image_columns: Option<usize>,
    pub image_rows: Option<usize>,
    // Gaps from the edges of the terminal
    pub right_gap: Option<usize>,
    pub left_gap: Option<usize>,
}

pub struct Image {
    pub image_columns: usize,
    pub image_rows: usize,
    pub right_gap: usize,
    pub left_gap: usize,
}

impl Image {
    pub fn from_config(config: &Config) -> Self {
        Image {
            image_columns: config.image.image_columns.unwrap_or(24),
            image_rows: config.image.image_rows.unwrap_or(12),
            right_gap: config.image.right_gap.unwrap_or(3),
            left_gap: config.image.left_gap.unwrap_or(1),
        }
    }
}
