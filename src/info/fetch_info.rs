use crate::config_stuff::fields::Field;
use crate::config_stuff::fields::needs_field;
use crate::info::stats::most_used_languages::get_most_used_language;
use crate::info::stats::prs::get_total_prs;
use crate::info::stats::total_issues::get_total_issues;
use crate::info::stats::total_stars::get_total_stars;
use crate::info::user_info::get_user_info;

use octocrab::models::UserProfile;

/// Fetch the user data along with data octocrab does not have.
pub async fn fetch_user_data(
    username: &str,
    octocrab: &octocrab::Octocrab,
    fields: &[Field],
) -> (
    UserProfile,
    u32,
    u32,
    Result<Option<String>, anyhow::Error>,
    u32,
) {
    let fetch_stars = needs_field(fields, "stars");
    let fetch_issues = needs_field(fields, "issues");
    let fetch_language = needs_field(fields, "language");
    let fetch_prs = needs_field(fields, "prs");

    tokio::join!(
        get_user_info(username, octocrab),
        async {
            if fetch_stars {
                get_total_stars(octocrab, username).await
            } else {
                0
            }
        },
        async {
            if fetch_issues {
                get_total_issues(octocrab, username).await
            } else {
                0
            }
        },
        async {
            if fetch_language {
                get_most_used_language(octocrab, username).await
            } else {
                Ok(None)
            }
        },
        async {
            if fetch_prs {
                get_total_prs(octocrab, username).await
            } else {
                0
            }
        }
    )
}
