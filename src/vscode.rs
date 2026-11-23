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
pub const FILES: &[&str] = &[VSIX, CSS, SETTINGS, crate::build::GUIDE];
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
    ("keyword.control", "keyword.control"),
    ("storage.type", "keyword.declaration"),
    ("storage.modifier", "keyword.declaration"),
    ("constant.character.escape", "string.escape"),
    ("string.regexp", "string.escape"),
    ("constant.language", "constant.builtin"),
    ("entity.name.function.macro", "function.macro"),
    ("support.function", "function"),
    ("entity.name.type", "type"),
    ("support.type", "type"),
    ("variable.other", "variable"),
    ("variable.parameter", "variable.parameter"),
    ("variable.other.property", "property"),
    ("support.type.property-name", "property"),
    ("keyword.operator", "operator"),
    ("entity.other.attribute-name", "attribute"),
    ("punctuation", "punctuation"),
];
const NATIVE_SCOPES: &[(&str, &str, &str)] = &[
    ("rust", "entity.name.type.macro.rust", "function.macro"),
    ("rust", "entity.name.type.lifetime.rust", "attribute"),
    ("rust", "punctuation.definition.lifetime.rust", "attribute"),
    ("rust", "meta.attribute.rust", "attribute"),
    ("rust", "punctuation.definition.attribute.rust", "attribute"),
    ("rust", "punctuation.brackets.attribute.rust", "attribute"),
    ("rust", "meta.attribute.rust entity.name.type", "type"),
    ("rust", "keyword.operator.namespace.rust", "punctuation"),
    ("rust", "storage.modifier.mut.rust", "keyword.declaration"),
    (
        "python",
        "entity.name.function.decorator.python",
        "attribute",
    ),
    (
        "python",
        "punctuation.definition.decorator.python",
        "attribute",
    ),
    ("python", "storage.type.string.python", "string.escape"),
    ("python", "storage.type.format.python", "string.escape"),
    ("python", "support.function.magic.python", "function.macro"),
    (
        "powershell",
        "support.variable.automatic.powershell",
        "constant.builtin",
    ),
    ("powershell", "support.function.powershell", "function"),
    (
        "powershell",
        "variable.parameter.powershell",
        "variable.parameter",
    ),
    ("typescript", "support.type.primitive", "type"),
    ("typescript", "storage.type.interface", "type"),
    ("typescript", "storage.type.type", "type"),
    ("typescript", "entity.name.tag.tsx", "keyword.declaration"),
    ("typescript", "support.class.component.tsx", "function"),
    (
        "typescript",
        "punctuation.definition.template-expression",
        "string.escape",
    ),
    ("json", "punctuation.support.type.property-name", "property"),
    (
        "jsonc",
        "punctuation.support.type.property-name",
        "property",
    ),
    ("yaml", "entity.name.tag.yaml", "property"),
    ("yaml", "variable.other.anchor.yaml", "attribute"),
    ("yaml", "variable.other.alias.yaml", "attribute"),
    ("yaml", "punctuation.definition.anchor.yaml", "attribute"),
    ("yaml", "punctuation.definition.alias.yaml", "attribute"),
    ("yaml", "constant.language.merge.yaml", "keyword.control"),
    (
        "yaml",
        "keyword.control.flow.block-scalar",
        "keyword.declaration",
    ),
    (
        "yaml",
        "storage.modifier.chomping-indicator.yaml",
        "keyword.declaration",
    ),
    ("toml", "entity.name.section", "type"),
    (
        "javascript",
        "punctuation.definition.template-expression",
        "string.escape",
    ),
    ("javascript", "support.class.component", "function"),
    ("javascript", "entity.name.tag", "keyword.declaration"),
    ("html", "entity.name.tag.html", "keyword.declaration"),
    ("html", "constant.character.entity", "string.escape"),
    ("css", "support.type.vendored.property-name", "property"),
    ("css", "keyword.control.at-rule", "keyword.declaration"),
    ("sql", "keyword.other.DML", "keyword.control"),
    ("sql", "keyword.other.DDL", "keyword.declaration"),
    ("java", "storage.type.primitive.java", "type"),
    ("java", "storage.type.annotation.java", "attribute"),
    (
        "java",
        "punctuation.definition.annotation.java",
        "attribute",
    ),
    ("csharp", "keyword.preprocessor", "attribute"),
    ("c", "entity.name.function.preprocessor", "function.macro"),
    ("cpp", "entity.name.function.preprocessor", "function.macro"),
    ("shell", "variable.parameter.positional", "constant.builtin"),
    ("shell", "variable.other.normal.shell", "variable"),
    ("go", "storage.type", "type"),
    ("markdown", "markup.heading.markdown", "keyword.declaration"),
    ("markdown", "markup.inline.raw", "string"),
    (
        "dockerfile",
        "keyword.control.dockerfile",
        "keyword.declaration",
    ),
    ("xml", "entity.name.tag.xml", "keyword.declaration"),
    ("xml", "constant.character.entity", "string.escape"),
    ("log", "log.error", "error"),
    ("log", "log.warning", "attribute"),
    ("log", "log.date", "number"),
    ("yaml", "entity.other.document", "punctuation"),
    ("yaml", "string.unquoted.block.yaml", "string"),
    (
        "powershell",
        "punctuation.section.embedded.substatement",
        "string.escape",
    ),
    ("php", "support.attribute", "attribute"),
    ("ruby", "constant.language.symbol.hashkey.ruby", "property"),
    ("swift", "storage.modifier.attribute.swift", "attribute"),
    (
        "swift",
        "punctuation.definition.attribute.swift",
        "attribute",
    ),
    ("dart", "storage.type.annotation.dart", "attribute"),
    ("lua", "entity.other.attribute.lua", "attribute"),
    ("julia", "support.function.macro.julia", "function.macro"),
];

