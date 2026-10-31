use crate::info::user_info::UserInfo;

use anyhow::Result;
use octocrab::models::UserProfile;

/// Build all the user info info a single struct.
pub fn build_user_info(
    user: UserProfile,
    total_stars: u32,
    total_issues: u32,
    most_used_languages: Result<Option<String>>,
    total_prs: u32,
) -> Result<UserInfo> {
    Ok(UserInfo {
        user: user.login,
        id: user.id,
        stars: total_stars,
        followers: user.followers,
        following: user.following,
        public_repos: user.public_repos,
        issues: total_issues,
        created_at: user.created_at.format("%Y-%m-%d").to_string(),
        language: most_used_languages?,
        prs: total_prs,

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
