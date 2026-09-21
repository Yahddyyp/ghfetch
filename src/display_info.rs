use crate::{
    config::{Field, Layout},
    user_info::UserInfo,
};
use owo_colors::OwoColorize;

pub fn print_user_info(info: &UserInfo, fields: &[Field], layout: &Layout) {
    let mut lines = Vec::new();

    lines.push(String::new());

    lines.push(format!("{}", info.name.bold().truecolor(203, 166, 247)));

    lines.push("─".repeat(info.name.len()));

    for field in fields {
        let line = match field {
            Field::Id => Some(format!(
                "{:<12} {}",
                "ID".bold().truecolor(137, 220, 236),
                info.id
            )),
            Field::TotalStars => Some(format!(
                "{:<12} {}",
                "Total Stars".bold().truecolor(166, 227, 161),
                info.total_stars
            )),
            Field::Followers => Some(format!(
                "{:<12} {}",
                "Followers".bold().truecolor(250, 179, 125),
                info.followers
            )),
            Field::Repos => Some(format!(
                "{:<12} {}",
                "Repos".bold().truecolor(116, 227, 161),
                info.public_repos
            )),
            Field::Joined => Some(format!(
                "{:<12} {}",
                "Joined".bold().truecolor(137, 220, 235),
                info.created_at
            )),
            Field::Company => info
                .company
                .as_ref()
                .map(|c| format!("{:<12} {}", "Company".bold().truecolor(250, 179, 125), c)),
            Field::Location => info
                .location
                .as_ref()
                .map(|l| format!("{:<12} {}", "Location".bold().truecolor(137, 220, 235), l)),
            Field::Twitter => info
                .twitter_user
                .as_ref()
                .map(|t| format!("{:<12} @{}", "Twitter".bold().truecolor(203, 166, 247), t)),
            Field::Blog => info
                .blog
                .as_ref()
                .map(|b| format!("{:<12} {}", "Blog".bold().truecolor(203, 166, 247), b)),
            Field::Bio => info.bio.as_ref().map(|b| b.to_string()),
        };

        if let Some(line) = line {
            if *field == Field::Bio {
                lines.push(String::new());
            }
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
