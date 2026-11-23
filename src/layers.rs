use crate::model::Theme;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, fs, path::Path};

pub type Mapping = BTreeMap<String, String>;
pub const TOKENS: &[&str] = &[
    "comment", "keyword", "string", "function", "number", "error",
];
const EDITORS: &[(&str, &[&str])] = &[
    ("windows-terminal", &["cursorColor", "selectionBackground"]),
    ("iterm2", &["Cursor Color", "Selected Text Color"]),
    ("vscode", &["editorCursor.foreground", "sideBar.background"]),
    ("vim", &["LineNr", "Comment"]),
    ("helix", &["ui.linenr", "ui.linenr.selected"]),
    ("typora", &["--lbrx-quote", "--lbrx-link"]),
    ("obsidian", &["--link-external-color", "--text-muted"]),
];

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Layers {
    pub common: Mapping,
    pub languages: BTreeMap<String, Mapping>,
    pub editors: BTreeMap<String, Mapping>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LanguageFile {
    tokens: Mapping,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EditorFile {
    colors: Mapping,
}

fn read<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn files(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", dir.display())),
    };
    let mut paths = Vec::new();
    for entry in entries {
        let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

pub fn load(path: &Path) -> Result<Theme, String> {
    let mut theme: Theme = read(path)?;
    let root = path.parent().unwrap_or(Path::new("."));
    for (dir, language) in [("languages", true), ("editors", false)] {
        for file in files(&root.join(dir))? {
            let id = file.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if !id.starts_with(|c: char| c.is_ascii_lowercase())
                || !id
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            {
                return Err(format!("{}: 잘못된 파일 ID", file.display()));
            }
            let (map, allowed) = if language {
                (read::<LanguageFile>(&file)?.tokens, TOKENS)
            } else {
                let (_, keys) = EDITORS
                    .iter()
                    .find(|(name, _)| *name == id)
                    .ok_or_else(|| format!("{}: 지원하지 않는 앱", file.display()))?;
                (read::<EditorFile>(&file)?.colors, *keys)
            };
            for (key, role) in &map {
                if !allowed.contains(&key.as_str()) {
                    return Err(format!("{}: 지원하지 않는 키 '{key}'", file.display()));
                }
                for (name, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
                    if !mode.roles.contains_key(role) {
                        return Err(format!(
                            "{}: {key}: {name}.roles에 '{role}' 없음",
                            file.display()
                        ));
                    }
                }
            }
            if !language {
                theme.layers.editors.insert(id.into(), map);
            } else if id == "common" {
                theme.layers.common = map;
            } else {
                theme.layers.languages.insert(id.into(), map);
            }
        }
    }
    Ok(theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        build::{self, App},
        resolve,
    };
    use std::io::Read;

    #[test]
    fn sparse_languages_override_only_matching_outputs_and_editors_are_isolated() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let path = root.join("theme.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        fs::create_dir(root.join("languages")).unwrap();
        fs::create_dir(root.join("editors")).unwrap();
        let original = resolve::resolve(&load(&path).unwrap()).unwrap();
        fs::write(
            root.join("languages/common.toml"),
            "[tokens]\nfunction='keyword'\nstring='link'",
        )
        .unwrap();
        fs::write(
            root.join("languages/python.toml"),
            "[tokens]\nfunction='text'",
        )
        .unwrap();
        fs::write(
            root.join("editors/vscode.toml"),
            "[colors]\n'sideBar.background'='background'",
        )
        .unwrap();
        let theme = load(&path).unwrap();
        let out = root.join("dist");
        build::build(&theme, &out, &App::ALL).unwrap();
        let package =
            fs::File::open(App::VsCode.output_path(&out).join(crate::vscode::VSIX)).unwrap();
        let mut zip = zip::ZipArchive::new(package).unwrap();
        for (name, mode) in [("light", &original.light), ("dark", &original.dark)] {
            let mut buf = String::new();
            zip.by_name(&format!("extension/themes/lbrx-{name}.json"))
                .unwrap()
                .read_to_string(&mut buf)
                .unwrap();
            let json: serde_json::Value = serde_json::from_str(&buf).unwrap();
            let semantics = &json["semanticTokenColors"];
            assert_eq!(semantics["function"], mode.roles["keyword"]);
            assert_eq!(semantics["function:python"], mode.roles["text"]);
            assert!(semantics.get("function:rust").is_none());
            let rules = json["tokenColors"].as_array().unwrap();
            assert!(rules.iter().any(
                |r| r["scope"] == "string" && r["settings"]["foreground"] == mode.roles["link"]
            ));
            assert!(
                rules
                    .iter()
                    .any(|r| r["scope"] == "source.python entity.name.function"
                        && r["settings"]["foreground"] == mode.roles["text"])
            );
            assert_eq!(
                json["colors"]["sideBar.background"],
                mode.roles["background"]
            );
            let hx = fs::read_to_string(
                App::Helix
                    .output_path(&out)
                    .join(format!("lbrx_{name}.toml")),
            )
            .unwrap();
            let hx: toml::Table = toml::from_str(&hx).unwrap();
            assert_eq!(hx["function"].as_str().unwrap(), mode.roles["keyword"]);
            assert_eq!(
                hx["ui.statusline"]["bg"].as_str().unwrap(),
                mode.roles["surface"]
            );
            let vim =
                fs::read_to_string(App::Vim.output_path(&out).join(format!("lbrx_{name}.vim")))
                    .unwrap();
            assert!(vim.contains(&format!(
                "highlight pythonFunction guifg={}",
                mode.roles["text"]
            )));
            assert!(!vim.contains("highlight rustFunction "));
            let typora = fs::read_to_string(
                App::Typora
                    .output_path(&out)
                    .join(format!("lbrx-{name}.css")),
            )
            .unwrap();
            assert!(typora.contains(&format!(
                ".md-fences[lang=\"python\"] {{ --lbrx-function: {};",
                mode.roles["text"]
            )));
        }
        let css = fs::read_to_string(App::Obsidian.output_path(&out).join("theme.css")).unwrap();
        assert!(css.contains("code.language-python .token.function"));
        assert!(!css.contains("code.language-rust"));
        for (file, expected) in crate::windows_terminal::render(&original).unwrap() {
            assert_eq!(
                fs::read_to_string(App::WindowsTerminal.output_path(&out).join(file)).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn invalid_sidecar_reports_its_file_and_key() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        for (key, role) in [("unknown", "text"), ("LineNr", "typo")] {
            fs::write(dir.join("vim.toml"), format!("[colors]\n{key}='{role}'")).unwrap();
            let err = load(&path).err().unwrap();
            assert!(err.contains("vim.toml") && err.contains(key));
        }
    }
}
