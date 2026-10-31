use crate::Colors;
use crate::Image;
use crate::config_stuff::fields::{Field, UnderlineField, UnderlineTarget};
use crate::info::user_info::UserInfo;

use owo_colors::OwoColorize;

/// Check if the fields need to be colorized.
fn colorize(text: &str, color: (u8, u8, u8), enabled: bool) -> String {
    if enabled {
        let (r, g, b) = color;
        text.bold().truecolor(r, g, b).to_string()
    } else {
        text.to_string()
    }
}

/// Format the text.
fn format_text(text: &str, color: (u8, u8, u8), enabled: bool) -> String {
    let text = format!("{:<12}", text);
    colorize(&text, color, enabled)
}

/// Print and display the user info nicely.
pub fn print_user_info(
    info: &UserInfo,
    fields: &[Field],
    colors: &Colors,
    layout: &Image,
    show_avatar: bool,
) {
    let mut lines = Vec::new();

    for field in fields {
        let line = match field {
            Field::User => Some(colorize(&info.name, colors.name, colors.enabled)),

            Field::Underline(target) => Some(underline(target, info, colors)),

            Field::Id => Some(format!(
                "{} {}",
                format_text("ID", colors.id, colors.enabled),
                info.id
            )),

            Field::Stars => Some(format!(
                "{} {}",
                format_text("Stars", colors.stars, colors.enabled),
                info.stars
            )),

            Field::Prs => Some(format!(
                "{} {}",
                format_text("Prs", colors.prs, colors.enabled),
                info.prs
            )),

            Field::Followers => Some(format!(
                "{} {}",
                format_text("Followers", colors.followers, colors.enabled),
                info.followers
            )),

            Field::Following => Some(format!(
                "{} {}",
                format_text("Following", colors.following, colors.enabled),
                info.following
            )),

            Field::Repos => Some(format!(
                "{} {}",
                format_text("Repos", colors.repos, colors.enabled),
                info.public_repos
            )),

            Field::Issues => Some(format!(
                "{} {}",
                format_text("Issues", colors.issues, colors.enabled),
                info.issues
            )),

            Field::Joined => Some(format!(
                "{} {}",
                format_text("Joined", colors.joined, colors.enabled),
                info.created_at
            )),

            Field::Language => info.language.as_ref().map(|language| {
                format!(
                    "{} {}",
                    format_text("Language", colors.language, colors.enabled),
                    language
                )
            }),

            Field::Company => info.company.as_ref().map(|c| {
                format!(
                    "{} {}",
                    format_text("Company", colors.company, colors.enabled),
                    c
                )
            }),

            Field::Location => info.location.as_ref().map(|l| {
                format!(
                    "{} {}",
                    format_text("Location", colors.location, colors.enabled),
                    l
                )
            }),

            Field::Twitter => info.twitter_user.as_ref().map(|t| {
                format!(
                    "{} @{}",
                    format_text("Twitter", colors.twitter, colors.enabled),
                    t
                )
            }),

            Field::Blog => info.blog.as_ref().map(|bl| {
                format!(
                    "{} {}",
                    format_text("Blog", colors.blog, colors.enabled),
                    bl
                )
            }),

            Field::Break => Some(String::new()),

            Field::BreakIf(field) => field_has_value(field, info).then(String::new),

            Field::Bio => info
                .bio
                .as_deref()
                .filter(|bio| !bio.trim().is_empty())
                .map(str::to_owned),

            Field::Text(text) => Some(text.to_string()),
        };

        if let Some(line) = line {
            lines.push(line);
        }
    }

    let indent = if show_avatar {
        " ".repeat(layout.left_gap + layout.image_columns + layout.right_gap)
    } else {
        // No left gap for no avatar
        String::new()
    };

    if show_avatar {
        print!("\x1b[{}A\r", layout.image_rows.saturating_sub(1));
    }

    let text_line_count = lines.len();

    for line in lines {
        println!("{indent}{line}");
    }

    // Determine the taller element between the avatar image and the fields
    let max_lines = if show_avatar {
        layout.image_rows.max(text_line_count)
    } else {
        text_line_count
    };

    // Pad down if the image is taller than the text
    let extra = max_lines.saturating_sub(text_line_count);
    if extra > 0 {
        print!("{}", "\n".repeat(extra));
    }
}

/// Give the field width for underline.
fn field_width(field: &UnderlineField, info: &UserInfo) -> usize {
    match field {
        UnderlineField::User => info.name.len(),

        UnderlineField::Id => format!("{:<12} {}", "ID", info.id).len(),

        UnderlineField::Stars => format!("{:<12} {}", "Stars", info.stars).len(),

        UnderlineField::Prs => format!("{:<12} {}", "Prs", info.prs).len(),

        UnderlineField::Followers => format!("{:<12} {}", "Followers", info.followers).len(),

        UnderlineField::Following => format!("{:<12} {}", "Following", info.following).len(),

        UnderlineField::Repos => format!("{:<12} {}", "Repos", info.public_repos).len(),

        UnderlineField::Issues => format!("{:<12} {}", "Issues", info.issues).len(),

        UnderlineField::Joined => format!("{:<12} {}", "Joined", info.created_at).len(),

        UnderlineField::Language => match &info.language {
            Some(language) => format!("{:<12} {}", "Language", language).len(),
            None => 0,
        },

        UnderlineField::Company => match &info.company {
            Some(company) => format!("{:<12} {}", "Company", company).len(),
            None => 0,
        },

        UnderlineField::Location => match &info.location {
            Some(location) => format!("{:<12} {}", "Location", location).len(),
            None => 0,
        },

        UnderlineField::Twitter => match &info.twitter_user {
            Some(twitter) => format!("{:<12} @{}", "Twitter", twitter).len(),
            None => 0,
        },

        UnderlineField::Blog => match &info.blog {
            Some(blog) => format!("{:<12} {}", "Blog", blog).len(),
            None => 0,
        },

        UnderlineField::Bio => info.bio.as_deref().unwrap_or("").len(),

        UnderlineField::Text(text) => text.len(),
    }
}

/// The actual underline.
fn underline(target: &UnderlineTarget, info: &UserInfo, colors: &Colors) -> String {
    let width = match target {
        UnderlineTarget::Field(field) => field_width(field, info),
        UnderlineTarget::Width(width) => *width,
    };

    let underline = "─".repeat(width);

    match (colors.enabled, colors.underline) {
        (true, Some((r, g, b))) => underline.truecolor(r, g, b).to_string(),
        _ => underline,
    }
}

/// Check if value will always be printed or not.
fn field_has_value(field: &UnderlineField, info: &UserInfo) -> bool {
    match field {
        UnderlineField::User
        | UnderlineField::Id
        | UnderlineField::Stars
        | UnderlineField::Prs
        | UnderlineField::Followers
        | UnderlineField::Following
        | UnderlineField::Repos
        | UnderlineField::Issues
        | UnderlineField::Joined => true,

        UnderlineField::Language => info.language.is_some(),
        UnderlineField::Company => info.company.is_some(),
        UnderlineField::Location => info.location.is_some(),
        UnderlineField::Twitter => info.twitter_user.is_some(),
        UnderlineField::Blog => info.blog.is_some(),

        UnderlineField::Bio => info
            .bio
            .as_deref()
            .is_some_and(|bio| !bio.trim().is_empty()),

        UnderlineField::Text(text) => !text.trim().is_empty(),
    }
}
