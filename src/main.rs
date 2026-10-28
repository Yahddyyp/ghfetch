use crate::config_stuff::{Colors, Image, load_config};
use crate::info::{
    build_user_info, get_image, get_octocrab, get_total_issues, get_total_stars, get_user_info,
    print_user_info,
};

use anyhow::Result;
use clap::Parser;

mod config_stuff;
mod info;

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

    let (octocrab, token_source) = get_octocrab()?;

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

    let (user_result, stars_result, issues_result) = tokio::join!(
        get_user_info(username, &octocrab, token_source),
        get_total_stars(&octocrab, username),
        get_total_issues(&octocrab, username)
    );

    let user = user_result?;
    let total_stars = stars_result;
    let total_issues = issues_result;

    let user_info = build_user_info(user, total_stars, total_issues)?;

    if !cli.no_avatar {
        get_image(&user_info.avatar_url, image_id, &image).await?;
    }

    print_user_info(&user_info, &fields, &colors, &image, !cli.no_avatar);

    Ok(())
}
