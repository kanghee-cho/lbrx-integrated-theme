use crate::model::FontStyle;
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
