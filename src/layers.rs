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
    (
        "typora",
        &["--lbrx-quote", "--lbrx-link", "--lbrx-footnote-reference"],
    ),
    ("obsidian", &["--link-external-color", "--text-muted"]),
];

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Layers {
    pub common: Mapping,
    pub languages: BTreeMap<String, Mapping>,
    pub editors: BTreeMap<String, Mapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_layout: Option<TyporaLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_paragraph: Option<TyporaParagraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_headings: Option<TyporaHeadings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_emphasis: Option<TyporaEmphasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_footnotes: Option<TyporaFootnotes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_horizontal_rule: Option<TyporaHorizontalRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_table: Option<TyporaTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_lists: Option<TyporaLists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_task_lists: Option<TyporaTaskLists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_images: Option<TyporaImages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_math: Option<TyporaMath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_alerts: Option<TyporaAlerts>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaAlerts {
    pub shaded_background: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaMath {
    pub padding_block_px: u16,
    pub padding_inline_px: u16,
    pub shaded_background: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaImages {
    pub caption_size_percent: std::num::NonZeroU16,
    pub caption_gap_px: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaTaskLists {
    pub strike_completed: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaLists {
    pub indent_px: std::num::NonZeroU16,
    pub item_gap_px: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaTable {
    pub cell_padding_block_px: u16,
    pub cell_padding_inline_px: u16,
    pub row_rule_px: std::num::NonZeroU16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaFootnotes {
    pub body_size_percent: std::num::NonZeroU16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaHorizontalRule {
    pub thickness_px: std::num::NonZeroU16,
    pub margin_block_px: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaEmphasis {
    pub light: TyporaEmphasisColors,
    pub dark: TyporaEmphasisColors,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaEmphasisColors {
    pub strong: String,
    pub italic: String,
    pub strikethrough: String,
    pub highlight_background: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaHeadings {
    h1: TyporaHeading,
    h2: TyporaHeading,
    h3: TyporaHeading,
    h4: TyporaHeading,
    h5: TyporaHeading,
    h6: TyporaHeading,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaHeading {
    pub size_percent: u16,
    pub weight: u16,
    pub margin_before_px: u16,
    pub margin_after_px: u16,
    pub divider: bool,
}

impl TyporaHeadings {
    pub fn levels(&self) -> [(&'static str, &TyporaHeading); 6] {
        [
            ("h1", &self.h1),
            ("h2", &self.h2),
            ("h3", &self.h3),
            ("h4", &self.h4),
            ("h5", &self.h5),
            ("h6", &self.h6),
        ]
    }

    fn validate(&self, path: &Path) -> Result<(), String> {
        for (level, style) in self.levels() {
            if style.size_percent == 0 || !(1..=1000).contains(&style.weight) {
                return Err(format!(
                    "{}: headings.{level}: size_percent는 양수, weight는 1–1000이어야 합니다",
                    path.display()
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaParagraph {
    pub gap_px: Option<u16>,
    pub alignment: Option<TyporaParagraphAlignment>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaParagraphAlignment {
    Start,
    Justify,
}

impl TyporaParagraphAlignment {
    pub const fn css_value(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Justify => "justify",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaLayout {
    pub max_width_px: u16,
    pub padding_inline_min_px: u16,
    pub padding_inline_max_px: u16,
}

impl TyporaLayout {
    fn validate(&self, path: &Path) -> Result<(), String> {
        if self.max_width_px == 0
            || self.padding_inline_min_px > self.padding_inline_max_px
            || u32::from(self.padding_inline_max_px) * 2 >= u32::from(self.max_width_px)
        {
            return Err(format!(
                "{}: layout: 최대 폭은 양수, 최소 여백은 최대 여백 이하, 양쪽 최대 여백의 합은 최대 폭 미만이어야 합니다",
                path.display()
            ));
        }
        Ok(())
    }
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TyporaFile {
    #[serde(default)]
    colors: Mapping,
    layout: Option<TyporaLayout>,
    paragraph: Option<TyporaParagraph>,
    headings: Option<TyporaHeadings>,
    emphasis: Option<TyporaEmphasis>,
    footnotes: Option<TyporaFootnotes>,
    horizontal_rule: Option<TyporaHorizontalRule>,
    table: Option<TyporaTable>,
    lists: Option<TyporaLists>,
    task_lists: Option<TyporaTaskLists>,
    images: Option<TyporaImages>,
    math: Option<TyporaMath>,
    alerts: Option<TyporaAlerts>,
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
                let colors = if id == "typora" {
                    let config = read::<TyporaFile>(&file)?;
                    if let Some(layout) = &config.layout {
                        layout.validate(&file)?;
                    }
                    if let Some(headings) = &config.headings {
                        headings.validate(&file)?;
                    }
                    theme.layers.typora_layout = config.layout;
                    theme.layers.typora_paragraph = config.paragraph;
                    theme.layers.typora_headings = config.headings;
                    theme.layers.typora_emphasis = config.emphasis;
                    theme.layers.typora_footnotes = config.footnotes;
                    theme.layers.typora_horizontal_rule = config.horizontal_rule;
                    theme.layers.typora_table = config.table;
                    theme.layers.typora_lists = config.lists;
                    theme.layers.typora_task_lists = config.task_lists;
                    theme.layers.typora_images = config.images;
                    theme.layers.typora_math = config.math;
                    theme.layers.typora_alerts = config.alerts;
                    config.colors
                } else {
                    read::<EditorFile>(&file)?.colors
                };
                (colors, *keys)
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
    fn typora_layout_controls_both_stylesheets_and_rejects_invalid_padding() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let layout =
            "[layout]\nmax_width_px=900\npadding_inline_min_px=12\npadding_inline_max_px=36";
        fs::write(&sidecar, layout).unwrap();
        let output = resolve::resolve(&load(&path).unwrap()).unwrap();
        for (_, css) in crate::typora::render(&output).iter().take(2) {
            assert!(css.contains("max-width: min(100%, 900px);"));
            assert!(css.contains("padding-inline: clamp(12px, 4vw, 36px);"));
        }
        fs::write(&sidecar, layout.replace("=36", "=8")).unwrap();
        assert!(load(&path).err().unwrap().contains("layout"));
    }

    #[test]
    fn typora_paragraph_gap_styles_only_adjacent_top_level_paragraphs() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || {
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            crate::typora::render(&theme)
        };
        assert!(!render()[0].1.contains("#write > p + p"));
        fs::write(dir.join("typora.toml"), "[paragraph]\ngap_px=14").unwrap();
        for (_, css) in render().iter().take(2) {
            assert!(css.contains("#write > p:has(+ p) { margin-block-end: 0; }"));
            assert!(css.contains("#write > p + p { margin-block-start: 14px; }"));
        }
    }

    #[test]
    fn typora_paragraph_alignment_is_independent_of_gap() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for value in ["start", "justify"] {
            fs::write(&sidecar, format!("[paragraph]\nalignment = \"{value}\"")).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            for (_, css) in crate::typora::render(&theme).iter().take(2) {
                assert!(css.contains(&format!("#write > p {{ text-align: {value}; }}")));
                assert!(!css.contains("#write > p + p"));
            }
        }
        fs::write(&sidecar, "[paragraph]\nalignment = \"center\"").unwrap();
        assert!(matches!(load(&path), Err(error) if error.contains("alignment")));
    }

    #[test]
    fn typora_headings_render_configured_levels_and_validate_weight() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        use std::fmt::Write as _;
        let mut config = String::new();
        for n in 1..=6 {
            writeln!(
                config,
                "[headings.h{n}]\nsize_percent = {}\nweight = 650\nmargin_before_px = 21\nmargin_after_px = 9\ndivider = {}",
                110 + n,
                n == 3
            )
            .unwrap();
        }
        fs::write(&sidecar, &config).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        for (_, css) in crate::typora::render(&theme).iter().take(2) {
            assert!(css.contains("#write h3 {\n  font-size: 113%;\n  font-weight: 650;\n  margin-block: 21px 9px;\n  border-block-end: 1px solid var(--lbrx-muted);"));
            assert!(css.contains("#write h6 {\n  font-size: 116%;\n  font-weight: 650;\n  margin-block: 21px 9px;\n  border-block-end: none;"));
        }
        fs::write(
            &sidecar,
            config.replacen("weight = 650", "weight = 1001", 1),
        )
        .unwrap();
        assert!(matches!(load(&path), Err(error) if error.contains("headings.h1")));
    }

    #[test]
    fn typora_emphasis_uses_each_mode_and_rejects_low_highlight_contrast() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let config = include_str!("../themes/editors/typora.toml");
        fs::write(&sidecar, config).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [(_, light), (_, dark), _] = crate::typora::render(&theme);
        for (css, strong, italic, strike, highlight) in [
            (light, "#8B404B", "#684882", "#555A63", "#F4E7A3"),
            (dark, "#F2B9BE", "#D8C2F1", "#C5C8CF", "#51451D"),
        ] {
            assert!(css.contains(&format!("font-weight: 700; color: {strong};")));
            assert!(css.contains(&format!("font-style: italic; color: {italic};")));
            assert!(css.contains(&format!(
                "text-decoration-line: line-through; color: {strike};"
            )));
            assert!(css.contains(&format!("background-color: {highlight}; color: inherit;")));
        }
        fs::write(&sidecar, config.replace("#555A63", "#696969")).unwrap();
        let error = resolve::resolve(&load(&path).unwrap()).unwrap_err();
        assert!(error.contains("emphasis.strikethrough"));
        assert!(error.contains("emphasis.highlight_background"));
    }

    #[test]
    fn typora_footnotes_and_rule_render_only_when_configured() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        for (_, css) in render().iter().take(2) {
            assert!(!css.contains("sup.md-footnote"));
            assert!(!css.contains("hr:not(.footnotes-area hr)"));
        }
        let config = "\
            [colors]\n'--lbrx-footnote-reference' = 'function'\n\
            [footnotes]\nbody_size_percent = 85\n\
            [horizontal_rule]\nthickness_px = 2\nmargin_block_px = 18";
        let sidecar = dir.join("typora.toml");
        fs::write(&sidecar, config).unwrap();
        let [(_, light), (_, dark), _] = render();
        for (css, blue) in [(light, "#43617B"), (dark, "#97BBD6")] {
            assert!(css.contains(&format!("#write sup.md-footnote {{ color: {blue};")));
            assert!(css.contains("#write :is(.md-def-footnote, .footnote-line) { font-size: 85%;"));
            assert!(css.contains("#write hr:not(.footnotes-area hr) {"));
            assert!(css.contains("border-block-start: 2px solid var(--lbrx-muted);"));
            assert!(css.contains("margin-block: 18px;"));
        }
        fs::write(
            &sidecar,
            config.replace("thickness_px = 2", "thickness_px = 0"),
        )
        .unwrap();
        assert!(matches!(load(&path), Err(error) if error.contains("thickness_px")));
    }

    #[test]
    fn typora_table_emits_custom_spacing_and_screen_only_scrolling() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        assert!(!render()[0].1.contains("figure.table-figure"));
        fs::write(
            dir.join("typora.toml"),
            "[table]\ncell_padding_block_px = 6\ncell_padding_inline_px = 10\nrow_rule_px = 2",
        )
        .unwrap();
        let [(_, light), (_, dark), _] = render();
        for (css, surface) in [(light, "#E8EDE4"), (dark, "#25332C")] {
            assert!(css.contains(&format!("--lbrx-surface: {surface};")));
            assert!(css.contains("padding: 6px 10px;\n  border: 0;\n  border-block-end: 2px"));
            assert!(css.contains("thead th {\n  background-color: var(--lbrx-surface);"));
            assert!(css.contains(
                "@media screen {\n  #write figure.table-figure { max-width: 100%; overflow-x: auto; }"
            ));
        }
    }

    #[test]
    fn typora_lists_render_custom_spacing_and_preserve_task_item_markers() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        assert!(!render()[0].1.contains("list-style-type: square"));
        fs::write(
            dir.join("typora.toml"),
            "[lists]\nindent_px=20\nitem_gap_px=2",
        )
        .unwrap();
        for (_, css) in render().iter().take(2) {
            assert!(css.contains("padding-inline-start: 20px"));
            assert!(css.contains("margin-block-start: 2px"));
            for marker in ["disc", "circle", "square"] {
                let rule = css
                    .lines()
                    .find(|line| line.contains(&format!("list-style-type: {marker};")))
                    .unwrap();
                assert!(rule.contains("ul") && rule.contains(".md-task-list-item"));
            }
        }
    }

    #[test]
    fn typora_task_lists_can_omit_the_completed_strike() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let sidecar = dir.join("typora.toml");
        assert!(!render()[0].1.contains("accent-color:"));
        for strike in [true, false] {
            fs::write(
                &sidecar,
                format!("[task_lists]\nstrike_completed = {strike}"),
            )
            .unwrap();
            let [(_, light), (_, dark), _] = render();
            for (css, accent) in [(light, "#246958"), (dark, "#84C7AC")] {
                assert!(css.contains(&format!("accent-color: {accent};")));
                let rule = css
                    .lines()
                    .find(|line| line.contains(":checked) > p"))
                    .unwrap();
                assert!(rule.contains("color: var(--lbrx-muted);"));
                assert_eq!(rule.contains("line-through"), strike);
            }
        }
    }

    #[test]
    fn typora_images_style_explicit_captions_only_when_configured() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        for (_, css) in render().iter().take(2) {
            assert!(!css.contains("figcaption"));
        }
        fs::write(
            dir.join("typora.toml"),
            "[images]\ncaption_size_percent = 85\ncaption_gap_px = 10",
        )
        .unwrap();
        for (_, css) in render().iter().take(2) {
            assert!(css.contains("figcaption {\n  margin-block-start: 10px;\n  font-size: 85%;"));
        }
    }

    #[test]
    fn typora_math_applies_only_configured_block_styles() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        assert!(!render()[0].1.contains("#write .md-math-block"));
        for shading in [true, false] {
            fs::write(
                dir.join("typora.toml"),
                format!(
                    "[math]\npadding_block_px=7\npadding_inline_px=11\nshaded_background={shading}"
                ),
            )
            .unwrap();
            for (_, css) in render().iter().take(2) {
                assert!(css.contains("padding-block: 7px;\n  padding-inline: 11px;"));
                assert!(css.contains("@media screen {\n  #write .md-math-block > .md-math-container { max-inline-size: 100%; overflow-x: auto; }"));
                assert_eq!(
                    css.contains(
                        "#write .md-math-block { background-color: var(--lbrx-surface); }"
                    ),
                    shading
                );
                if shading {
                    assert!(css.contains("--lbrx-surface:"));
                }
            }
        }
    }

    #[test]
    fn typora_alerts_emit_type_colors_without_replacing_native_labels() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        assert!(!render()[0].1.contains(".md-alert"));
        let sidecar = dir.join("typora.toml");
        for shaded in [true, false] {
            fs::write(&sidecar, format!("[alerts]\nshaded_background = {shaded}")).unwrap();
            let [(_, light), (_, dark), _] = render();
            for (css, note) in [(light, "#43617B"), (dark, "#97BBD6")] {
                assert!(css.contains(&format!(".md-alert-note {{ border-left-color: {note}; }}")));
                assert!(css.contains(&format!(".md-alert-text-note {{ color: {note}; }}")));
                for kind in ["tip", "important", "warning", "caution"] {
                    assert!(css.contains(&format!(".md-alert-text-{kind} {{ color:")));
                }
                assert_eq!(
                    css.contains(".md-alert { background-color: var(--lbrx-surface); }"),
                    shaded
                );
                assert!(!css.contains(".md-alert-text-container"));
            }
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
