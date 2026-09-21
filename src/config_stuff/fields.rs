use crate::config_stuff::config::Config;
use serde::Deserialize;

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
