use crate::model::{FontStyle, Mode, Theme, Typography};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const CODE_BLOCK_BACKGROUND: &str = "code_block_background";
const PROSE_SELECTION_BACKGROUND: &str = "prose_selection_background";
const ANSI_SLOTS: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
];
const REQUIRED_ROLES: &[&str] = &[
    "background",
    "surface",
    "text",
    "accent",
    "muted",
    "focus",
    "selection_text",
    "selection_background",
    "heading",
    "link",
    "quote",
    "inline_code",
    "comment",
    "string",
    "keyword",
    "function",
    "number",
    "error",
    "warning",
    "success",
    "terminal_foreground",
    "terminal_background",
    "terminal_cursor",
];

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedTheme {
    #[serde(default)]
    pub layers: crate::layers::Layers,
    pub schema_version: u32,
    pub tool_version: String,
    pub light: ResolvedMode,
    pub dark: ResolvedMode,
    pub typography: Typography,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedMode {
    pub roles: BTreeMap<String, String>,
    pub ansi: [String; 16],
}

impl ResolvedMode {
    pub fn prose_selection_background(&self) -> &str {
        self.roles
            .get(PROSE_SELECTION_BACKGROUND)
            .unwrap_or(&self.roles["selection_background"])
    }

    pub fn code_block_background_role(&self) -> &'static str {
        if self.roles.contains_key(CODE_BLOCK_BACKGROUND) {
            CODE_BLOCK_BACKGROUND
        } else {
            "background"
        }
    }

    pub fn code_block_background(&self) -> &str {
        &self.roles[self.code_block_background_role()]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorSource {
    Language,
    Common,
    Base,
}

impl ResolvedTheme {
    pub fn token_role<'a>(&'a self, lang: &str, token: &'a str) -> (&'a str, ColorSource) {
        if let Some(role) = self.layers.languages.get(lang).and_then(|m| m.get(token)) {
            (role, ColorSource::Language)
        } else if let Some(role) = self.layers.common.get(token) {
            (role, ColorSource::Common)
        } else if let Some(parent) = token_parent(token) {
            self.token_role(lang, parent)
        } else {
            (token, ColorSource::Base)
        }
    }

    pub fn token<'a>(&self, mode: &'a ResolvedMode, lang: &str, token: &str) -> &'a str {
        &mode.roles[self.token_role(lang, token).0]
    }

    pub fn token_style(&self, lang: &str, token: &str) -> crate::layers::TokenStyle {
        self.layers
            .language_styles
            .get(lang)
            .and_then(|styles| styles.get(token))
            .or_else(|| self.layers.common_styles.get(token))
            .copied()
            .unwrap_or_default()
    }

    pub fn has_token_override(&self, lang: &str, token: &str) -> bool {
        self.token_role(lang, token).1 == ColorSource::Language
            || self
                .layers
                .language_styles
                .get(lang)
                .is_some_and(|styles| styles.contains_key(token))
    }

    pub fn language_ids(&self) -> impl Iterator<Item = &str> {
        self.layers.languages.keys().map(String::as_str)
    }

    pub fn editor_role<'a>(&'a self, app: &str, key: &str) -> Option<&'a str> {
        self.layers.editors.get(app)?.get(key).map(String::as_str)
    }

    pub fn editor_color<'a>(
        &self,
        mode: &'a ResolvedMode,
        app: &str,
        key: &str,
        fallback: &'a str,
    ) -> &'a str {
        self.editor_role(app, key)
            .map_or(fallback, |role| &mode.roles[role])
    }
}

fn token_parent(token: &str) -> Option<&'static str> {
    match token {
        "keyword.control" | "keyword.declaration" => Some("keyword"),
        "string.escape" => Some("string"),
        "function.macro" => Some("function"),
        "variable.parameter" => Some("variable"),
        "constant.builtin" | "type" | "variable" | "property" | "operator" | "attribute"
        | "punctuation" => Some("text"),
        _ => None,
    }
}

pub fn resolve(theme: &Theme) -> Result<ResolvedTheme, String> {
    check(theme)?;
    let mode = |mode: &Mode| ResolvedMode {
        roles: mode
            .roles
            .iter()
            .map(|(key, reference)| (key.clone(), mode.palette[reference].to_ascii_uppercase()))
            .collect(),
        ansi: std::array::from_fn(|index| {
            mode.palette[&mode.ansi[ANSI_SLOTS[index]]].to_ascii_uppercase()
        }),
    };
    Ok(ResolvedTheme {
        layers: theme.layers.clone(),
        schema_version: theme.schema_version,
        tool_version: env!("CARGO_PKG_VERSION").into(),
        light: mode(&theme.light),
        dark: mode(&theme.dark),
        typography: theme.typography.clone(),
    })
}

