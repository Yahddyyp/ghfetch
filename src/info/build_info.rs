use crate::info::user_info::UserInfo;

use anyhow::Result;
use octocrab::models::UserProfile;

/// Build all the user info info a single struct.
pub fn build_user_info(
    user: UserProfile,
    total_stars: u32,
    total_issues: u32,
    most_used_languages: Result<Option<String>>,
) -> Result<UserInfo> {
    Ok(UserInfo {
        name: user.login,
        id: user.id.0.try_into()?,
        stars: total_stars,
        followers: user.followers.try_into()?,
        public_repos: user.public_repos,
        issues: total_issues,
        created_at: user.created_at.format("%d-%m-%Y at %I:%M %p").to_string(),
        language: most_used_languages?,

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
    })
}
