use crate::config_stuff::config::Config;
use serde::Deserialize;

impl Config {
    pub fn fields(&self) -> Vec<Field> {
        self.fields.clone().unwrap_or_else(|| {
            vec![
                Field::User,
                Field::Id,
                Field::Stars,
                Field::Prs,
                Field::Followers,
                Field::Following,
                Field::Repos,
                Field::Issues,
                Field::Joined,
                Field::Language,
                Field::Company,
                Field::Location,
                Field::Twitter,
                Field::Blog,
                Field::Bio,
                Field::Text("".to_string()),
            ]
        })
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UnderlineField {
    User,
    Id,
    Stars,
    Prs,
    Followers,
    Following,
    Repos,
    Issues,
    Joined,
    Language,
    Company,
    Location,
    Twitter,
    Blog,
    Bio,
    Text(String),
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
    Stars,
    Prs,
    Followers,
    Following,
    Repos,
    Issues,
    Joined,
    Language,
    Company,
    Location,
    Twitter,
    Blog,
    Bio,
    Break,
    BreakIf(UnderlineField),
    Text(String),
}

/// Check what functions need to run based on what fields the
/// config file has.
pub fn needs_field(fields: &[Field], target: &str) -> bool {
    fields.iter().any(|field| match field {
        Field::Stars => target == "stars",
        Field::Issues => target == "issues",
        Field::Prs => target == "prs",
        Field::Language => target == "language",

        Field::Text(text) => text.contains(&format!("{{{target}}}")),

        _ => false,
    })
}
