use std::env;

use crate::totalstars::get_total_stars;
use anyhow::Result;
use octocrab::{Octocrab, models::UserProfile};
use owo_colors::OwoColorize;

mod totalstars;

#[allow(dead_code)]
#[derive(Debug)]
struct UserInfo {
    name: String,
    followers: i32,
    avater_url: String,
    created_at: String,
    public_repos: u64,
    twitter_user: Option<String>,
    company: Option<String>,
    location: Option<String>,
    id: i64,
    bio: Option<String>,
    blog: Option<String>,
    total_stars: u32,
}

/// Print the user info
fn print_user_info(info: &UserInfo) {
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

/// Build the client instance and get the info
async fn get_user_info(
    args: &[String],
    username: &str,
    octocrab: &Octocrab,
) -> Result<UserProfile, Box<dyn std::error::Error>> {
    if args.len() >= 3 {
        eprintln!("Cannot take more than two arguments");
        std::process::exit(1);
    }

    match octocrab.users(username).profile().await {
        // If user actually exists
        Ok(user) => Ok(user),

        // No wifi
        Err(octocrab::Error::Service { source, .. }) if source.to_string().contains("Connect") => {
            eprintln!("No wifi");
            std::process::exit(1);
        }

        // User was not found
        Err(octocrab::Error::GitHub { source, .. })
            if source.status_code == reqwest::StatusCode::NOT_FOUND =>
        {
            eprintln!("user '{}' not found.", username);
            std::process::exit(1);
        }

        // API rate limit exceeded
        Err(octocrab::Error::GitHub { source, .. })
            if source.status_code == reqwest::StatusCode::FORBIDDEN =>
        {
            eprintln!("GitHub API rate limit exceeded.");
            std::process::exit(1);
        }

        // IDK what happened
        Err(e) => {
            eprintln!("Unexpected error: {:#?}", e);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let username = args.get(1).ok_or("no username provided")?;

    // Build the client instance
    let octocrab = octocrab::Octocrab::builder().build()?;

    let (user_result, stars_result) = tokio::join!(
        get_user_info(&args, username, &octocrab),
        get_total_stars(&octocrab, username)
    );

    let user = user_result?;
    let total_stars = stars_result?;

    let user_info = UserInfo {
        name: user.login,
        id: user.id.0.try_into()?,
        total_stars: total_stars,
        followers: user.followers.try_into()?,
        public_repos: user.public_repos,
        created_at: user.created_at.format("%d-%m-%Y at %I:%M %p").to_string(),

        bio: user
            .bio
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" ")),

        company: user
            .company
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" ")),

        location: user
            .location
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" ")),

        blog: user
            .blog
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" ")),

        twitter_user: user
            .twitter_username
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" ")),

        avater_url: user.avatar_url.to_string(),
    };

    print_user_info(&user_info);

    Ok(())
}