pub fn check(theme: &Theme) -> Result<(), String> {
    if theme.schema_version != 1 {
        return Err(format!(
            "schema_version: 지원하지 않는 값 {}",
            theme.schema_version
        ));
    }
    check_mode("light", &theme.light)?;
    check_mode("dark", &theme.dark)?;
    for (name, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
        let luminance_of = |role: &str| {
            luminance(parse_rgb(&mode.palette[&mode.roles[role]]).expect("checked RGB"))
        };
        if let Some(callouts) = &theme.layers.obsidian_callouts {
            if callouts.shaded_background {
                for (kind, fallback) in crate::layers::DOCUMENT_ALERT_ROLES {
                    let role = callouts.role(kind, fallback);
                    check_contrast(
                        name,
                        role,
                        "surface",
                        luminance_of(role),
                        luminance_of("surface"),
                        4.5,
                    )
                    .map_err(|error| {
                        format!("editors/obsidian.toml: callouts.colors.{kind}: {error}")
                    })?;
                }
            }
        }
        if let Some(role) = theme
            .layers
            .editors
            .get("obsidian")
            .and_then(|colors| colors.get(crate::obsidian::INLINE_CODE_COLOR))
        {
            check_contrast(
                name,
                role,
                "background",
                luminance_of(role),
                luminance_of("background"),
                4.5,
            )
            .map_err(|error| {
                format!(
                    "editors/obsidian.toml: colors.{}: {error}",
                    crate::obsidian::INLINE_CODE_COLOR
                )
            })?;
        }
        for (app, emphasis) in [
            ("typora", &theme.layers.typora_emphasis),
            ("obsidian", &theme.layers.obsidian_emphasis),
        ] {
            let Some(emphasis) = emphasis else { continue };
            let colors = if name == "light" {
                &emphasis.light
            } else {
                &emphasis.dark
            };
            let emphasis_luminance = |key: &str, value: &str| {
                parse_rgb(value).map(luminance).ok_or_else(|| {
                    format!("editors/{app}.toml: emphasis.{name}.{key}: #RRGGBB 형식 필요")
                })
            };
            let highlight =
                emphasis_luminance("highlight_background", &colors.highlight_background)?;
            let highlight_key = format!("editors/{app}.toml: emphasis.highlight_background");
            check_contrast(
                name,
                "text",
                &highlight_key,
                luminance_of("text"),
                highlight,
                4.5,
            )?;
            for (key, color) in [
                ("strong", &colors.strong),
                ("italic", &colors.italic),
                ("strikethrough", &colors.strikethrough),
            ] {
                let fg = emphasis_luminance(key, color)?;
                let label = format!("editors/{app}.toml: emphasis.{key}");
                check_contrast(
                    name,
                    &label,
                    "background",
                    fg,
                    luminance_of("background"),
                    4.5,
                )?;
                check_contrast(name, &label, &highlight_key, fg, highlight, 4.5)?;
            }
        }
        let code_bg = mode
            .roles
            .contains_key(CODE_BLOCK_BACKGROUND)
            .then(|| luminance_of(CODE_BLOCK_BACKGROUND));
        if let Some(bg) = code_bg {
            for token in std::iter::once("text").chain(crate::layers::TOKENS.iter().copied()) {
                let mut fallback = token;
                let role = loop {
                    if let Some(role) = theme.layers.common.get(fallback) {
                        break role.as_str();
                    }
                    if let Some(parent) = token_parent(fallback) {
                        fallback = parent;
                    } else {
                        break fallback;
                    }
                };
                check_contrast(
                    name,
                    token,
                    CODE_BLOCK_BACKGROUND,
                    luminance_of(role),
                    bg,
                    4.5,
                )?;
            }
        }
        for (language, tokens) in std::iter::once(("common", &theme.layers.common))
            .chain(theme.layers.languages.iter().map(|(k, v)| (k.as_str(), v)))
        {
            for (token, role) in tokens {
                check_contrast(
                    name,
                    &format!("languages/{language}.toml: {token} ({role})"),
                    "background",
                    luminance_of(role),
                    luminance_of("background"),
                    4.5,
                )?;
                if let Some(bg) = code_bg.filter(|_| language != "common") {
                    check_contrast(
                        name,
                        &format!("languages/{language}.toml: {token} ({role})"),
                        CODE_BLOCK_BACKGROUND,
                        luminance_of(role),
                        bg,
                        4.5,
                    )?;
                }
            }
        }
    }
    check_font("prose", &theme.typography.prose)?;
    check_font("mono", &theme.typography.mono)?;
    for (name, this, other) in [
        ("light", &theme.light, &theme.dark),
        ("dark", &theme.dark, &theme.light),
    ] {
        for (field, own, required) in [
            ("palette", &this.palette, &other.palette),
            ("roles", &this.roles, &other.roles),
        ] {
            if let Some(key) = required.keys().find(|k| !own.contains_key(*k)) {
                return Err(format!(
                    "{name}.{field}.{key}: 반대 모드에 있는 키가 누락됨"
                ));
            }
        }
    }
    Ok(())
}