pub(crate) fn grammar_roots(lang: &str) -> &'static [&'static str] {
    match lang {
        "typescript" => &["source.ts", "source.tsx"],
        "rust" => &["source.rust"],
        "python" => &["source.python"],
        "powershell" => &["source.powershell"],
        "toml" => &["source.toml"],
        "json" => &["source.json"],
        "jsonc" => &["source.json.comments"],
        "yaml" => &["source.yaml"],
        "javascript" => &["source.js", "source.js.jsx"],
        "html" => &["text.html.basic", "text.html.derivative"],
        "css" => &["source.css"],
        "sql" => &["source.sql"],
        "java" => &["source.java"],
        "csharp" => &["source.cs"],
        "c" => &["source.c"],
        "cpp" => &["source.cpp"],
        "shell" => &["source.shell"],
        "go" => &["source.go"],
        "markdown" => &["text.html.markdown"],
        "dockerfile" => &["source.dockerfile"],
        "xml" => &["text.xml"],
        "log" => &["text.log"],
        "php" => &["source.php", "text.html.php"],
        "ruby" => &["source.ruby"],
        "swift" => &["source.swift"],
        "dart" => &["source.dart"],
        "lua" => &["source.lua"],
        "r" => &["source.r"],
        "julia" => &["source.julia"],
        _ => &[],
    }
}

const SEMANTIC: &[(&str, &str)] = &[
    ("function", "function"),
    ("method", "function"),
    ("number", "number"),
    ("macro", "function.macro"),
    ("type", "type"),
    ("class", "type"),
    ("enum", "type"),
    ("interface", "type"),
    ("struct", "type"),
    ("typeParameter", "type"),
    ("parameter", "variable.parameter"),
    ("property", "property"),
    ("decorator", "attribute"),
    ("regexp", "string.escape"),
    ("comment", "comment"),
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

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, Vec<u8>); 4], String> {
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
        (
            crate::build::GUIDE,
            include_bytes!("../Docs/18-vscode.md").to_vec(),
        ),
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
    let settings = |lang, token| {
        let style = theme.token_style(lang, token);
        json!({"foreground": theme.token(mode, lang, token),
               "fontStyle": if style.name() == "normal" { "" } else { style.name() }})
    };
    let semantic_style = |lang, token| {
        let style = theme.token_style(lang, token);
        let color = theme.token(mode, lang, token);
        if style.name() == "normal" {
            json!(color)
        } else {
            json!({"foreground": color, "fontStyle": style.name()})
        }
    };
    let mut token_colors: Vec<_> = TOKENS
        .iter()
        .map(|&(scope, token)| json!({ "scope": scope, "settings": settings("", token) }))
        .collect();
    let mut semantic = Map::new();
    for &(selector, token) in SEMANTIC {
        semantic.insert(selector.into(), semantic_style("", token));
    }
    for lang in theme.layers.languages.keys() {
        let grammar = grammar_roots(lang);
        for &(scope, role) in TOKENS {
            if !theme.has_token_override(lang, role) {
                continue;
            }
            for parent in grammar {
                token_colors.push(
                    json!({"scope": format!("{parent} {scope}"), "settings": settings(lang, role)}),
                );
            }
        }
        for &(_, scope, token) in NATIVE_SCOPES.iter().filter(|&&(id, _, _)| id == lang) {
            for parent in grammar {
                token_colors.push(json!({"scope": format!("{parent} {scope}"), "settings": settings(lang, token)}));
            }
        }
        let ids: &[&str] = match lang.as_str() {
            "typescript" => &["typescript", "typescriptreact"],
            "javascript" => &["javascript", "javascriptreact"],
            "shell" => &["shellscript"],
            _ => &[lang],
        };
        for &(selector, token) in SEMANTIC {
            if theme.has_token_override(lang, token) {
                for id in ids {
                    semantic.insert(format!("{selector}:{id}"), semantic_style(lang, token));
                }
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
        let [(.., vsix), (.., css), (.., settings), ..] = render(&theme).unwrap();
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
