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
    twitter_user: String,
    company: String,
    location: String,
    id: i64,
    bio: String,
    blog: String,
    total_stars: u32,
}

/// Sum up all the stars of the public repos
async fn get_total_stars(octocrab: &octocrab::Octocrab, username: &str) -> octocrab::Result<u32> {
    let mut current_page = octocrab
        .users(username)
        .repos()
        .per_page(100)
        .send()
        .await?;

    let mut all_repos = current_page.take_items();

    while let Ok(Some(mut next_page)) = octocrab.get_page(&current_page.next).await {
        all_repos.extend(next_page.take_items());
        current_page = next_page;
    }

    let total_stars: u32 = all_repos
        .iter()
        .map(|r| r.stargazers_count.unwrap_or(0))
        .sum();

    Ok(total_stars)
}

/// Build the client instance and get the info
async fn get_user_info(
    args: &Vec<String>,
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

    let user = get_user_info(&args, &username, &octocrab).await?;

    let total_stars_var = get_total_stars(&octocrab, &username).await?;

    let user_info = UserInfo {
        name: user.login,
        followers: user.followers.try_into()?,
        total_stars: total_stars_var,
        created_at: user.created_at.format("%d-%m-%Y at %I:%M %p").to_string(),
        public_repos: user.public_repos,
        blog: user.blog.unwrap_or_else(|| "NOT_FOUND".to_string()),
        twitter_user: user
            .twitter_username
            .unwrap_or_else(|| "NOT_FOUND".to_string()),
        company: user.company.unwrap_or_else(|| "Unemployed".to_string()),
        location: user.location.unwrap_or("No where".to_string()),
        id: user.id.0.try_into()?,
        avater_url: user.avatar_url.to_string(),
        bio: user
            .bio
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" "))
            .unwrap_or_else(|| "NOT_FOUND".to_string()),
    };

    println!("{user_info:#?}");

    Ok(())
}
