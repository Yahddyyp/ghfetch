use std::env;

use anyhow::Result;
use octocrab::{Octocrab, models::UserProfile};

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
}

/// Print the user info clearly
fn print_user_info(info: &UserInfo) {
    println!("{}", info.name);
    println!("{}", "─".repeat(info.name.len()));
    println!("{:<12} {}", "ID", info.id);
    println!("{:<12} {}", "Followers", info.followers);
    println!("{:<12} {}", "Repos", info.public_repos);
    println!("{:<12} {}", "Joined", info.created_at);

    if let Some(company) = &info.company {
        println!("{:<12} {}", "Company", company)
    }

    if let Some(location) = &info.location {
        println!("{:<12} {}", "Location", location)
    }

    if let Some(twitter) = &info.twitter_user {
        println!("{:<12} {}", "Twitter", twitter)
    }

    if let Some(blog) = &info.blog {
        println!("{:<12} {}", "Blog", blog)
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

        // IDK what happened
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
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

    let user = get_user_info(&args, username, &octocrab).await?;

    let user_info = UserInfo {
        name: user.login,
        id: user.id.0.try_into()?,
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
