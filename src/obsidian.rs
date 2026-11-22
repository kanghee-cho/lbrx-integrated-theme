use crate::{css::font_stack, resolve::ResolvedTheme};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "obsidian/LBRX";
pub const FILES: &[&str] = &["manifest.json", "theme.css"];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 2] {
    let manifest = serde_json::json!({
        "name": "LBRX",
        "version": env!("CARGO_PKG_VERSION"),
        "minAppVersion": "1.10.6",
        "author": "LBRX"
    });
    let mut css = String::new();
    for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
        writeln!(css, ".theme-{class} {{").unwrap();
        for (variable, role) in [
            ("background-primary", "background"),
            ("background-secondary", "surface"),
            ("text-normal", "text"),
            ("text-muted", "muted"),
            ("link-color", "link"),
            ("link-external-color", "link"),
            ("text-selection", "selection_background"),
            ("lbrx-selection-text", "selection_text"),
            ("code-background", "background"),
            ("code-normal", "text"),
            ("code-comment", "comment"),
            ("code-function", "function"),
            ("code-keyword", "keyword"),
            ("code-string", "string"),
        ] {
            writeln!(css, "  --{variable}: {};", mode.roles[role]).unwrap();
        }
        css.push_str("}\n");
    }
    css.push_str(".theme-light, .theme-dark {\n");
    for (variable, style) in [
        ("font-text-theme", &theme.typography.prose),
        ("font-monospace-theme", &theme.typography.mono),
    ] {
        writeln!(css, "  --{variable}: {};", font_stack(style)).unwrap();
    }
    css.push_str("}\n\n.markdown-rendered ::selection, .markdown-source-view ::selection {\n  background-color: var(--text-selection);\n  color: var(--lbrx-selection-text);\n}\n");
    [(FILES[0], format!("{manifest:#}\n")), (FILES[1], css)]
}
