use std::env;

#[allow(dead_code)]
#[derive(Debug)]
struct UserInfo {
    name: String,
    followers: i32,
    avater_url: String,
    created_at: String,
    public_repos: u64,
    twitter_user: String,
    company: String,
    location: String,
    id: i64,
}

async fn get_user_info(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() >= 3 {
        eprintln!("Cannot take more than two arguments");
        std::process::exit(1);
    }

    let username = args.get(1).ok_or("no username provided")?;

    let octocrab = octocrab::Octocrab::builder().build()?;

    match octocrab.users(username).profile().await {
        Ok(user) => {
            let user_info = UserInfo {
                name: user.login,
                followers: user.followers.try_into()?,
                created_at: user.created_at.format("%d-%m-%Y at %I:%M %p").to_string(),
                public_repos: user.public_repos,
                twitter_user: user.twitter_username.unwrap_or_else(|| "none".to_string()),
                company: user.company.unwrap_or_else(|| "Unemployed".to_string()),
                location: user.location.unwrap_or("No where".to_string()),
                id: user.id.0.try_into()?,
                avater_url: user.avatar_url.to_string(),
            };
            println!("{user_info:#?}");
        }
        Err(e) => {
            eprintln!("Error: user '{}' not found ({})", username, e);
            std::process::exit(1);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    get_user_info(args).await?;

    Ok(())
}
