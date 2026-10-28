mod build_info;
mod display_info;
mod errors;
mod get_avatar_image;
mod token;
mod total_issues;
mod total_stars;
mod user_info;

pub use build_info::build_user_info;
pub use display_info::print_user_info;
pub use get_avatar_image::get_image;
pub use token::{get_octocrab, validate_token};
pub use total_issues::get_total_issues;
pub use total_stars::get_total_stars;
pub use user_info::{UserInfo, get_user_info};
