use crate::{
    css::font_stack,
    resolve::{ResolvedMode, ResolvedTheme},
};
use serde_json::{Map, Value, json};
use std::{
    fmt::Write as _,
    io::{Cursor, Read, Write as _},
};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

pub const DIRECTORY: &str = "vscode";
pub const VSIX: &str = "lbrx-theme.vsix";
pub const CSS: &str = "lbrx-code.css";
pub const SETTINGS: &str = "settings-font.json";
pub const FILES: &[&str] = &[VSIX, CSS, SETTINGS];
const VERSION: &str = env!("CARGO_PKG_VERSION");
const ENGINE: &str = "^1.147.0";
const PACKAGE: &str = "extension/package.json";
const LIGHT: &str = "themes/lbrx-light.json";
const DARK: &str = "themes/lbrx-dark.json";
const TYPES: &str = "[Content_Types].xml";
const MANIFEST: &str = "extension.vsixmanifest";
const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="json" ContentType="application/json"/>
  <Default Extension="vsixmanifest" ContentType="text/xml"/>
</Types>
"#;
const TOKENS: &[(&str, &str)] = &[
    ("comment", "comment"),
    ("keyword", "keyword"),
    ("string", "string"),
    ("entity.name.function", "function"),
    ("constant.numeric", "number"),
    ("invalid.illegal", "error"),
];
const COLORS: &[(&str, &str)] = &[
    ("focusBorder", "focus"),
    ("foreground", "text"),
    ("errorForeground", "error"),
    ("editor.background", "background"),
    ("editor.foreground", "text"),
    ("editorLineNumber.foreground", "muted"),
    ("editor.selectionBackground", "selection_background"),
    ("editor.selectionForeground", "selection_text"),
    ("editorCursor.foreground", "focus"),
    ("sideBar.background", "surface"),
    ("sideBar.foreground", "text"),
    ("input.background", "background"),
    ("input.foreground", "text"),
    ("textLink.foreground", "link"),
    ("terminal.background", "terminal_background"),
    ("terminal.foreground", "terminal_foreground"),
    ("terminalCursor.foreground", "terminal_cursor"),
    ("terminal.selectionBackground", "selection_background"),
    ("terminal.selectionForeground", "selection_text"),
];
const ANSI_COLORS: [&str; 16] = [
    "Black",
    "Red",
    "Green",
    "Yellow",
    "Blue",
    "Magenta",
    "Cyan",
    "White",
    "BrightBlack",
    "BrightRed",
    "BrightGreen",
    "BrightYellow",
    "BrightBlue",
    "BrightMagenta",
    "BrightCyan",
    "BrightWhite",
];

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, Vec<u8>); 3], String> {
    let archive = make_vsix(theme).map_err(|error| format!("VS Code VSIX: {error}"))?;
    let mono = &theme.typography.mono;
    let settings = json!({
        "editor.fontFamily": font_stack(mono),
        "editor.fontSize": mono.size_px,
        "editor.lineHeight": (mono.size_px * mono.line_height).round().max(8.0),
        "terminal.integrated.fontFamily": font_stack(mono),
        "terminal.integrated.fontSize": mono.size_px,
    });
    Ok([
        (VSIX, archive),
        (CSS, code_css(theme).into_bytes()),
        (SETTINGS, json_bytes(&settings)),
    ])
}

fn json_bytes(value: &Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("JSON value serializes");
    bytes.push(b'\n');
    bytes
}

