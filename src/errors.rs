// Handle the errors octocrab throws, then print a message then exit
pub fn handle_octocrab_error(username: &str, e: octocrab::Error) -> ! {
    match e {
        // No wifi
        octocrab::Error::Service { source, .. } if source.to_string().contains("Connect") => {
            eprintln!("No wifi");
        }

        // User was not found
        octocrab::Error::GitHub { source, .. }
            if source.status_code == reqwest::StatusCode::NOT_FOUND =>
        {
            eprintln!("user '{}' not found", username);
        }

        // API rate limit exceeded
        octocrab::Error::GitHub { source, .. }
            if source.status_code == reqwest::StatusCode::FORBIDDEN
                && source.message.to_lowercase().contains("rate limit") =>
        {
            eprintln!("GitHub API rate limit exceeded");
            eprintln!("Try again later");
        }

        // IDK what happened
        e => {
            eprintln!("Unexpected error: {:#?}", e);
        }
    }
    std::process::exit(1);
}
