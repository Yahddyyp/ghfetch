use crate::{
    config_stuff::{Colors, Field, Image, UnderlineField, UnderlineTarget},
    user_info::UserInfo,
};
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

    lines.push(String::new());

    for field in fields {
        let line = match field {
            Field::User => Some(colorize(&info.name, colors.name, colors.enabled)),

            Field::Underline(target) => Some(underline(target, info, colors)),

            Field::Id => Some(format!(
                "{} {}",
                format_text("ID", colors.id, colors.enabled),
                info.id
            )),

            Field::TotalStars => Some(format!(
                "{} {}",
                format_text("Total Stars", colors.total_stars, colors.enabled),
                info.total_stars
            )),

            Field::Followers => Some(format!(
                "{} {}",
                format_text("Followers", colors.followers, colors.enabled),
                info.followers
            )),

            Field::Repos => Some(format!(
                "{} {}",
                format_text("Repos", colors.repos, colors.enabled),
                info.public_repos
            )),

            Field::Joined => Some(format!(
                "{} {}",
                format_text("Joined", colors.joined, colors.enabled),
                info.created_at
            )),

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

            Field::Bio => info.bio.as_ref().map(|b| b.to_string()),
        };

        if let Some(line) = line {
            lines.push(line);
        }
    }

    // Remove the empty string if that is the last thing in the lines
    while lines.len() > 1 && lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }

    let indent = if show_avatar {
        " ".repeat(layout.left_gap + layout.image_columns + layout.right_gap)
    } else {
        // No left gap for no avatar
        String::new()
    };

    if show_avatar {
        print!("\x1b[{}A\r", layout.image_rows);
    }

    let text_line_count = lines.len();

    for line in lines {
        println!("{indent}{line}");
    }

    // Ensure the cursor ends up below whichever is taller
    if show_avatar && layout.image_rows > text_line_count {
        let extra = layout.image_rows - text_line_count;
        if extra > 1 {
            print!("{}", "\n".repeat(extra - 1));
        }
    }

    if show_avatar {
        println!();
    }
}

/// Give the field width for underline.
fn field_width(field: UnderlineField, info: &UserInfo) -> usize {
    match field {
        UnderlineField::User => info.name.len(),

        UnderlineField::Id => format!("{:<12} {}", "ID", info.id).len(),

        UnderlineField::TotalStars => format!("{:<12} {}", "Total Stars", info.total_stars).len(),

        UnderlineField::Followers => format!("{:<12} {}", "Followers", info.followers).len(),

        UnderlineField::Repos => format!("{:<12} {}", "Repos", info.public_repos).len(),

        UnderlineField::Joined => format!("{:<12} {}", "Joined", info.created_at).len(),

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
    }
}

/// The actual underline.
fn underline(target: &UnderlineTarget, info: &UserInfo, colors: &Colors) -> String {
    let width = match target {
        UnderlineTarget::Field(field) => field_width(*field, info),
        UnderlineTarget::Width(width) => *width,
    };

    let underline = "─".repeat(width);

    match (colors.enabled, colors.underline) {
        (true, Some((r, g, b))) => underline.truecolor(r, g, b).to_string(),
        _ => underline,
    }
}
