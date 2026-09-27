use crate::errors::handle_octocrab_error;

/// Get the total issues of the user.
pub async fn get_issues(octocrab: &octocrab::Octocrab, username: &str) -> u32 {
    let query = format!("author:{} type:issue", username);

    let page = octocrab
        .search()
        .issues_and_pull_requests(&query)
        .per_page(1)
        .send()
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e));

    page.total_count.unwrap_or(0) as u32
}