fn make_vsix(theme: &ResolvedTheme) -> Result<Vec<u8>, zip::result::ZipError> {
    let package = json!({
        "name": "lbrx-theme",
        "displayName": "LBRX",
        "publisher": "lbrx",
        "version": VERSION,
        "engines": { "vscode": ENGINE },
        "categories": ["Themes"],
        "contributes": { "themes": [
            { "label": "LBRX Light", "uiTheme": "vs", "path": format!("./{LIGHT}") },
            { "label": "LBRX Dark", "uiTheme": "vs-dark", "path": format!("./{DARK}") }
        ] }
    });
    let manifest = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
  <Metadata>
    <Identity Id="lbrx-theme" Version="{VERSION}" Language="en-US" Publisher="lbrx"/>
    <DisplayName>LBRX</DisplayName>
    <Categories>Themes</Categories>
    <Properties><Property Id="Microsoft.VisualStudio.Code.Engine" Value="{ENGINE}"/></Properties>
  </Metadata>
  <Installation><InstallationTarget Id="Microsoft.VisualStudio.Code"/></Installation>
  <Assets><Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="{PACKAGE}" Addressable="true"/></Assets>
</PackageManifest>
"#
    );
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (path, bytes) in [
        (TYPES.into(), CONTENT_TYPES.as_bytes().to_vec()),
        (MANIFEST.into(), manifest.into_bytes()),
        (PACKAGE.into(), json_bytes(&package)),
        (
            format!("extension/{LIGHT}"),
            json_bytes(&color_theme(theme, "LBRX Light", "light", &theme.light)),
        ),
        (
            format!("extension/{DARK}"),
            json_bytes(&color_theme(theme, "LBRX Dark", "dark", &theme.dark)),
        ),
    ] {
        zip.start_file(path, options)?;
        zip.write_all(&bytes)?;
    }
    Ok(zip.finish()?.into_inner())
}

fn color_theme(theme: &ResolvedTheme, name: &str, kind: &str, mode: &ResolvedMode) -> Value {
    let mut colors: Map<String, Value> = COLORS
        .iter()
        .map(|&(id, role)| {
            (
                id.into(),
                json!(theme.editor_color(mode, DIRECTORY, id, &mode.roles[role])),
            )
        })
        .collect();
    for (name, color) in ANSI_COLORS.into_iter().zip(&mode.ansi) {
        colors.insert(format!("terminal.ansi{name}"), json!(color));
    }
    let mut token_colors: Vec<_> = TOKENS.iter()
        .map(|&(scope, role)| json!({ "scope": scope, "settings": { "foreground": theme.token(mode, "", role) } }))
        .collect();
    let mut semantic = json!({"function": theme.token(mode, "", "function"), "number": theme.token(mode, "", "number")});
    for (lang, tokens) in &theme.layers.languages {
        let grammar = match lang.as_str() {
            "typescript" => Some("source.ts"),
            "rust" => Some("source.rust"),
            "python" => Some("source.python"),
            "powershell" => Some("source.powershell"),
            "toml" => Some("source.toml"),
            _ => None,
        };
        for &(scope, role) in TOKENS {
            if !tokens.contains_key(role) {
                continue;
            }
            let fg = theme.token(mode, lang, role);
            if let Some(parent) = grammar {
                token_colors.push(
                    json!({"scope": format!("{parent} {scope}"), "settings": {"foreground": fg}}),
                );
            }
            if role != "error" {
                semantic[format!("{role}:{lang}")] = json!(fg);
            }
        }
    }
    json!({
        "name": name,
        "type": kind,
        "colors": colors,
        "tokenColors": token_colors,
        "semanticHighlighting": true,
        "semanticTokenColors": semantic
    })
}

fn code_css(theme: &ResolvedTheme) -> String {
    let mut css = String::from("/* LBRX - generated from the theme source. */\n");
    for (selector, mode) in [
        (".lbrx-code, .lbrx-code[data-theme=\"light\"]", &theme.light),
        (".lbrx-code[data-theme=\"dark\"]", &theme.dark),
    ] {
        writeln!(css, "{selector} {{").unwrap();
        writeln!(
            css,
            "  background-color: {};\n  color: {};",
            mode.roles["background"], mode.roles["text"]
        )
        .unwrap();
        for &(_, role) in TOKENS {
            writeln!(
                css,
                "  --lbrx-code-{role}: {};",
                theme.token(mode, "", role)
            )
            .unwrap();
        }
        css.push_str("}\n");
        let container = selector.split(", ").next().unwrap();
        for (lang, tokens) in &theme.layers.languages {
            for token in tokens.keys() {
                writeln!(
                    css,
                    "{container} .language-{lang} {{ --lbrx-code-{token}: {}; }}",
                    theme.token(mode, lang, token)
                )
                .unwrap();
            }
        }
    }
    for &(_, role) in TOKENS {
        writeln!(
            css,
            ".lbrx-code .tok-{role} {{ color: var(--lbrx-code-{role}); }}"
        )
        .unwrap();
    }
    css
}

