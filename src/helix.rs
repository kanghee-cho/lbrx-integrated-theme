use crate::resolve::{ResolvedMode, ResolvedTheme};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "helix";
pub const FILES: &[&str] = &["lbrx_light.toml", "lbrx_dark.toml", crate::build::GUIDE];

pub(crate) fn is_supported_scope(scope: &str) -> bool {
    include_str!("../Docs/17-helix-options.md")
        .lines()
        .filter_map(|line| line.strip_prefix("| `")?.split_once("` | "))
        .any(|(name, rest)| {
            name == scope
                && (rest.starts_with("공통 | ")
                    || rest.starts_with("합일 후보 | ")
                    || rest.starts_with("Helix 전용 | "))
        })
}

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, String); 3], String> {
    Ok([
        (FILES[0], colorscheme(theme, &theme.light)?),
        (FILES[1], colorscheme(theme, &theme.dark)?),
        (
            crate::build::GUIDE,
            include_str!("../Docs/16-helix.md").into(),
        ),
    ])
}

fn colorscheme(theme: &ResolvedTheme, mode: &ResolvedMode) -> Result<String, String> {
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
    if theme.layers.helix_styles.is_empty() {
        return Ok(out);
    }
    apply_styles(&out, theme, mode).map_err(|err| format!("Helix 스타일 생성 실패: {err}"))
}

fn apply_styles(
    base: &str,
    theme: &ResolvedTheme,
    mode: &ResolvedMode,
) -> Result<String, Box<dyn std::error::Error>> {
    use toml::{Table, Value};

    let mut scopes: Table = toml::from_str(base)?;
    let color = |role: &str| {
        Value::String(if role == "default" {
            role.to_owned()
        } else {
            mode.roles[role].clone()
        })
    };
    for (scope, style) in &theme.layers.helix_styles {
        let mut name = scope.as_str();
        let mut entry = loop {
            if let Some(value) = scopes.get(name) {
                break match value {
                    Value::String(fg) => {
                        Table::from_iter([("fg".into(), Value::String(fg.clone()))])
                    }
                    Value::Table(fields) => fields.clone(),
                    _ => return Err(format!("잘못된 스코프 '{name}'").into()),
                };
            }
            let Some((parent, _)) = name.rsplit_once('.') else {
                break Table::new();
            };
            name = parent;
        };
        for (field, role) in [("fg", &style.fg), ("bg", &style.bg)] {
            if let Some(role) = role {
                entry.insert(field.into(), color(role));
            }
        }
        if let Some(modifiers) = &style.modifiers {
            entry.insert("modifiers".into(), Value::try_from(modifiers)?);
        }
        if let Some(underline) = &style.underline {
            let mut fields = match entry.remove("underline") {
                Some(Value::Table(fields)) => fields,
                _ => Table::new(),
            };
            if let Some(role) = &underline.color {
                fields.insert("color".into(), color(role));
            }
            if let Some(style) = &underline.style {
                fields.insert("style".into(), Value::try_from(style)?);
            }
            entry.insert("underline".into(), Value::Table(fields));
        }
        scopes.insert(scope.clone(), Value::Table(entry));
    }
    Ok(format!(
        "# LBRX - generated from the theme source.\n{}",
        toml::to_string_pretty(&scopes)?
    ))
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
