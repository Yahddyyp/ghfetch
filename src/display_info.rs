use crate::get_avatar_image::{IMAGE_COLUMNS, IMAGE_ROWS};
use crate::user_info::UserInfo;
use owo_colors::OwoColorize;

pub const RIGHT_GAP: usize = 3;
pub const LEFT_GAP: usize = 1;

pub fn print_user_info(info: &UserInfo) {
    let mut lines = Vec::new();

    lines.push(String::new());

    lines.push(format!("{}", info.name.bold().truecolor(203, 166, 247)));

    lines.push("─".repeat(info.name.len()).to_string());

    lines.push(format!(
        "{:<12} {}",
        "ID".bold().truecolor(137, 220, 236),
        info.id
    ));

    lines.push(format!(
        "{:<12} {}",
        "Total Stars".bold().truecolor(166, 227, 161),
        info.total_stars
    ));

    lines.push(format!(
        "{:<12} {}",
        "Followers".bold().truecolor(250, 179, 125),
        info.followers
    ));

    lines.push(format!(
        "{:<12} {}",
        "Repos".bold().truecolor(116, 227, 161),
        info.public_repos
    ));

    lines.push(format!(
        "{:<12} {}",
        "Joined".bold().truecolor(137, 220, 235),
        info.created_at
    ));

    if let Some(company) = &info.company {
        lines.push(format!(
            "{:<12} {}",
            "Company".bold().truecolor(250, 179, 125),
            company
        ));
    }

    if let Some(location) = &info.location {
        lines.push(format!(
            "{:<12} {}",
            "Location".bold().truecolor(137, 220, 235),
            location
        ));
    }

    if let Some(twitter) = &info.twitter_user {
        lines.push(format!(
            "{:<12} @{}",
            "Twitter".bold().truecolor(203, 166, 247),
            twitter
        ));
    }

    if let Some(blog) = &info.blog {
        lines.push(format!(
            "{:<12} {}",
            "Blog".bold().truecolor(203, 166, 247),
            blog
        ));
    }

    if let Some(bio) = &info.bio {
        lines.push(String::new());
        lines.push(bio.to_string());
    }

    print!("\x1b[{}A\r", IMAGE_ROWS);

    let indent = " ".repeat(LEFT_GAP + IMAGE_COLUMNS + RIGHT_GAP);
    let text_line_count = lines.len();

    for line in lines {
        println!("{indent}{line}");
    }

    // Ensure the cursor ends up below whichever is taller
    if IMAGE_ROWS > text_line_count {
        let extra = IMAGE_ROWS - text_line_count;
        print!("{}", "\n".repeat(extra));
    }
    println!()
}
