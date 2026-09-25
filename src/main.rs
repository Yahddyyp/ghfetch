use crate::config_stuff::{Colors, Image, load_config};
use crate::display_info::print_user_info;
use crate::get_avatar_image::get_image;
use crate::totalstars::get_total_stars;
use crate::user_info::{UserInfo, get_user_info};
use anyhow::Result;
use clap::Parser;
use std::env;

mod config_stuff;
mod display_info;
mod errors;
mod get_avatar_image;
mod totalstars;
mod user_info;

#[derive(Parser, Debug)]
#[command(
    name = "ghfetch",
    about = "A way to beautifully display your github stats",
    author = "Yahddyyp"
)]
#[command(version)]
pub struct Cli {
    /// Github username or an organisation's name to fetch
    pub username: String,

    /// Do not display the profile avatar
    #[arg(long)]
    pub no_avatar: bool,

    /// Disable the colored output
    #[arg(long)]
    pub no_color: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.no_color {
        owo_colors::set_override(false);
    }

    let username = &cli.username;

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

    // If --no-color is passed do no colors and no bold
    let colors = if cli.no_color {
        Colors::from_config(&config).no_color()
    } else {
        Colors::from_config(&config)
    };

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
        get_user_info(username, &octocrab, has_token),
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

    if !cli.no_avatar {
        get_image(&user_info.avatar_url, image_id, &image).await?;
    }

    print_user_info(&user_info, &fields, &colors, &image, !cli.no_avatar);

    Ok(())
}
