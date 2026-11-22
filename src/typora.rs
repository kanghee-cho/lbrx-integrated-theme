use crate::{
    css::font_stack,
    model::Typography,
    resolve::{ResolvedMode, ResolvedTheme},
};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "typora";
pub const LIGHT: &str = "lbrx-light.css";
pub const DARK: &str = "lbrx-dark.css";
pub const FILES: &[&str] = &[LIGHT, DARK, "README.md"];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 3] {
    [
        (LIGHT, stylesheet(&theme.light, &theme.typography, false)),
        (DARK, stylesheet(&theme.dark, &theme.typography, true)),
        (
            "README.md",
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Docs/07-typora.md")).into(),
        ),
    ]
}

fn stylesheet(mode: &ResolvedMode, typography: &Typography, dark: bool) -> String {
    let mut css = String::from("/* LBRX — generated from the theme source. */\n:root {\n");
    for role in [
        "background",
        "text",
        "heading",
        "link",
        "quote",
        "inline_code",
        "muted",
        "focus",
        "selection_background",
        "selection_text",
        "comment",
        "keyword",
        "string",
        "function",
        "number",
        "error",
    ] {
        writeln!(
            css,
            "  --lbrx-{}: {};",
            role.replace('_', "-"),
            mode.roles[role]
        )
        .unwrap();
    }
    if dark {
        writeln!(css, "  --lbrx-surface: {};", mode.roles["surface"]).unwrap();
    }
    css.push_str("  --bg-color: var(--lbrx-background);\n  --text-color: var(--lbrx-text);\n");
    for (name, font) in [("prose", &typography.prose), ("mono", &typography.mono)] {
        let stack = font_stack(font);
        writeln!(css, "  --lbrx-{name}-family: {stack};").unwrap();
        writeln!(css, "  --lbrx-{name}-size: {}px;", font.size_px).unwrap();
        writeln!(css, "  --lbrx-{name}-leading: {};", font.line_height).unwrap();
    }
    css.push_str("}\n");
    css.push_str(include_str!("typora.css"));
    if dark {
        css.push_str(include_str!("typora-preferences-dark.css"));
    } else {
        css.push_str(include_str!("typora-preferences-light.css"));
    }
    css
}
