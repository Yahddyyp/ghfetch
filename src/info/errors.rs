use reqwest::StatusCode;

/// Handle the errors octocrab throws.
pub fn handle_octocrab_error(username: &str, e: octocrab::Error) -> ! {
    match e {
        // No internet connection
        octocrab::Error::Service { source, .. } if source.to_string().contains("Connect") => {
            eprintln!("No internet connection");
        }

        // User not found
        octocrab::Error::GitHub { source, .. } if source.status_code == StatusCode::NOT_FOUND => {
            eprintln!("User '{username}' not found");
        }

        // Bad token
        octocrab::Error::GitHub { source, .. }
            if source.status_code == StatusCode::UNAUTHORIZED =>
        {
            eprintln!("GitHub token is not valid");
        }

        // API rate limit exceeded
        octocrab::Error::GitHub { source, .. }
            if (source.status_code == StatusCode::FORBIDDEN
                && source.message.to_lowercase().contains("rate limit"))
                || source.status_code == StatusCode::TOO_MANY_REQUESTS =>
        {
            eprintln!("GitHub API rate limit exceeded");
        }

        // IDK what happened
        e => {
            eprintln!("Unexpected error: {e:#?}");
        }
    }

    std::process::exit(1);
}
