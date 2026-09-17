use crate::user_info::UserInfo;
use owo_colors::OwoColorize;

/// Print the user info
pub fn print_user_info(info: &UserInfo) {
    println!("{}", info.name.bold().truecolor(203, 166, 247));
    println!("{}", "─".repeat(info.name.len()));
    println!("{:<12} {}", "ID".bold().truecolor(137, 220, 236), info.id);
    println!(
        "{:<12} {}",
        "Total Stars".bold().truecolor(166, 227, 161),
        info.total_stars
    );
    println!(
        "{:<12} {}",
        "Followers".bold().truecolor(250, 179, 125),
        info.followers
    );
    println!(
        "{:<12} {}",
        "Repos".bold().truecolor(116, 227, 161),
        info.public_repos
    );
    println!(
        "{:<12} {}",
        "Joined".bold().truecolor(137, 220, 235),
        info.created_at
    );

    if let Some(company) = &info.company {
        println!(
            "{:<12} {}",
            "Company".bold().truecolor(250, 179, 125),
            company
        )
    }

    if let Some(location) = &info.location {
        println!(
            "{:<12} {}",
            "Location".bold().truecolor(137, 220, 235),
            location
        )
    }

    if let Some(twitter) = &info.twitter_user {
        println!(
            "{:<12} @{}",
            "Twitter".bold().truecolor(203, 166, 247),
            twitter
        )
    }

    if let Some(blog) = &info.blog {
        println!("{:<12} {}", "Blog".bold().truecolor(203, 166, 247), blog)
    }

    if let Some(bio) = &info.bio {
        println!("\n{}", bio);
    }
}
