use crate::model::FontStyle;
use crate::resolve::{ResolvedMode, ResolvedTheme};
use std::fmt::Write as _;

pub fn font_stack(font: &FontStyle) -> String {
    font.families
        .iter()
        .map(|name| {
            let name = name.trim();
            if matches!(
                name.to_ascii_lowercase().as_str(),
                "serif"
                    | "sans-serif"
                    | "monospace"
                    | "cursive"
                    | "fantasy"
                    | "system-ui"
                    | "ui-serif"
                    | "ui-sans-serif"
                    | "ui-monospace"
                    | "ui-rounded"
                    | "emoji"
                    | "math"
                    | "fangsong"
            ) {
                return name.to_ascii_lowercase();
            }
            let mut quoted = String::from("\"");
            for ch in name.chars() {
                match ch {
                    '"' | '\\' => {
                        quoted.push('\\');
                        quoted.push(ch);
                    }
                    '\0' => quoted.push('\u{fffd}'),
                    ch if ch.is_control() => write!(quoted, "\\{:x} ", u32::from(ch)).unwrap(),
                    ch => quoted.push(ch),
                }
            }
            quoted.push('"');
            quoted
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn code_css(theme: &ResolvedTheme, modes: &[(&str, &ResolvedMode)]) -> String {
    let mut css = String::from("/* LBRX - generated from the theme source. */\n");
    for &(selector, mode) in modes {
        writeln!(css, "{selector} {{").unwrap();
        writeln!(
            css,
            "  background-color: {};\n  color: {};",
            mode.roles["background"], mode.roles["text"]
        )
        .unwrap();
        for &role in crate::layers::TOKENS {
            let slug = role.replace('.', "-");
            writeln!(
                css,
                "  --lbrx-code-{slug}: {};",
                theme.token(mode, "", role),
            )
            .unwrap();
            let style = theme.token_style("", role);
            let weight = if style.name() == "bold" {
                "bold"
            } else {
                "normal"
            };
            let italic = if style.name() == "italic" {
                "italic"
            } else {
                "normal"
            };
            writeln!(
                css,
                "  --lbrx-code-{slug}-weight: {weight};\n  --lbrx-code-{slug}-style: {italic};"
            )
            .unwrap();
        }
        css.push_str("}\n");
        let container = selector.split(", ").next().unwrap();
        for lang in theme.layers.languages.keys() {
            for &token in crate::layers::TOKENS
                .iter()
                .filter(|&&token| theme.has_token_override(lang, token))
            {
                let slug = token.replace('.', "-");
                let style = theme.token_style(lang, token);
                let weight = if style.name() == "bold" {
                    "bold"
                } else {
                    "normal"
                };
                let italic = if style.name() == "italic" {
                    "italic"
                } else {
                    "normal"
                };
                writeln!(
                    css,
                    "{container} .language-{lang}, {container}.language-{lang} {{ --lbrx-code-{slug}: {}; --lbrx-code-{slug}-weight: {weight}; --lbrx-code-{slug}-style: {italic}; }}",
                    theme.token(mode, lang, token)
                )
                .unwrap();
            }
        }
    }
    for &role in crate::layers::TOKENS {
        let slug = role.replace('.', "-");
        writeln!(
            css,
            ".lbrx-code .tok-{slug} {{ color: var(--lbrx-code-{slug}); font-weight: var(--lbrx-code-{slug}-weight); font-style: var(--lbrx-code-{slug}-style); }}"
        )
        .unwrap();
    }
    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_family_names_are_css_strings_not_declarations() {
        let font = FontStyle {
            families: vec!["Name\";\\\nNext".into(), "SANS-SERIF".into()],
            size_px: 16.0,
            line_height: 1.65,
        };
        assert_eq!(font_stack(&font), "\"Name\\\";\\\\\\a Next\", sans-serif");
    }
}
