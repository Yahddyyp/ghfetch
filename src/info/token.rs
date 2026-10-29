use crate::info::errors::handle_octocrab_error;
use anyhow::Result;
use octocrab::Octocrab;
use std::{env, process::Command};

/// Validate that the token is authorized before making requests.
pub async fn validate_token(octocrab: &Octocrab, token_source: Option<&str>) {
    if let Some(source) = token_source
        && let Err(e) = octocrab.current().user().await
    {
        match e {
            octocrab::Error::GitHub { source: gh_err, .. }
                if gh_err.status_code == reqwest::StatusCode::UNAUTHORIZED =>
            {
                eprintln!("{source} is not valid");
                std::process::exit(1);
            }

            other => handle_octocrab_error("", other),
        }
    }
}

/// Get token for authorized requests:
/// GHFETCH_TOKEN -> gh auth token -> GITHUB_TOKEN -> Unauthorized requests
pub fn get_octocrab(access_token: Option<String>) -> Result<(Octocrab, Option<&'static str>)> {
    let (token, source) = if let Some(t) = access_token.and_then(clean_token) {
        (Some(t), Some("access-token"))
    } else if let Some(t) = env::var("GHFETCH_TOKEN").ok().and_then(clean_token) {
        (Some(t), Some("GHFETCH_TOKEN"))
    } else if let Some(t) = get_gh_token() {
        (Some(t), Some("gh auth token"))
    } else if let Some(t) = env::var("GITHUB_TOKEN").ok().and_then(clean_token) {
        (Some(t), Some("GITHUB_TOKEN"))
    } else {
        (None, None)
    };

    let builder = Octocrab::builder();

    let client = match token {
        Some(token) => builder.personal_token(token).build()?,
        None => builder.build()?,
    };

    Ok((client, source))
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
