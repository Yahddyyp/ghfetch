use crate::{
    config::{Colors, Field, Image},
    user_info::UserInfo,
};
use owo_colors::OwoColorize;

pub fn print_user_info(info: &UserInfo, fields: &[Field], colors: &Colors, layout: &Image) {
    let mut lines = Vec::new();

    let (r, g, b) = colors.name;

    lines.push(String::new());

    lines.push(format!("{}", info.name.bold().truecolor(r, g, b)));

    lines.push("─".repeat(info.name.len()));

    for field in fields {
        let line = match field {
            Field::Id => {
                let (r, g, b) = colors.id;
                Some(format!(
                    "{:<12} {}",
                    "ID".bold().truecolor(r, g, b),
                    info.id
                ))
            }

            Field::TotalStars => {
                let (r, g, b) = colors.total_stars;
                Some(format!(
                    "{:<12} {}",
                    "Total Stars".bold().truecolor(r, g, b),
                    info.total_stars
                ))
            }

            Field::Followers => {
                let (r, g, b) = colors.followers;
                Some(format!(
                    "{:<12} {}",
                    "Followers".bold().truecolor(r, g, b),
                    info.followers
                ))
            }

            Field::Repos => {
                let (r, g, b) = colors.repos;
                Some(format!(
                    "{:<12} {}",
                    "Repos".bold().truecolor(r, g, b),
                    info.public_repos
                ))
            }

            Field::Joined => {
                let (r, g, b) = colors.joined;
                Some(format!(
                    "{:<12} {}",
                    "Joined".bold().truecolor(r, g, b),
                    info.created_at
                ))
            }

            Field::Company => {
                let (r, g, b) = colors.company;
                info.company
                    .as_ref()
                    .map(|c| format!("{:<12} {}", "Company".bold().truecolor(r, g, b), c))
            }

            Field::Location => {
                let (r, g, b) = colors.location;
                info.location
                    .as_ref()
                    .map(|l| format!("{:<12} {}", "Location".bold().truecolor(r, g, b), l))
            }

            Field::Twitter => {
                let (r, g, b) = colors.twitter;
                info.twitter_user
                    .as_ref()
                    .map(|t| format!("{:<12} @{}", "Twitter".bold().truecolor(r, g, b), t))
            }

            Field::Blog => {
                let (r, g, b) = colors.blog;
                info.blog
                    .as_ref()
                    .map(|bl| format!("{:<12} {}", "Blog".bold().truecolor(r, g, b), bl))
            }

            Field::Break => Some(String::new()),

            Field::Bio => info.bio.as_ref().map(|b| b.to_string()),
        };

        if let Some(line) = line {
            lines.push(line);
        }
    }

    print!("\x1b[{}A\r", layout.image_rows);

    let indent = " ".repeat(layout.left_gap + layout.image_columns + layout.right_gap);
    let text_line_count = lines.len();

    for line in lines {
        println!("{indent}{line}");
    }

    // Ensure the cursor ends up below whichever is taller
    if layout.image_rows > text_line_count {
        let extra = layout.image_rows - text_line_count;
        print!("{}", "\n".repeat(extra));
    }
    println!()
}
