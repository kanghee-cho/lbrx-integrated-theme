use crate::{resolve::ResolvedTheme, vscode};

pub const DIRECTORY: &str = "antigravity";
pub const FILES: &[&str] = vscode::FILES;

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, Vec<u8>); 4], String> {
    let mut files = vscode::render_for(theme, DIRECTORY, "^1.90.0")?;
    files[3].1 = include_bytes!("../Docs/26-antigravity.md").to_vec();
    Ok(files)
}
