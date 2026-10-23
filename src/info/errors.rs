// Handle the errors octocrab throws, then print a message then exit
pub fn handle_octocrab_error(username: &str, e: octocrab::Error) -> ! {
    match e {
        // No internet connection
        octocrab::Error::Service { source, .. } if source.to_string().contains("Connect") => {
            eprintln!("No internet connection");
        }

        // User was not found
        octocrab::Error::GitHub { source, .. }
            if source.status_code == reqwest::StatusCode::NOT_FOUND =>
        {
            eprintln!("User '{}' not found", username);
        }

        // API rate limit exceeded
        octocrab::Error::GitHub { source, .. }
            if source.status_code == reqwest::StatusCode::FORBIDDEN
                && source.message.to_lowercase().contains("rate limit") =>
        {
            eprintln!("GitHub API rate limit exceeded");
        }

        // IDK what happened
        e => {
            eprintln!("Unexpected error: {:#?}", e);
        }
    }
    std::process::exit(1);
}
