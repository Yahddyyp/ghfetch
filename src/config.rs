use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub fields: Option<Vec<Field>>,
    #[serde(default)]
    pub layout: LayoutConfig,
}

impl Config {
    pub fn fields(&self) -> Vec<Field> {
        self.fields.clone().unwrap_or_else(|| {
            vec![
                Field::Id,
                Field::TotalStars,
                Field::Followers,
                Field::Repos,
                Field::Joined,
                Field::Company,
                Field::Location,
                Field::Twitter,
                Field::Blog,
                Field::Bio,
            ]
        })
    }
}

// Raw, optional values as read from the TOML file
#[derive(Deserialize, Default)]
pub struct LayoutConfig {
    // Size of the image
    pub image_columns: Option<usize>,
    pub image_rows: Option<usize>,
    // Gaps from the edges of the terminal
    pub right_gap: Option<usize>,
    pub left_gap: Option<usize>,
}

pub struct Layout {
    pub image_columns: usize,
    pub image_rows: usize,
    pub right_gap: usize,
    pub left_gap: usize,
}

impl Layout {
    pub fn from_config(config: &Config) -> Self {
        Layout {
            image_columns: config.layout.image_columns.unwrap_or(24),
            image_rows: config.layout.image_rows.unwrap_or(12),
            right_gap: config.layout.right_gap.unwrap_or(3),
            left_gap: config.layout.left_gap.unwrap_or(1),
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Id,
    TotalStars,
    Followers,
    Repos,
    Joined,
    Company,
    Location,
    Twitter,
    Blog,
    Bio,
}

pub fn load_config() -> Config {
    let path = match dirs::home_dir() {
        Some(home) => home.join(".config").join("ghfetch").join("config.toml"),
        None => return Config::default(),
    };

    if !path.exists() {
        write_default_config(&path);
    }

    match std::fs::read_to_string(&path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

fn write_default_config(path: &std::path::Path) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let default_contents = r#"
fields = [
    "id",
    "total_stars",
    "followers",
    "repos",
    "joined",
    "company",
    "location",
    "twitter",
    "blog",
    "bio",
]

[layout]
image_columns = 24
image_rows = 12
left_gap = 1
right_gap = 3
"#;

    let _ = std::fs::write(path, default_contents);
}
