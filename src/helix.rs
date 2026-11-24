use crate::resolve::{ResolvedMode, ResolvedTheme};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};

pub const DIRECTORY: &str = "helix";
pub const FILES: &[&str] = &[
    "lbrx_light.toml",
    "lbrx_dark.toml",
    "config.toml",
    "languages.toml",
    crate::build::GUIDE,
];

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

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, String); 5], String> {
    Ok([
        (FILES[0], colorscheme(theme, &theme.light)?),
        (FILES[1], colorscheme(theme, &theme.dark)?),
        (
            FILES[2],
            render_config(FILES[2], &theme.layers.helix_config)?,
        ),
        (FILES[3], render_config(FILES[3], &language_config(theme)?)?),
        (
            crate::build::GUIDE,
            include_str!("../Docs/16-helix.md").into(),
        ),
    ])
}

fn language_config(theme: &ResolvedTheme) -> Result<toml::Table, String> {
    use toml::Value;
    let catalog: BTreeMap<String, toml::Table> =
        toml::from_str(include_str!("helix-language-defaults.toml"))
            .map_err(|err| format!("Helix 언어 기본값 읽기 실패: {err}"))?;
    let mut defaults = BTreeMap::new();
    for id in theme.language_ids() {
        let name = match id {
            "csharp" => "c-sharp",
            "shell" => "bash",
            _ => id,
        };
        if let Some(settings) = catalog.get(name).filter(|settings| !settings.is_empty()) {
            defaults.entry(name).or_insert_with(|| settings.clone());
        }
    }

    let mut config = theme.layers.helix_languages.clone();
    let mut languages = Vec::new();
    if let Some(entries) = config.remove("language") {
        let Value::Array(entries) = entries else {
            return Err("Helix: [[languages.language]]는 언어 표의 배열이어야 합니다".into());
        };
        let mut names = BTreeSet::new();
        for (index, entry) in entries.into_iter().enumerate() {
            let Value::Table(mut user) = entry else {
                return Err(format!(
                    "Helix: languages.language[{index}]는 표여야 합니다"
                ));
            };
            let name = user
                .get("name")
                .and_then(Value::as_str)
                .filter(|name| !name.trim().is_empty())
                .ok_or_else(|| format!("Helix: languages.language[{index}].name이 필요합니다"))?
                .to_owned();
            if !names.insert(name.clone()) {
                return Err(format!("Helix: 언어 '{name}' 설정이 중복되었습니다"));
            }
            let mut settings = defaults.remove(name.as_str()).unwrap_or_default();
            if user.contains_key("comment-token") && user.contains_key("comment-tokens") {
                return Err(format!(
                    "Helix: 언어 '{name}'의 줄 주석 키는 하나만 지정하세요"
                ));
            }
            // Preserve the built-in key spelling to avoid duplicate aliases after Helix merges.
            for (native, alternate) in [
                ("comment-token", "comment-tokens"),
                ("comment-tokens", "comment-token"),
            ] {
                if settings.contains_key(native) {
                    if let Some(value) = user.remove(alternate) {
                        user.insert(native.into(), value);
                    }
                }
            }
            if let (Some(Value::Table(default_indent)), Some(Value::Table(user_indent))) =
                (settings.get_mut("indent"), user.get("indent"))
            {
                default_indent.extend(user_indent.clone());
                user.remove("indent");
            }
            settings.extend(user);
            languages.push(Value::Table(settings));
        }
    }
    for (name, mut settings) in defaults {
        settings.insert("name".into(), Value::String(name.into()));
        languages.push(Value::Table(settings));
    }
    if !languages.is_empty() {
        config.insert("language".into(), Value::Array(languages));
    }
    Ok(config)
}

fn render_config(filename: &str, config: &toml::Table) -> Result<String, String> {
    let settings = toml::to_string_pretty(config)
        .map_err(|err| format!("Helix {filename} 생성 실패: {err}"))?;
    Ok(format!(
        "# LBRX - 사용자 설정과 비교한 뒤 적용하세요.\n{settings}"
    ))
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
