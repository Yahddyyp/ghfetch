/// Write the default config.
pub fn write_default_config(path: &std::path::Path) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let default_contents = r#"fields = [
    "user",
    { underline = "user" },
    "id",
    "total_stars",
    "followers",
    "repos",
    "joined",
    "company",
    "location",
    "twitter",
    "blog",
    "break",
    "bio",
]

[colors]
name = { r = 203, g = 166, b = 247 }
# Uses the terminal's default foreground color if omitted
# underline = { r = 255, g = 255, b = 255 }
id = { r = 137, g = 220, b = 236 }
total_stars = { r = 166, g = 227, b = 161 }
followers = { r = 250, g = 179, b = 125 }
repos = { r = 116, g = 227, b = 161 }
joined = { r = 137, g = 220, b = 235 }
company = { r = 250, g = 179, b = 125 }
location = { r = 137, g = 220, b = 235 }
twitter = { r = 203, g = 166, b = 247 }
blog = { r = 203, g = 166, b = 247 }

[image]
image_columns = 24
image_rows = 12
left_gap = 1
right_gap = 3
"#;

    let _ = std::fs::write(path, default_contents);
}
