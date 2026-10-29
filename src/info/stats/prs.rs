use crate::info::errors::handle_octocrab_error;

/// Get the total pull requests.
pub async fn get_total_prs(octocrab: &octocrab::Octocrab, username: &str) -> u32 {
    let query = format!("author:{username} type:pr");

    let page = octocrab
        .search()
        .issues_and_pull_requests(&query)
        .per_page(1)
        .send()
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e));

    page.total_count.unwrap_or(0) as u32
}
