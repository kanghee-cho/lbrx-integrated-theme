use crate::{
    css::font_stack,
    resolve::{ResolvedMode, ResolvedTheme},
};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "typora";
pub const LIGHT: &str = "lbrx-light.css";
pub const DARK: &str = "lbrx-dark.css";
pub const FILES: &[&str] = &[LIGHT, DARK, "README.md"];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 3] {
    [
        (LIGHT, stylesheet(theme, &theme.light, false)),
        (DARK, stylesheet(theme, &theme.dark, true)),
        (
            "README.md",
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Docs/07-typora.md")).into(),
        ),
    ]
}

fn stylesheet(theme: &ResolvedTheme, mode: &ResolvedMode, dark: bool) -> String {
    let typography = &theme.typography;
    let mut css = String::from("/* LBRX — generated from the theme source. */\n:root {\n");
    writeln!(
        css,
        "  --lbrx-code-block-background: {};",
        mode.code_block_background()
    )
    .unwrap();
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
        let key = format!("--lbrx-{}", role.replace('_', "-"));
        let fg = theme.token(mode, "", role);
        writeln!(
            css,
            "  {key}: {};",
            theme.editor_color(mode, DIRECTORY, &key, fg)
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
    for (lang, tokens) in &theme.layers.languages {
        for token in tokens.keys() {
            writeln!(
                css,
                "#write .md-fences[lang=\"{lang}\"] {{ --lbrx-{token}: {}; }}",
                theme.token(mode, lang, token)
            )
            .unwrap();
        }
    }
    if dark {
        css.push_str(include_str!("typora-preferences-dark.css"));
    } else {
        css.push_str(include_str!("typora-preferences-light.css"));
    }
    css
}