fn check_mode(name: &str, mode: &Mode) -> Result<(), String> {
    let palette = mode
        .palette
        .iter()
        .map(|(key, value)| {
            let rgb = parse_rgb(value)
                .ok_or_else(|| format!("{name}.palette.{key}: #RRGGBB 형식 필요"))?;
            Ok((key.as_str(), luminance(rgb)))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for &key in REQUIRED_ROLES {
        if !mode.roles.contains_key(key) {
            return Err(format!("{name}.roles.{key}: 필수 역할 누락"));
        }
    }
    let mut roles = BTreeMap::new();
    for (key, reference) in &mode.roles {
        let value = palette
            .get(reference.as_str())
            .ok_or_else(|| format!("{name}.roles.{key}: 팔레트에 '{reference}' 없음"))?;
        roles.insert(key.as_str(), *value);
    }
    for (slot, reference) in &mode.ansi {
        if !ANSI_SLOTS.contains(&slot.as_str()) {
            return Err(format!("{name}.ansi.{slot}: 슬롯 키는 0–15만 허용"));
        }
        if !palette.contains_key(reference.as_str()) {
            return Err(format!("{name}.ansi.{slot}: 팔레트에 '{reference}' 없음"));
        }
    }
    for slot in ANSI_SLOTS {
        if !mode.ansi.contains_key(slot) {
            return Err(format!("{name}.ansi.{slot}: 필수 ANSI 슬롯 누락"));
        }
    }

    check_contrast(
        name,
        "text",
        "background",
        roles["text"],
        roles["background"],
        7.0,
    )?;
    for foreground in [
        "muted",
        "heading",
        "link",
        "quote",
        "inline_code",
        "comment",
        "string",
        "keyword",
        "function",
        "number",
        "error",
        "warning",
        "success",
    ] {
        check_contrast(
            name,
            foreground,
            "background",
            roles[foreground],
            roles["background"],
            4.5,
        )?;
    }
    check_contrast(
        name,
        "selection_text",
        "selection_background",
        roles["selection_text"],
        roles["selection_background"],
        4.5,
    )?;
    if let Some(&bg) = roles.get(PROSE_SELECTION_BACKGROUND) {
        check_contrast(
            name,
            "selection_text",
            PROSE_SELECTION_BACKGROUND,
            roles["selection_text"],
            bg,
            4.5,
        )?;
    }
    for adjacent in ["background", "surface"] {
        check_contrast(
            name,
            "focus",
            adjacent,
            roles["focus"],
            roles[adjacent],
            3.0,
        )?;
    }
    Ok(())
}

fn check_font(kind: &str, font: &FontStyle) -> Result<(), String> {
    if font.families.is_empty() {
        return Err(format!("typography.{kind}.families: 글꼴이 하나 이상 필요"));
    }
    if let Some(index) = font
        .families
        .iter()
        .position(|family| family.trim().is_empty())
    {
        return Err(format!("typography.{kind}.families[{index}]: 빈 글꼴 이름"));
    }
    for (key, value) in [("size_px", font.size_px), ("line_height", font.line_height)] {
        if !value.is_finite() || value <= 0.0 {
            return Err(format!("typography.{kind}.{key}: 유한한 양수가 필요"));
        }
    }
    Ok(())
}

pub(crate) fn parse_rgb(value: &str) -> Option<[u8; 3]> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some([
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ])
}

fn luminance(rgb: [u8; 3]) -> f64 {
    let [red, green, blue] = rgb.map(|channel| {
        let srgb = f64::from(channel) / 255.0;
        if srgb <= 0.04045 {
            srgb / 12.92
        } else {
            ((srgb + 0.055) / 1.055).powf(2.4)
        }
    });
    0.2126 * red + 0.7152 * green + 0.0722 * blue
}

fn check_contrast(
    mode: &str,
    foreground: &str,
    background: &str,
    fg: f64,
    bg: f64,
    minimum: f64,
) -> Result<(), String> {
    let ratio = luminance_ratio(fg, bg);
    if ratio < minimum {
        return Err(format!(
            "{mode}.roles.{foreground} / {mode}.roles.{background}: 대비 {ratio}:1 (최소 {minimum}:1)"
        ));
    }
    Ok(())
}

fn luminance_ratio(fg: f64, bg: f64) -> f64 {
    (fg.max(bg) + 0.05) / (fg.min(bg) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Theme {
        toml::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/themes/lbrx.toml"
        )))
        .unwrap()
    }

    #[test]
    fn sample_passes() {
        assert!(check(&sample()).is_ok());
    }

    #[test]
    fn reference_must_resolve_in_its_own_mode() {
        let mut theme = sample();
        theme.dark.roles.insert("link".into(), "missing".into());
        assert!(check(&theme).unwrap_err().contains("dark.roles.link"));
    }

    #[test]
    fn rejects_non_ascii_hex() {
        let mut theme = sample();
        theme.light.palette.insert("ink".into(), "#１２３".into());
        assert!(check(&theme).unwrap_err().contains("light.palette.ink"));
    }

    #[test]
    fn extra_role_requires_counterpart() {
        let mut theme = sample();
        theme.light.roles.insert("info".into(), "ink".into());
        assert!(check(&theme).unwrap_err().contains("dark.roles.info"));
    }

    #[test]
    fn ansi_reports_missing_extra_and_unknown_reference() {
        let mut theme = sample();
        let color = theme.dark.ansi.remove("15").unwrap();
        assert!(check(&theme).unwrap_err().contains("dark.ansi.15"));
        theme.dark.ansi.insert("15".into(), color.clone());
        theme.dark.ansi.insert("16".into(), color);
        assert!(check(&theme).unwrap_err().contains("dark.ansi.16"));
        theme.dark.ansi.remove("16");
        theme.dark.ansi.insert("15".into(), "missing".into());
        assert!(check(&theme).unwrap_err().contains("dark.ansi.15"));
    }

    #[test]
    fn typography_rejects_empty_families_and_invalid_dimensions() {
        let mut theme = sample();
        theme.typography.mono.families.push(" \t".into());
        assert!(
            check(&theme)
                .unwrap_err()
                .contains("typography.mono.families[2]")
        );
        theme.typography.mono.families.clear();
        assert!(
            check(&theme)
                .unwrap_err()
                .contains("typography.mono.families")
        );
        theme = sample();
        theme.typography.prose.size_px = 0.0;
        assert!(
            check(&theme)
                .unwrap_err()
                .contains("typography.prose.size_px")
        );
        theme = sample();
        theme.typography.prose.line_height = f64::NAN;
        assert!(
            check(&theme)
                .unwrap_err()
                .contains("typography.prose.line_height")
        );
    }

    #[test]
    fn selection_is_checked_against_selection_background() {
        let mut theme = sample();
        theme
            .dark
            .roles
            .insert("selection_background".into(), "ink".into());
        let error = check(&theme).unwrap_err();
        assert!(error.contains("dark.roles.selection_text / dark.roles.selection_background"));
        assert!(error.contains("최소 4.5:1"));
    }

    #[test]
    fn focus_is_checked_against_surface_as_well_as_background() {
        let mut theme = sample();
        theme.light.roles.insert("surface".into(), "pine".into());
        let error = check(&theme).unwrap_err();
        assert!(error.contains("light.roles.focus / light.roles.surface"));
        assert!(error.contains("최소 3:1"));
    }

    #[test]
    fn link_contrast_does_not_round_up_to_threshold() {
        let mut theme = sample();
        theme.light.palette.insert("paper".into(), "#FFFFFF".into());
        theme
            .light
            .palette
            .insert("link_test".into(), "#767676".into());
        theme
            .dark
            .palette
            .insert("link_test".into(), "#767676".into());
        theme.light.roles.insert("link".into(), "link_test".into());
        assert!(check(&theme).is_ok());
        theme
            .light
            .palette
            .insert("link_test".into(), "#777777".into());
        let error = check(&theme).unwrap_err();
        assert!(error.contains("light.roles.link / light.roles.background"));
        assert!(error.contains("최소 4.5:1"));
    }
}
