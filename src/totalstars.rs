use crate::errors::handle_octocrab_error;

/// Sum up all the stars of the public repos
pub async fn get_total_stars(octocrab: &octocrab::Octocrab, username: &str) -> u32 {
    let mut current_page = octocrab
        .users(username)
        .repos()
        .per_page(100)
        .send()
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e));

    let mut all_repos = current_page.take_items();

    // Keep fetching the next page of repos, adding each page's repos to the running list,
    // until GitHub says there are no more pages left
    while let Some(mut next_page) = octocrab
        .get_page(&current_page.next)
        .await
        .unwrap_or_else(|e| handle_octocrab_error(username, e))
    {
        all_repos.extend(next_page.take_items());
        current_page = next_page;
    }

    all_repos
        .iter()
        .map(|r| r.stargazers_count.unwrap_or(0))
        .sum()
}
