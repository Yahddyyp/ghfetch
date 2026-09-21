use crate::config_stuff::config::Config;
use serde::Deserialize;

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
