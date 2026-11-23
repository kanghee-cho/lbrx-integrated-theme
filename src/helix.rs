use crate::resolve::{ResolvedMode, ResolvedTheme};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "helix";
pub const FILES: &[&str; 2] = &["lbrx_light.toml", "lbrx_dark.toml"];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 2] {
    [
        (FILES[0], colorscheme(theme, &theme.light)),
        (FILES[1], colorscheme(theme, &theme.dark)),
    ]
}

fn colorscheme(theme: &ResolvedTheme, mode: &ResolvedMode) -> String {
    let color = |role: &str| &mode.roles[role];
    let mut out = String::from("# LBRX - generated from the theme source.\n");
    for (scope, role) in [
        ("comment", "comment"),
        ("keyword", "keyword"),
        ("string", "string"),
        ("function", "function"),
        ("constant.numeric", "number"),
        ("markup.heading", "heading"),
        ("markup.link.url", "link"),
        ("markup.quote", "quote"),
        ("markup.raw.inline", "inline_code"),
        ("ui.text", "text"),
        ("ui.linenr", "muted"),
        ("ui.linenr.selected", "text"),
        ("warning", "warning"),
        ("error", "error"),
    ] {
        let fg = if matches!(
            scope,
            "comment" | "keyword" | "string" | "function" | "constant.numeric"
        ) {
            theme.token(mode, "", role)
        } else {
            color(role)
        };
        writeln!(
            out,
            "\"{scope}\" = \"{}\"",
            theme.editor_color(mode, DIRECTORY, scope, fg)
        )
        .unwrap();
    }
    writeln!(
        out,
        "\"ui.background\" = {{ bg = \"{}\" }}",
        color("background")
    )
    .unwrap();
    for (scope, fg, bg) in [
        ("ui.cursor", "background", "focus"),
        ("ui.cursor.primary", "background", "focus"),
        ("ui.selection", "selection_text", "selection_background"),
        (
            "ui.selection.primary",
            "selection_text",
            "selection_background",
        ),
        ("ui.statusline", "text", "surface"),
        ("ui.statusline.inactive", "muted", "surface"),
        ("ui.popup", "text", "surface"),
        ("ui.menu", "text", "surface"),
        ("ui.menu.selected", "selection_text", "selection_background"),
    ] {
        writeln!(
            out,
            "\"{scope}\" = {{ fg = \"{}\", bg = \"{}\" }}",
            color(fg),
            color(bg)
        )
        .unwrap();
    }
    for severity in ["warning", "error"] {
        writeln!(
            out,
            "\"diagnostic.{severity}\" = {{ underline = {{ color = \"{}\", style = \"curl\" }} }}",
            color(severity)
        )
        .unwrap();
    }
    out
}
