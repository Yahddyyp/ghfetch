use crate::info::errors::handle_octocrab_error;
use octocrab::{Octocrab, models::UserProfile};

pub struct UserInfo {
    pub name: String,
    pub followers: i32,
    pub avatar_url: String,
    pub created_at: String,
    pub language: Option<String>,
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

/// Fetch the user profile
pub async fn get_user_info(username: &str, octocrab: &Octocrab) -> UserProfile {
    octocrab
        .users(username)
        .profile()
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e))
}
