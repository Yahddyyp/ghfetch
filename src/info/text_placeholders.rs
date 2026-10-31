use crate::info::user_info::UserInfo;
use std::borrow::Cow;

/// What values the placeholder should have.
fn placeholder_value<'a>(key: &str, info: &'a UserInfo) -> Option<Cow<'a, str>> {
    match key {
        "user" => Some(Cow::Borrowed(info.user.as_str())),
        "bio" => info.bio.as_deref().map(Cow::Borrowed),
        "company" => info.company.as_deref().map(Cow::Borrowed),
        "location" => info.location.as_deref().map(Cow::Borrowed),
        "language" => info.language.as_deref().map(Cow::Borrowed),
        "twitter" => info.twitter_user.as_deref().map(Cow::Borrowed),
        "blog" => info.blog.as_deref().map(Cow::Borrowed),

        // Numeric fields to format into owned strings
        "id" => Some(Cow::Owned(info.id.to_string())),
        "stars" => Some(Cow::Owned(info.stars.to_string())),
        "prs" => Some(Cow::Owned(info.prs.to_string())),
        "followers" => Some(Cow::Owned(info.followers.to_string())),
        "following" => Some(Cow::Owned(info.following.to_string())),
        "repos" => Some(Cow::Owned(info.public_repos.to_string())),
        "issues" => Some(Cow::Owned(info.issues.to_string())),

        "joined" => Some(Cow::Borrowed(info.created_at.as_str())),

        _ => None,
    }
}

/// Check if it is a known placeholder.
fn is_known_placeholder(key: &str) -> bool {
    matches!(
        key,
        "user"
            | "id"
            | "stars"
            | "prs"
            | "followers"
            | "following"
            | "repos"
            | "issues"
            | "joined"
            | "language"
            | "company"
            | "location"
            | "twitter"
            | "blog"
            | "bio"
    )
}

/// Render the {user} placeholder with the actual value.
pub fn render_template(template: &str, info: &UserInfo) -> String {
    let mut output = String::with_capacity(template.len());
    let mut remaining = template;

    while let Some(start) = remaining.find('{') {
        output.push_str(&remaining[..start]);

        let after_open = &remaining[start + 1..];

        let Some(end) = after_open.find('}') else {
            output.push_str(&remaining[start..]);
            return output;
        };

        let placeholder = &after_open[..end];

        match placeholder_value(placeholder, info) {
            Some(value) => output.push_str(value.as_ref()),
            None if is_known_placeholder(placeholder) => {
                // If it is a know field but it has no value then don't print anything
            }
            None => {
                // Preserve unknown placeholder
                output.push('{');
                output.push_str(placeholder);
                output.push('}');
            }
        }

        remaining = &after_open[end + 1..];
    }

    output.push_str(remaining);
    output
}
