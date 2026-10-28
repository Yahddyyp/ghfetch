use crate::info::errors::handle_octocrab_error;
use octocrab::{Octocrab, models::UserProfile};

pub struct UserInfo {
    pub name: String,
    pub followers: i32,
    pub avatar_url: String,
    pub created_at: String,
    pub public_repos: u64,
    pub issues: u32,
    pub twitter_user: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub id: i64,
    pub bio: Option<String>,
    pub blog: Option<String>,
    pub stars: u32,
}

/// Build the client instance and get the info
pub async fn get_user_info(
    username: &str,
    octocrab: &Octocrab,
    token_source: Option<&'static str>,
) -> Result<UserProfile, Box<dyn std::error::Error>> {
    // Check if PAT actually works
    if let Some(source) = token_source {
        match octocrab.current().user().await {
            Err(octocrab::Error::GitHub { .. }) => {
                eprintln!("{source} is not valid");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error validating token from {source}: {e:#?}");
                std::process::exit(1);
            }
            Ok(_) => (),
        }
    }

    let user = octocrab
        .users(username)
        .profile()
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e));

    Ok(user)
}
