use crate::resolve::ResolvedTheme;

pub const DIRECTORY: &str = "agy-cli";
pub const FILES: &[&str] = &[
    "settings-theme.json",
    "schemes.json",
    "profile-font.json",
    crate::build::GUIDE,
];

pub fn render(theme: &ResolvedTheme) -> Result<Vec<(&'static str, String)>, String> {
    let terminal = crate::windows_terminal::render_for(theme, DIRECTORY)?;
    Ok(vec![
        (FILES[0], "{\n  \"colorScheme\": \"terminal\"\n}\n".into()),
        (FILES[1], terminal[0].1.clone()),
        (FILES[2], terminal[1].1.clone()),
        (FILES[3], include_str!("../Docs/28-agy-cli.md").into()),
    ])
}
