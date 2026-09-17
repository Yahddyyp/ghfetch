use octocrab::{Octocrab, models::UserProfile};

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserInfo {
    pub name: String,
    pub followers: i32,
    pub avatar_url: String,
    pub created_at: String,
    pub public_repos: u64,
    pub twitter_user: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub id: i64,
    pub bio: Option<String>,
    pub blog: Option<String>,
    pub total_stars: u32,
}

/// Build the client instance and get the info
pub async fn get_user_info(
    args: &[String],
    username: &str,
    octocrab: &Octocrab,
    token: bool,
) -> Result<UserProfile, Box<dyn std::error::Error>> {
    if args.len() >= 3 {
        eprintln!("Cannot take more than two arguments");
        std::process::exit(1);
    }

    // Check if PAT actually works
    if token {
        match octocrab.current().user().await {
            Err(octocrab::Error::GitHub { .. }) => {
                eprintln!("GHFETCH_TOKEN is not valid");
                std::process::exit(1);
            }

            Err(e) => {
                eprintln!("Error validating token: {e:#?}");
                std::process::exit(1);
            }

            Ok(_) => (),
        }
    }

    match octocrab.users(username).profile().await {
        // If user actually exists
        Ok(user) => Ok(user),

        // No wifi
        Err(octocrab::Error::Service { source, .. }) if source.to_string().contains("Connect") => {
            eprintln!("No wifi");
            std::process::exit(1);
        }

        // User was not found
        Err(octocrab::Error::GitHub { source, .. })
            if source.status_code == reqwest::StatusCode::NOT_FOUND =>
        {
            eprintln!("user '{}' not found", username);
            std::process::exit(1);
        }

        // API rate limit exceeded
        Err(octocrab::Error::GitHub { source, .. })
            if source.status_code == reqwest::StatusCode::FORBIDDEN
                && source.message.to_lowercase().contains("rate limit") =>
        {
            eprintln!("GitHub API rate limit exceeded");
            eprintln!("Try again later");
            std::process::exit(1);
        }

        // IDK what happened
        Err(e) => {
            eprintln!("Unexpected error: {:#?}", e);
            std::process::exit(1);
        }
    }
}
