use anyhow::Result;
use octocrab::Octocrab;
use std::{env, process::Command};

/// Get token for authorized requests:
/// GITHUB_TOKEN -> gh auth token -> GHFETCH_TOKEN -> Unauthorized requests
pub fn get_octocrab(token: Option<String>) -> Result<(Octocrab, bool)> {
    let token = env::var("GITHUB_TOKEN")
        .ok()
        .and_then(clean_token)
        .or_else(get_gh_token)
        .or_else(|| token.and_then(clean_token));

    let has_token = token.is_some();

    let builder = Octocrab::builder();

    let client = match token {
        Some(token) => builder.personal_token(token).build()?,
        None => builder.build()?,
    };

    Ok((client, has_token))
}

/// Return a non-empty, trimmed token.
fn clean_token(token: String) -> Option<String> {
    let token = token.trim().to_owned();
    (!token.is_empty()).then_some(token)
}

/// Get a token from the GitHub CLI, if available and authenticated.
fn get_gh_token() -> Option<String> {
    let output = Command::new("gh").args(["auth", "token"]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout).ok().and_then(clean_token)
}
