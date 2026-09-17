use crate::display_info::print_user_info;
use crate::get_avatar_image::download_avatar;
use crate::totalstars::get_total_stars;
use crate::user_info::{UserInfo, get_user_info};
use anyhow::Result;
use std::env;
use std::num::NonZeroU32;

mod display_info;
mod get_avatar_image;
mod totalstars;
mod user_info;

fn print_avatar(path: &str) {
    let conf = viuer::Config {
        width: Some(20),
        height: Some(10),
        ..Default::default()
    };

    if viuer::print_from_file(path, &conf).is_err() {
        print_ascii_avatar(path);
    }
}

fn print_ascii_avatar(path: &str) {
    let img = match image::open(path) {
        Ok(img) => img,
        Err(_) => {
            println!("[could not load avatar]");
            return;
        }
    };

    let config = artem::config::ConfigBuilder::new()
        .target_size(NonZeroU32::new(30).unwrap())
        .build();

    let ascii = artem::convert(img, &config);
    println!("{}", ascii);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let username = args.get(1).ok_or("no username provided")?;
    let token = env::var("GHFETCH_TOKEN");
    let has_token = token.is_ok();

    // If pat token is avalible use that or else use unauthorized
    // requests to build the client instance
    let octocrab = match &token {
        Ok(token) => octocrab::Octocrab::builder()
            .personal_token(token.clone())
            .build()?,

        Err(_) => octocrab::Octocrab::builder().build()?,
    };

    let (user_result, stars_result) = tokio::join!(
        get_user_info(&args, username, &octocrab, has_token),
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

        avatar_url: user.avatar_url.to_string(),
    };
    let avater_path = download_avatar(&user_info.avatar_url).await?;
    print_avatar(&avater_path);

    print_user_info(&user_info);

    Ok(())
}
