use std::collections::HashMap;

use anyhow::Result;
use futures_util::stream::{self, StreamExt, TryStreamExt};
use octocrab::Octocrab;

/// Get the most used language by the user.
pub async fn get_most_used_language(octocrab: &Octocrab, username: &str) -> Result<Option<String>> {
    let mut languages: HashMap<String, u64> = HashMap::new();

    let mut page = octocrab
        .users(username)
        .repos()
        .per_page(100)
        .send()
        .await?;

    loop {
        let repos: Vec<(String, String)> = page
            .items
            .iter()
            .filter(|repo| !repo.fork.unwrap_or(false))
            .filter_map(|repo| {
                repo.owner
                    .as_ref()
                    .map(|owner| (owner.login.clone(), repo.name.clone()))
            })
            .collect();

        let results = stream::iter(repos)
            .map(|(owner, name)| async move {
                let route = format!("/repos/{owner}/{name}/languages");

                let result: HashMap<String, u64> = octocrab.get(&route, None::<&()>).await?;

                Ok::<_, octocrab::Error>(result)
            })
            .buffer_unordered(10)
            .try_collect::<Vec<HashMap<String, u64>>>()
            .await?;

        for repo_languages in results {
            for (language, bytes) in repo_languages {
                *languages.entry(language).or_default() += bytes;
            }
        }

        match octocrab.get_page(&page.next).await? {
            Some(next_page) => page = next_page,
            None => break,
        }
    }

    Ok(languages
        .into_iter()
        .max_by_key(|(_, bytes)| *bytes)
        .map(|(language, _)| language))
}
