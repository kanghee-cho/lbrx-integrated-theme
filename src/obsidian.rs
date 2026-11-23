use crate::{
    css::font_stack,
    resolve::{ResolvedTheme, parse_rgb},
};
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
        let [hue, saturation, lightness] =
            rgb_to_hsl(parse_rgb(&mode.roles["accent"]).expect("resolved accent is valid RGB"));
        writeln!(css, "  --accent-h: {hue:.8};").unwrap();
        writeln!(css, "  --accent-s: {saturation:.8}%;").unwrap();
        writeln!(css, "  --accent-l: {lightness:.8}%;").unwrap();
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
            let fg = theme.token(mode, "", role);
            writeln!(
                css,
                "  --{variable}: {};",
                theme.editor_color(mode, "obsidian", &format!("--{variable}"), fg)
            )
            .unwrap();
        }
        writeln!(
            css,
            "  --lbrx-code-block-background: {};",
            mode.code_block_background()
        )
        .unwrap();
        css.push_str("}\n");
        for token in ["number", "error"] {
            if theme.layers.common.contains_key(token) {
                writeln!(
                    css,
                    ".theme-{class} .markdown-rendered pre code .token.{token} {{ color: {}; }}",
                    theme.token(mode, "", token)
                )
                .unwrap();
            }
        }
        for (lang, tokens) in &theme.layers.languages {
            for token in tokens.keys() {
                writeln!(css, ".theme-{class} .markdown-rendered code.language-{lang} .token.{token} {{ color: {}; }}",
                    theme.token(mode, lang, token)).unwrap();
            }
        }
    }
    css.push_str(concat!(
        ".markdown-rendered pre,\n",
        ".markdown-source-view.is-live-preview .HyperMD-codeblock {\n",
        "  --code-background: var(--lbrx-code-block-background);\n",
        "  background-color: var(--code-background);\n",
        "}\n"
    ));
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

fn rgb_to_hsl(rgb: [u8; 3]) -> [f64; 3] {
    let [r, g, b] = rgb.map(|channel| f64::from(channel) / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (max + min) / 2.0;
    let delta = max - min;
    if delta == 0.0 {
        return [0.0, 0.0, lightness * 100.0];
    }
    let sector = if max == r {
        ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());
    [sector * 60.0, saturation * 100.0, lightness * 100.0]
}
