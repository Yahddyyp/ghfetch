use crate::info::errors::handle_octocrab_error;
use octocrab::{
    Octocrab,
    models::{UserId, UserProfile},
};

pub struct UserInfo {
    pub name: String,
    pub followers: u64,
    pub avatar_url: String,
    pub created_at: String,
    pub language: Option<String>,
    pub public_repos: u64,
    pub issues: u32,
    pub twitter_user: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub id: UserId,
    pub bio: Option<String>,
    pub blog: Option<String>,
    pub stars: u32,
    pub following: u64,
    pub prs: u32,
}

/// Fetch the user profile
pub async fn get_user_info(username: &str, octocrab: &Octocrab) -> UserProfile {
    octocrab
        .users(username)
        .profile()
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e))
}
