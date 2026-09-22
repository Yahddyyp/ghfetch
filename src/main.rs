use crate::config_stuff::{Colors, Image, load_config};
use crate::display_info::print_user_info;
use crate::get_avatar_image::get_image;
use crate::totalstars::get_total_stars;
use crate::user_info::{UserInfo, get_user_info};
use anyhow::Result;
use std::env;

mod config_stuff;
mod display_info;
mod errors;
mod get_avatar_image;
mod totalstars;
mod user_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // TODO: Convert the flag system to use clap instead of just a match
    match args.get(1).map(String::as_str) {
        Some("--help") | Some("-h") => {
            println!("A way to beautifully display your github stats");
            println!();
            println!("Made by Yahddyyp");
            println!();
            println!("Usage:");
            println!("    ghfetch <username>");
            println!();
            println!("Options:");
            println!("    -h, --help       Print help");
            println!("    -v, --version    Print version");

            return Ok(());
        }

        Some("--version") | Some("-v") => {
            println!("ghfetch {}", env!("CARGO_PKG_VERSION"));

            return Ok(());
        }

        _ => {}
    }

    let username = match args.get(1) {
        Some(username) => username,
        None => {
            println!("A way to beautifully display your github stats");
            println!();
            println!("Made by Yahddyyp");
            println!();
            println!("Usage:");
            println!("    ghfetch <username>");
            println!();
            println!("Options:");
            println!("    -h, --help       Print help");
            println!("    -v, --version    Print version");
            return Ok(());
        }
    };

    // Take GHFETCH_TOKEN from env, if it returns "" then take it as not being there
    let token = env::var("GHFETCH_TOKEN")
        .ok()
        .filter(|token| !token.is_empty());
    let has_token = token.is_some();
    // ID of the image from the process id given by the os
    let image_id = std::process::id();

    // Load the config and use it
    let config = load_config()?;
    let fields = config.fields();
    let colors = Colors::from_config(&config);
    let image = Image::from_config(&config);

    // If pat token is avalible use that or else use unauthorized
    // requests and build the client instance
    let octocrab = match &token {
        Some(token) => octocrab::Octocrab::builder()
            .personal_token(token.clone())
            .build()?,

        None => octocrab::Octocrab::builder().build()?,
    };

    let (user_result, stars_result) = tokio::join!(
        get_user_info(&args, username, &octocrab, has_token),
        get_total_stars(&octocrab, username)
    );

    let user = user_result?;
    let total_stars = stars_result;

    let user_info = UserInfo {
        name: user.login,
        id: user.id.0.try_into()?,
        total_stars,
        followers: user.followers.try_into()?,
        public_repos: user.public_repos,
        created_at: user.created_at.format("%d-%m-%Y at %I:%M %p").to_string(),

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

        bio: user
            .bio
            .map(|b| b.split_whitespace().collect::<Vec<_>>().join(" ")),

        avatar_url: format!("{}&s=200", user.avatar_url),
    };

    get_image(&user_info.avatar_url, image_id, &image).await?;

    print_user_info(&user_info, &fields, &colors, &image);

    Ok(())
}