pub fn validate(bytes: &[u8]) -> Result<(), String> {
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(|error| error.to_string())?;
    let mut read = |path: &str| -> Result<Vec<u8>, String> {
        let mut entry = zip
            .by_name(path)
            .map_err(|error| format!("{path}: {error}"))?;
        let mut bytes = Vec::new();
        entry
            .read_to_end(&mut bytes)
            .map_err(|error| format!("{path}: {error}"))?;
        Ok(bytes)
    };
    read(TYPES)?;
    read(MANIFEST)?;
    let package: Value =
        serde_json::from_slice(&read(PACKAGE)?).map_err(|error| error.to_string())?;
    let themes = package["contributes"]["themes"]
        .as_array()
        .filter(|themes| themes.len() == 2)
        .ok_or("두 색 테마 등록이 필요합니다")?;
    for (entry, (path, kind, ui_theme)) in themes
        .iter()
        .zip([(LIGHT, "light", "vs"), (DARK, "dark", "vs-dark")])
    {
        if entry["path"] != format!("./{path}") || entry["uiTheme"] != ui_theme {
            return Err(format!("{path}: 테마 등록 불일치"));
        }
        let theme: Value = serde_json::from_slice(&read(&format!("extension/{path}"))?)
            .map_err(|error| error.to_string())?;
        if theme["type"] != kind || !theme["colors"].is_object() || !theme["tokenColors"].is_array()
        {
            return Err(format!("{path}: 색 테마 구조 불일치"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve::resolve;
    use zip::read::ZipFile;

    fn entry(bytes: &[u8], path: &str) -> Value {
        let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
        let file: ZipFile<'_> = archive.by_name(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    #[test]
    fn vsix_exports_both_modes_and_keeps_dark_changes_isolated() {
        let mut theme =
            resolve(&toml::from_str(include_str!("../themes/lbrx.toml")).unwrap()).unwrap();
        let [(.., vsix), (.., css), (.., settings)] = render(&theme).unwrap();
        validate(&vsix).unwrap();
        assert_eq!(vsix, render(&theme).unwrap()[0].1);
        let package = entry(&vsix, "extension/package.json");
        assert!(package.get("main").is_none());
        assert!(package.get("browser").is_none());
        let css = String::from_utf8(css).unwrap();
        let (light_css, dark_css) = css.split_once("[data-theme=\"dark\"]").unwrap();
        for (file, mode, css) in [
            ("lbrx-light.json", &theme.light, light_css),
            ("lbrx-dark.json", &theme.dark, dark_css),
        ] {
            let value = entry(&vsix, &format!("extension/themes/{file}"));
            assert_eq!(value["colors"].as_object().unwrap().len(), 35);
            assert_eq!(value["colors"]["sideBar.background"], mode.roles["surface"]);
            assert_eq!(value["colors"]["terminal.ansiMagenta"], mode.ansi[5]);
            assert_eq!(value["colors"]["terminal.ansiBrightMagenta"], mode.ansi[13]);
            assert_eq!(value["tokenColors"][3]["scope"], "entity.name.function");
            assert_eq!(
                value["tokenColors"][3]["settings"]["foreground"],
                mode.roles["function"]
            );
            assert_eq!(
                value["semanticTokenColors"]["function"],
                mode.roles["function"]
            );
            assert!(css.contains(&format!(
                "--lbrx-code-function: {};",
                mode.roles["function"]
            )));
        }
        let settings: Value = serde_json::from_slice(&settings).unwrap();
        assert_eq!(settings["editor.lineHeight"], 20.0);
        assert!(
            settings["editor.fontFamily"]
                .as_str()
                .unwrap()
                .contains("D2Coding")
        );
        theme.dark.roles.insert("function".into(), "#ABCDEF".into());
        let changed = render(&theme).unwrap();
        let light = "extension/themes/lbrx-light.json";
        assert_eq!(entry(&vsix, light), entry(&changed[0].1, light));
        assert_eq!(
            entry(&changed[0].1, "extension/themes/lbrx-dark.json")["semanticTokenColors"]["function"],
            "#ABCDEF"
        );
    }
}
