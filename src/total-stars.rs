/// Sum up all the stars of the public repos
async fn get_total_stars(octocrab: &octocrab::Octocrab, username: &str) -> octocrab::Result<u32> {
    let mut current_page = octocrab
        .users(username)
        .repos()
        .per_page(100)
        .send()
        .await?;

    let mut all_repos = current_page.take_items();

    while let Ok(Some(mut next_page)) = octocrab.get_page(&current_page.next).await {
        all_repos.extend(next_page.take_items());
        current_page = next_page;
    }

    let total_stars: u32 = all_repos
        .iter()
        .map(|r| r.stargazers_count.unwrap_or(0))
        .sum();

    Ok(total_stars)
}
