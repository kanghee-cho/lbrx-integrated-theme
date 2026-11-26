use crate::{
    css::font_stack,
    resolve::{ResolvedMode, ResolvedTheme},
};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "lbrx-studio";
pub const FILES: &[&str] = &["theme.css", crate::build::GUIDE];
pub const COLOR_KEYS: &[&str] = &[
    "--background",
    "--surface",
    "--text",
    "--muted",
    "--accent",
    "--selection-background",
    "--selection-text",
    "--error",
    "--warning",
    "--success",
    "--border",
    "--focus",
    "--link",
];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 2] {
    let mut css = format!(
        "/* LBRX Studio - generated; edit themes/<style>/editors/lbrx-studio.toml instead.\n{}*/\n",
        include_str!("../LICENSE")
    );
    css.push_str(":root {\n");
    for (name, font) in [
        ("prose", &theme.typography.prose),
        ("mono", &theme.typography.mono),
    ] {
        writeln!(
            css,
            "  --font-{name}: {};\n  --font-{name}-size: {}px;\n  --font-{name}-line-height: {};",
            font_stack(font),
            font.size_px,
            font.line_height
        )
        .unwrap();
    }
    css.push_str("}\n");
    colors(&mut css, theme, &theme.light, ":root", "light");
    colors(
        &mut css,
        theme,
        &theme.dark,
        ":root[data-theme=\"dark\"]",
        "dark",
    );
    css.push_str(&crate::css::code_css(
        theme,
        &[
            (":root .lbrx-code", &theme.light),
            (":root[data-theme=\"dark\"] .lbrx-code", &theme.dark),
        ],
    ));
    css.push_str("@media (prefers-color-scheme: dark) {\n");
    let system = ":root:not([data-theme=\"light\"]):not([data-theme=\"dark\"])";
    colors(&mut css, theme, &theme.dark, system, "dark");
    css.push_str(&crate::css::code_css(
        theme,
        &[(&format!("{system} .lbrx-code"), &theme.dark)],
    ));
    css.push_str("}\n");
    css.push_str(
        ".lbrx-code {\n  font-family: var(--font-mono);\n  font-size: var(--font-mono-size);\n  line-height: var(--font-mono-line-height);\n}\n",
    );
    [
        (FILES[0], css),
        (FILES[1], include_str!("../Docs/25-lbrx-studio.md").into()),
    ]
}

fn colors(
    css: &mut String,
    theme: &ResolvedTheme,
    mode: &ResolvedMode,
    selector: &str,
    scheme: &str,
) {
    writeln!(css, "{selector} {{\n  color-scheme: {scheme};").unwrap();
    for &key in COLOR_KEYS {
        let fallback = if key == "--border" {
            "muted".into()
        } else {
            key.trim_start_matches("--").replace('-', "_")
        };
        let color = theme.editor_color(mode, DIRECTORY, key, &mode.roles[&fallback]);
        let alpha = if key == "--border" { "40" } else { "" };
        writeln!(css, "  {key}: {color}{alpha};").unwrap();
    }
    css.push_str("}\n");
}
