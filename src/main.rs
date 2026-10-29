use crate::config_stuff::colors::Colors;
use crate::config_stuff::config::load_config;
use crate::config_stuff::image::Image;
use crate::info::build_info::build_user_info;
use crate::info::display_info::print_user_info;
use crate::info::stats::total_issues::get_total_issues;
use crate::info::stats::total_stars::get_total_stars;
use crate::info::token::{get_octocrab, validate_token};
use crate::info::user_info::get_user_info;
use info::get_avatar_image::get_image;

use anyhow::Result;
use clap::Parser;

mod config_stuff;
mod info;

#[derive(Parser, Debug)]
#[command(
    name = "ghfetch",
    about = "A way to beautifully display your GitHub stats.",
    author = "Yahddyyp"
)]
#[command(version)]
pub struct Cli {
    /// GitHub username or an organisation's name to fetch
    pub username: String,

    /// Do not display the profile avatar
    #[arg(long)]
    pub no_avatar: bool,

    /// Disable the colored output
    #[arg(long)]
    pub no_color: bool,

    #[arg(long)]
    /// Token to make the authorized GitHub API requests
    pub access_token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.no_color {
        owo_colors::set_override(false);
    }

    let username = &cli.username;

    let (octocrab, token_source) = get_octocrab(cli.access_token)?;
    validate_token(&octocrab, token_source).await;

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

    let (user, stars, issues) = tokio::join!(
        get_user_info(username, &octocrab),
        get_total_stars(&octocrab, username),
        get_total_issues(&octocrab, username)
    );

    let user_info = build_user_info(user, stars, issues)?;

    if !cli.no_avatar {
        get_image(&user_info.avatar_url, image_id, &image).await?;
    }

    print_user_info(&user_info, &fields, &colors, &image, !cli.no_avatar);

    Ok(())
}
