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

impl Config {
    pub fn fields(&self) -> Vec<Field> {
        self.fields.clone().unwrap_or_else(|| {
            vec![
                Field::User,
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

// Raw, optional values as read from the config file
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

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum UnderlineField {
    User,
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

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum UnderlineTarget {
    Field(UnderlineField),
    Width(usize),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    User,
    Underline(UnderlineTarget),
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
    Break,
}

#[derive(Deserialize, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

#[derive(Deserialize, Default)]
pub struct ColorsConfig {
    pub name: Option<Color>,
    pub underline: Option<Color>,
    pub id: Option<Color>,
    pub total_stars: Option<Color>,
    pub followers: Option<Color>,
    pub repos: Option<Color>,
    pub joined: Option<Color>,
    pub company: Option<Color>,
    pub location: Option<Color>,
    pub twitter: Option<Color>,
    pub blog: Option<Color>,
}

pub struct Colors {
    pub name: (u8, u8, u8),
    pub underline: Option<(u8, u8, u8)>,
    pub id: (u8, u8, u8),
    pub total_stars: (u8, u8, u8),
    pub followers: (u8, u8, u8),
    pub repos: (u8, u8, u8),
    pub joined: (u8, u8, u8),
    pub company: (u8, u8, u8),
    pub location: (u8, u8, u8),
    pub twitter: (u8, u8, u8),
    pub blog: (u8, u8, u8),
}

impl Colors {
    // Take the colors from the config file and use them
    pub fn from_config(config: &Config) -> Self {
        let c = &config.colors;
        Colors {
            name: c.name.map(Color::to_rgb).unwrap_or((203, 166, 247)),
            underline: c.underline.map(Color::to_rgb),
            id: c.id.map(Color::to_rgb).unwrap_or((137, 220, 236)),
            total_stars: c.total_stars.map(Color::to_rgb).unwrap_or((166, 227, 161)),
            followers: c.followers.map(Color::to_rgb).unwrap_or((250, 179, 125)),
            repos: c.repos.map(Color::to_rgb).unwrap_or((116, 227, 161)),
            joined: c.joined.map(Color::to_rgb).unwrap_or((137, 220, 235)),
            company: c.company.map(Color::to_rgb).unwrap_or((250, 179, 125)),
            location: c.location.map(Color::to_rgb).unwrap_or((137, 220, 235)),
            twitter: c.twitter.map(Color::to_rgb).unwrap_or((203, 166, 247)),
            blog: c.blog.map(Color::to_rgb).unwrap_or((203, 166, 247)),
        }
    }
}

/// Load the config from file.
pub fn load_config() -> Result<Config> {
    let path = match dirs::home_dir() {
        Some(home) => home.join(".config").join("ghfetch").join("config.toml"),
        None => anyhow::bail!("could not find home directory"),
    };

    if !path.exists() {
        write_default_config(&path);
    }

    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    let config = toml::from_str::<Config>(&contents)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    Ok(config)
}

/// Write the default config.
fn write_default_config(path: &std::path::Path) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let default_contents = r#"fields = [
    "user",
    { underline = "user" },
    "id",
    "total_stars",
    "followers",
    "repos",
    "joined",
    "company",
    "location",
    "twitter",
    "blog",
    "break",
    "bio",
]

[colors]
name = { r = 203, g = 166, b = 247 }
# Uses the terminal's default foreground color if omitted
# underline = { r = 255, g = 255, b = 255 }
id = { r = 137, g = 220, b = 236 }
total_stars = { r = 166, g = 227, b = 161 }
followers = { r = 250, g = 179, b = 125 }
repos = { r = 116, g = 227, b = 161 }
joined = { r = 137, g = 220, b = 235 }
company = { r = 250, g = 179, b = 125 }
location = { r = 137, g = 220, b = 235 }
twitter = { r = 203, g = 166, b = 247 }
blog = { r = 203, g = 166, b = 247 }

[image]
image_columns = 24
image_rows = 12
left_gap = 1
right_gap = 3
"#;

    let _ = std::fs::write(path, default_contents);
}
