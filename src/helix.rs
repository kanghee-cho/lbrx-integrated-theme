use crate::resolve::{ResolvedMode, ResolvedTheme};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "helix";
pub const FILES: &[&str] = &["lbrx_light.toml", "lbrx_dark.toml", crate::build::GUIDE];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 3] {
    [
        (FILES[0], colorscheme(theme, &theme.light)),
        (FILES[1], colorscheme(theme, &theme.dark)),
        (
            crate::build::GUIDE,
            include_str!("../Docs/16-helix.md").into(),
        ),
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
        let fg = theme.editor_color(mode, DIRECTORY, scope, fg);
        let style = if scope == "comment" {
            theme.token_style("", role).name()
        } else {
            "normal"
        };
        write_scope(&mut out, scope, fg, style);
    }
    for (scope, token) in [
        ("keyword.control", "keyword.control"),
        ("keyword.function", "keyword.declaration"),
        ("keyword.storage", "keyword.declaration"),
        ("string.regexp", "string.escape"),
        ("constant.character.escape", "string.escape"),
        ("constant.builtin", "constant.builtin"),
        ("function.macro", "function.macro"),
        ("type", "type"),
        ("variable", "variable"),
        ("variable.parameter", "variable.parameter"),
        ("variable.other.member", "property"),
        ("operator", "operator"),
        ("attribute", "attribute"),
        ("punctuation", "punctuation"),
    ] {
        write_scope(
            &mut out,
            scope,
            theme.token(mode, "", token),
            theme.token_style("", token).name(),
        );
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

fn write_scope(out: &mut String, scope: &str, fg: &str, style: &str) {
    if style == "normal" {
        writeln!(out, "\"{scope}\" = \"{fg}\"").unwrap();
    } else {
        writeln!(
            out,
            "\"{scope}\" = {{ fg = \"{fg}\", modifiers = [\"{style}\"] }}"
        )
        .unwrap();
    }
}
