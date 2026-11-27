use crate::resolve::{ResolvedMode, ResolvedTheme};
use serde_json::{Value, json};

pub const DIRECTORY: &str = "claude-code";
pub const FILES: &[&str] = &[
    "lbrx-light.json",
    "lbrx-dark.json",
    "schemes.json",
    crate::build::GUIDE,
];
pub const COLORS: &[(&str, &str)] = &[
    ("claude", "accent"),
    ("claudeShimmer", "accent"),
    ("text", "text"),
    ("inverseText", "background"),
    ("inactive", "muted"),
    ("inactiveShimmer", "muted"),
    ("subtle", "muted"),
    ("suggestion", "function"),
    ("permission", "focus"),
    ("permissionShimmer", "focus"),
    ("remember", "keyword"),
    ("success", "success"),
    ("error", "error"),
    ("warning", "warning"),
    ("warningShimmer", "warning"),
    ("merged", "keyword"),
    ("promptBorder", "focus"),
    ("promptBorderShimmer", "focus"),
    ("planMode", "function"),
    ("autoAccept", "success"),
    ("bashBorder", "muted"),
    ("ide", "function"),
    ("fastMode", "warning"),
    ("fastModeShimmer", "warning"),
    ("userMessageBackground", "surface"),
    ("userMessageBackgroundHover", "selection_background"),
    ("bashMessageBackgroundColor", "surface"),
    ("memoryBackgroundColor", "surface"),
    ("selectionBg", "selection_background"),
    ("composerSidebarBackground", "surface"),
    ("rate_limit_fill", "accent"),
    ("rate_limit_empty", "muted"),
    ("briefLabelYou", "function"),
    ("briefLabelClaude", "accent"),
];

pub fn is_color(key: &str) -> bool {
    COLORS.iter().any(|(token, _)| *token == key)
}

fn color_theme(theme: &ResolvedTheme, mode: &ResolvedMode, base: &str) -> Value {
    let overrides: serde_json::Map<String, Value> = COLORS
        .iter()
        .map(|(token, role)| {
            (
                (*token).into(),
                json!(theme.editor_color(mode, DIRECTORY, token, &mode.roles[*role])),
            )
        })
        .collect();
    json!({
        "name": if base == "light" { "LBRX Light" } else { "LBRX Dark" },
        "base": base,
        "overrides": overrides,
    })
}

pub fn render(theme: &ResolvedTheme) -> Result<Vec<(&'static str, String)>, String> {
    let terminal = crate::windows_terminal::render(theme)?;
    let mut files = Vec::new();
    for (file, base, mode) in [
        (FILES[0], "light", &theme.light),
        (FILES[1], "dark", &theme.dark),
    ] {
        let contents = serde_json::to_string_pretty(&color_theme(theme, mode, base))
            .map_err(|e| format!("{DIRECTORY}\\{file}: JSON 생성 실패: {e}"))?;
        files.push((file, contents + "\n"));
    }
    files.push((FILES[2], terminal[0].1.clone()));
    files.push((FILES[3], include_str!("../Docs/27-claude-code.md").into()));
    Ok(files)
}
