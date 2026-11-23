use crate::model::Theme;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, fs, path::Path};

pub type Mapping = BTreeMap<String, String>;
pub const TYPORA_ALERT_ROLES: [(&str, &str); 5] = [
    ("note", "function"),
    ("tip", "success"),
    ("important", "keyword"),
    ("warning", "warning"),
    ("caution", "error"),
];
pub const TYPORA_SOURCE_COLORS: &[(&str, &str, &str)] = &[
    ("background", "", "background-color"),
    ("text", "", "color"),
    ("selection_background", "", "--select-text-bg-color"),
    ("selection_text", "", "--select-text-font-color"),
    (
        "active_line_background",
        " .CodeMirror-activeline-background",
        "background-color",
    ),
    ("cursor", " div.CodeMirror-cursor", "border-left-color"),
    ("line_number", " .CodeMirror-linenumber", "color"),
    ("heading", " :is(.cm-header, .cm-property)", "color"),
    ("link", " .cm-link", "color"),
    ("code", " .cm-code", "color"),
    ("comment", " .cm-comment", "color"),
    ("number", " :is(.cm-atom, .cm-number)", "color"),
    ("string", " .cm-string", "color"),
    ("tag", " .cm-tag", "color"),
];
pub const TOKENS: &[&str] = &[
    "comment",
    "keyword",
    "string",
    "function",
    "number",
    "error",
    "keyword.control",
    "keyword.declaration",
    "string.escape",
    "constant.builtin",
    "function.macro",
    "type",
    "variable",
    "variable.parameter",
    "property",
    "operator",
    "attribute",
    "punctuation",
];
pub type TokenStyles = BTreeMap<String, TokenStyle>;

pub(crate) fn language_labels(id: &str) -> &[&str] {
    match id {
        "javascript" => &["javascript", "js", "jsx"],
        "typescript" => &["typescript", "ts", "tsx"],
        "python" => &["python", "py"],
        "powershell" => &["powershell", "ps1"],
        "common-lisp" => &["common-lisp", "lisp", "cl"],
        "csharp" => &["csharp", "cs"],
        "cpp" => &["cpp", "c++"],
        "shell" => &["shell", "sh", "shellscript"],
        "markdown" => &["markdown", "md"],
        "dockerfile" => &["dockerfile", "docker"],
        "yaml" => &["yaml", "yml"],
        "text" => &["text", "txt", "plaintext"],
        _ => &[],
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenStyle {
    #[default]
    Normal,
    Bold,
    Italic,
}

impl TokenStyle {
    pub fn name(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Bold => "bold",
            Self::Italic => "italic",
        }
    }

    pub fn css(self) -> &'static str {
        match self {
            Self::Normal => "font-weight: normal; font-style: normal;",
            Self::Bold => "font-weight: bold; font-style: normal;",
            Self::Italic => "font-weight: normal; font-style: italic;",
        }
    }
}
const SEARCH_ACTIVE_TARGETS: &[&str] = &[
    "#write .md-search-select",
    "#write::highlight(search-select)",
    "#write ::highlight(search-select)",
];
pub const TYPORA_EDITING_AID_COLORS: &[(&str, &[&str], &str)] = &[
    (
        "yaml.background",
        &["#write > pre.md-meta-block"],
        "background-color",
    ),
    ("yaml.text", &["#write > pre.md-meta-block"], "color"),
    (
        "yaml.placeholder",
        &["#write > pre.md-meta-block:empty::before"],
        "color",
    ),
    (
        "search.match_background",
        &[
            "#write .md-search-hit:not(.md-search-select)",
            "#write::highlight(search-hit)",
            "#write ::highlight(search-hit)",
        ],
        "background-color",
    ),
    (
        "search.active_background",
        SEARCH_ACTIVE_TARGETS,
        "background-color",
    ),
    ("search.active_text", SEARCH_ACTIVE_TARGETS, "color"),
    (
        "link.destination_background",
        &[
            r#"#write [md-inline="link"]:is(.md-expand, .md-expand-on-selection) > .md-meta > .md-url"#,
        ],
        "background-color",
    ),
    (
        "link.title_background",
        &[
            r#"#write [md-inline="link"]:is(.md-expand, .md-expand-on-selection) > .md-meta > .md-link-title"#,
        ],
        "background-color",
    ),
];
pub const TYPORA_SIDEBAR_COLORS: &[(&str, &str, &str)] = &[
    ("background", "", "background-color"),
    ("outline.text", " #outline-content .outline-item", "color"),
    (
        "outline.hover_background",
        " #outline-content .outline-item:hover",
        "background-color",
    ),
    (
        "outline.active_background",
        " #outline-content .outline-item.outline-item-active",
        "background-color",
    ),
    (
        "outline.active_text",
        " #outline-content .outline-item.outline-item-active",
        "color",
    ),
    (
        "outline.selection_border",
        " #outline-content .outline-item.select",
        "outline-color",
    ),
    (
        "file_tree.text",
        " #file-library-tree .file-tree-node > .file-node-content",
        "color",
    ),
    (
        "file_tree.hover_background",
        " #file-library-tree .file-tree-node:not(.active):has(> .file-node-content:hover) > .file-node-background",
        "background-color",
    ),
    (
        "file_tree.active_background",
        " #file-library-tree .file-tree-node.active > .file-node-background",
        "background-color",
    ),
    (
        "file_tree.active_text",
        " #file-library-tree .file-tree-node.active > .file-node-content",
        "color",
    ),
    (
        "file_tree.active_border",
        " #file-library-tree .file-tree-node.active > .file-node-background",
        "border-color",
    ),
    (
        "file_tree.selection_border",
        " #file-library-tree .file-tree-node.select:not(.file-node-root) > .file-node-background",
        "outline-color",
    ),
    (
        "file_tree.expander",
        " #file-library-tree .file-node-content > .file-node-open-state",
        "color",
    ),
];
const EDITORS: &[(&str, &[&str])] = &[
    ("windows-terminal", &["cursorColor", "selectionBackground"]),
    ("iterm2", &["Cursor Color", "Selected Text Color"]),
    ("vscode", &["editorCursor.foreground", "sideBar.background"]),
    ("vim", &["LineNr", "Comment"]),
    ("helix", &["ui.linenr", "ui.linenr.selected"]),
    (
        "typora",
        &[
            "--lbrx-quote",
            "--lbrx-link",
            "--lbrx-footnote-reference",
            "--blur-text-color",
            "--lbrx-live-edit-marker",
            "--lbrx-live-edit-link-text",
            "--lbrx-live-edit-link-url",
            "--lbrx-code-selection-background",
            "--lbrx-code-selection-text",
            "--lbrx-code-active-line-background",
            "--lbrx-code-matching-bracket",
            "--lbrx-code-nonmatching-bracket",
        ],
    ),
    ("obsidian", &["--link-external-color", "--text-muted"]),
];

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Layers {
    pub common: Mapping,
    pub languages: BTreeMap<String, Mapping>,
    #[serde(skip_serializing_if = "TokenStyles::is_empty")]
    pub common_styles: TokenStyles,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub language_styles: BTreeMap<String, TokenStyles>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_source_mode: Option<TyporaSourceMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_toc: Option<TyporaToc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_sidebar: Option<TyporaSidebar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_editing_aids: Option<TyporaEditingAids>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_diagrams: Option<TyporaDiagrams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_print: Option<TyporaPrint>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaPrint {
    pub paper_color: Option<String>,
    pub text_color: Option<String>,
    pub link_color: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub keep_heading_with_next: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub keep_figure_with_caption: bool,
}

impl TyporaPrint {
    fn validate(&self, path: &Path, theme: &Theme) -> Result<(), String> {
        if self
            .paper_color
            .as_deref()
            .is_some_and(|color| crate::resolve::parse_rgb(color).is_none())
        {
            return Err(format!(
                "{}: print.paper_color: #RRGGBB 형식의 색이어야 합니다",
                path.display()
            ));
        }
        for (key, role) in [
            ("print.text_color", self.text_color.as_deref()),
            ("print.link_color", self.link_color.as_deref()),
        ] {
            if let Some(role) = role {
                validate_role(path, key, role, theme)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaDiagrams {
    pub mermaid: Option<TyporaMermaid>,
    pub sequence: Option<TyporaSequence>,
    pub flow: Option<TyporaFlow>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaMermaid {
    pub light_theme: Option<TyporaMermaidTheme>,
    pub dark_theme: Option<TyporaMermaidTheme>,
    pub alignment: Option<TyporaAlignment>,
    pub sequence_numbers: Option<bool>,
    pub flowchart_curve: Option<TyporaMermaidCurve>,
    pub gantt_left_padding_px: Option<u16>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaMermaidTheme {
    Base,
    Default,
    Dark,
    Forest,
    Neutral,
    Night,
}

impl TyporaMermaidTheme {
    pub const fn css_value(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Default => "default",
            Self::Dark => "dark",
            Self::Forest => "forest",
            Self::Neutral => "neutral",
            Self::Night => "night",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaMermaidCurve {
    Linear,
    Basis,
    Step,
    Natural,
}

impl TyporaMermaidCurve {
    pub const fn css_value(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Basis => "basis",
            Self::Step => "step",
            Self::Natural => "natural",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaSequence {
    pub theme: Option<TyporaSequenceTheme>,
    pub alignment: Option<TyporaAlignment>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaSequenceTheme {
    Simple,
    Hand,
}

impl TyporaSequenceTheme {
    pub const fn css_value(self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Hand => "hand",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaFlow {
    pub alignment: Option<TyporaAlignment>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TyporaEditingAids {
    #[serde(skip_serializing_if = "Mapping::is_empty")]
    yaml: Mapping,
    #[serde(skip_serializing_if = "Mapping::is_empty")]
    search: Mapping,
    #[serde(skip_serializing_if = "Mapping::is_empty")]
    link: Mapping,
}

impl TyporaEditingAids {
    pub fn groups(&self) -> [(&'static str, &Mapping); 3] {
        [
            ("yaml", &self.yaml),
            ("search", &self.search),
            ("link", &self.link),
        ]
    }

    fn validate(&self, path: &Path, theme: &Theme) -> Result<(), String> {
        for (group, colors) in self.groups() {
            for (name, role) in colors {
                let key = format!("{group}.{name}");
                let field = format!("editing_aids.{key}");
                if !TYPORA_EDITING_AID_COLORS
                    .iter()
                    .any(|(name, _, _)| *name == key)
                {
                    return Err(format!("{}: 지원하지 않는 키 '{field}'", path.display()));
                }
                validate_role(path, &field, role, theme)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaSidebar {
    pub background: Option<String>,
    pub outline: Option<TyporaSidebarOutline>,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    pub file_tree: Mapping,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaSidebarOutline {
    pub indent_step_px: Option<u16>,
    pub numbered: Option<bool>,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    pub colors: Mapping,
}

impl TyporaSidebar {
    pub fn color(&self, key: &str) -> Option<&str> {
        if let Some(key) = key.strip_prefix("outline.") {
            self.outline.as_ref()?.colors.get(key).map(String::as_str)
        } else if let Some(key) = key.strip_prefix("file_tree.") {
            self.file_tree.get(key).map(String::as_str)
        } else if key == "background" {
            self.background.as_deref()
        } else {
            None
        }
    }

    fn validate(&self, path: &Path, theme: &Theme) -> Result<(), String> {
        if let Some(role) = &self.background {
            validate_role(path, "sidebar.background", role, theme)?;
        }
        for (prefix, colors) in [("file_tree.", &self.file_tree)].into_iter().chain(
            self.outline
                .iter()
                .map(|outline| ("outline.", &outline.colors)),
        ) {
            for (name, role) in colors {
                let key = format!("{prefix}{name}");
                let field = if prefix == "outline." {
                    format!("sidebar.outline.colors.{name}")
                } else {
                    format!("sidebar.{key}")
                };
                if !TYPORA_SIDEBAR_COLORS
                    .iter()
                    .any(|(name, _, _)| *name == key)
                {
                    return Err(format!("{}: 지원하지 않는 키 '{field}'", path.display()));
                }
                validate_role(path, &field, role, theme)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaToc {
    pub max_depth: Option<u8>,
    pub item_gap_px: Option<u16>,
    pub link_color: Option<String>,
    pub hover_color: Option<String>,
    pub underline: Option<bool>,
    pub numbered: Option<bool>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub indent_px: BTreeMap<TyporaTocLevel, u16>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaTocLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl TyporaToc {
    fn validate(&self, path: &Path, theme: &Theme) -> Result<(), String> {
        if self
            .max_depth
            .is_some_and(|depth| !(1..=6).contains(&depth))
        {
            return Err(format!(
                "{}: toc.max_depth: 1–6이어야 합니다",
                path.display()
            ));
        }
        for (key, role) in [
            ("toc.link_color", self.link_color.as_deref()),
            ("toc.hover_color", self.hover_color.as_deref()),
        ] {
            if let Some(role) = role {
                validate_role(path, key, role, theme)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaSourceMode {
    pub font: Option<TyporaSourceFont>,
    pub size_px: Option<u16>,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    pub colors: Mapping,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaSourceFont {
    Prose,
    Mono,
}

impl TyporaSourceMode {
    fn validate(&self, path: &Path, theme: &Theme) -> Result<(), String> {
        if self.size_px == Some(0) {
            return Err(format!(
                "{}: source_mode.size_px: 글자 크기는 양수여야 합니다",
                path.display()
            ));
        }
        for (key, role) in &self.colors {
            let option = format!("source_mode.colors.{key}");
            if !TYPORA_SOURCE_COLORS.iter().any(|(name, _, _)| name == key) {
                return Err(format!("{}: 지원하지 않는 키 '{option}'", path.display()));
            }
            validate_role(path, &option, role, theme)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaAlerts {
    pub shaded_background: bool,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    pub colors: Mapping,
}

impl TyporaAlerts {
    fn validate(&self, path: &Path, theme: &Theme) -> Result<(), String> {
        for (kind, role) in &self.colors {
            if !TYPORA_ALERT_ROLES.iter().any(|(key, _)| key == kind) {
                return Err(format!(
                    "{}: alerts.colors.{kind}: 지원하지 않는 알림 유형",
                    path.display()
                ));
            }
            validate_role(path, &format!("alerts.colors.{kind}"), role, theme)?;
        }
        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<TyporaAlignment>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TyporaAlignment {
    Start,
    Center,
    End,
}

impl TyporaAlignment {
    pub const fn margins(self) -> &'static str {
        match self {
            Self::Start => "0 auto",
            Self::Center => "auto",
            Self::End => "auto 0",
        }
    }

    pub const fn css_value(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaTaskLists {
    pub strike_completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaLists {
    pub indent_px: std::num::NonZeroU16,
    pub item_gap_px: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markers: Option<TyporaListMarkers>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TyporaListMarkers {
    level_1: Option<TyporaListMarker>,
    level_2: Option<TyporaListMarker>,
    level_3: Option<TyporaListMarker>,
}

impl TyporaListMarkers {
    pub fn styles(&self) -> [&'static str; 3] {
        [
            self.level_1.unwrap_or(TyporaListMarker::Disc),
            self.level_2.unwrap_or(TyporaListMarker::Circle),
            self.level_3.unwrap_or(TyporaListMarker::Square),
        ]
        .map(TyporaListMarker::css_value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum TyporaListMarker {
    Disc,
    Circle,
    Square,
}

impl TyporaListMarker {
    const fn css_value(self) -> &'static str {
        match self {
            Self::Disc => "disc",
            Self::Circle => "circle",
            Self::Square => "square",
        }
    }
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
    #[serde(default)]
    tokens: Mapping,
    #[serde(default)]
    styles: TokenStyles,
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
    source_mode: Option<TyporaSourceMode>,
    toc: Option<TyporaToc>,
    sidebar: Option<TyporaSidebar>,
    editing_aids: Option<TyporaEditingAids>,
    diagrams: Option<TyporaDiagrams>,
    print: Option<TyporaPrint>,
}

fn read<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn validate_role(path: &Path, key: &str, role: &str, theme: &Theme) -> Result<(), String> {
    for (name, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
        if !mode.roles.contains_key(role) {
            return Err(format!(
                "{}: {key}: {name}.roles에 '{role}' 없음",
                path.display()
            ));
        }
    }
    Ok(())
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
                let config = read::<LanguageFile>(&file)?;
                for key in config.styles.keys() {
                    if !TOKENS.contains(&key.as_str()) {
                        return Err(format!(
                            "{}: styles: 지원하지 않는 키 '{key}'",
                            file.display()
                        ));
                    }
                }
                if id == "common" {
                    theme.layers.common_styles = config.styles;
                } else if !config.styles.is_empty() {
                    theme
                        .layers
                        .language_styles
                        .insert(id.into(), config.styles);
                }
                (config.tokens, TOKENS)
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
                    if let Some(alerts) = &config.alerts {
                        alerts.validate(&file, &theme)?;
                    }
                    if let Some(source) = &config.source_mode {
                        source.validate(&file, &theme)?;
                    }
                    if let Some(toc) = &config.toc {
                        toc.validate(&file, &theme)?;
                    }
                    if let Some(sidebar) = &config.sidebar {
                        sidebar.validate(&file, &theme)?;
                    }
                    if let Some(aids) = &config.editing_aids {
                        aids.validate(&file, &theme)?;
                    }
                    if let Some(print) = &config.print {
                        print.validate(&file, &theme)?;
                    }
                    if let Some(role) = config
                        .task_lists
                        .as_ref()
                        .and_then(|tasks| tasks.checkbox_color.as_deref())
                    {
                        validate_role(&file, "task_lists.checkbox_color", role, &theme)?;
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
                    theme.layers.typora_source_mode = config.source_mode;
                    theme.layers.typora_toc = config.toc;
                    theme.layers.typora_sidebar = config.sidebar;
                    theme.layers.typora_editing_aids = config.editing_aids;
                    theme.layers.typora_diagrams = config.diagrams;
                    theme.layers.typora_print = config.print;
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
                validate_role(&file, key, role, &theme)?;
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
        let sidecar = dir.join("typora.toml");
        let base = "[lists]\nindent_px=20\nitem_gap_px=2";
        fs::write(&sidecar, base).unwrap();
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
        for (config, expected) in [
            ("level_2 = 'disc'", ["disc", "disc", "square"]),
            (
                "level_1 = 'square'\nlevel_2 = 'disc'\nlevel_3 = 'circle'",
                ["square", "disc", "circle"],
            ),
        ] {
            fs::write(&sidecar, format!("{base}\n[lists.markers]\n{config}")).unwrap();
            for (_, css) in render().iter().take(2) {
                let rules: Vec<_> = css
                    .lines()
                    .filter(|line| line.contains("list-style-type:"))
                    .collect();
                assert_eq!(rules.len(), expected.len());
                for ((rule, marker), ancestors) in rules.iter().zip(expected).zip(0..3) {
                    assert!(
                        rule.starts_with(&format!("#write {}ul", ":is(ol, ul) ".repeat(ancestors)))
                    );
                    assert!(rule.ends_with(&format!("{{ list-style-type: {marker}; }}")));
                }
            }
        }
        fs::write(
            &sidecar,
            format!("{base}\n[lists.markers]\nlevel_1 = 'triangle'"),
        )
        .unwrap();
        let error = load(&path).err().unwrap();
        assert!(
            error.contains("typora.toml")
                && error.contains("level_1")
                && error.contains("triangle"),
            "{error}"
        );
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
    fn typora_checkbox_color_changes_only_the_task_checkbox_accent() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let base = "[task_lists]\nstrike_completed = true";
        fs::write(&sidecar, base).unwrap();
        let baseline = render();
        fs::write(&sidecar, format!("{base}\ncheckbox_color = 'warning'")).unwrap();
        for (((_, before), (_, after)), (default, custom)) in baseline
            .iter()
            .zip(render().iter())
            .zip([("#246958", "#815919"), ("#84C7AC", "#D6B378")])
        {
            let original = format!("accent-color: {default};");
            assert!(before.contains(&original));
            assert_eq!(
                *after,
                before.replace(&original, &format!("accent-color: {custom};"))
            );
        }
        fs::write(&sidecar, format!("{base}\ncheckbox_color = 'unknown'")).unwrap();
        let error = load(&path).err().unwrap();
        assert!(
            error.contains("typora.toml")
                && error.contains("task_lists.checkbox_color")
                && error.contains("unknown")
        );
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
            assert!(!css.contains("p > img:only-child"));
        }
        for (alignment, margins) in [("start", "0 auto"), ("center", "auto"), ("end", "auto 0")] {
            fs::write(
                dir.join("typora.toml"),
                format!("[images]\ncaption_size_percent = 85\ncaption_gap_px = 10\nalignment = '{alignment}'"),
            )
            .unwrap();
            for (_, css) in render().iter().take(2) {
                assert!(css.contains(&format!(
                    "p > img:only-child {{ margin-inline: {margins}; }}"
                )));
                assert!(css.contains(&format!("block-size: auto;\n  margin-inline: {margins};")));
                assert!(css.contains("color: var(--lbrx-muted);\n  text-align: center;"));
            }
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
    fn typora_alert_color_override_changes_only_the_named_alert() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let base_config = "[alerts]\nshaded_background = true";
        fs::write(&sidecar, base_config).unwrap();
        let baseline = render();
        fs::write(
            &sidecar,
            format!("{base_config}\n[alerts.colors]\nnote = 'accent'"),
        )
        .unwrap();
        for (((_, base), (_, css)), (original, accent)) in baseline
            .iter()
            .zip(render().iter())
            .zip([("#43617B", "#246958"), ("#97BBD6", "#84C7AC")])
        {
            let original_rule = format!(
                "#write .md-alert.md-alert-note {{ border-left-color: {original}; }}\n\
                 #write .md-alert .md-alert-text-note {{ color: {original}; }}"
            );
            assert!(base.contains(&original_rule));
            assert_eq!(
                *css,
                base.replace(&original_rule, &original_rule.replace(original, accent))
            );
        }
        for (kind, role, message) in [
            ("unknown", "accent", "지원하지 않는 알림 유형"),
            ("note", "missing", "light.roles"),
        ] {
            fs::write(
                &sidecar,
                format!("{base_config}\n[alerts.colors]\n{kind} = '{role}'"),
            )
            .unwrap();
            let error = load(&path).err().unwrap();
            assert!(error.contains("typora.toml"));
            assert!(error.contains(&format!("alerts.colors.{kind}")) && error.contains(message));
        }
    }

    #[test]
    fn typora_focus_mode_only_emits_a_color_when_overridden() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let baseline = render();
        fs::write(&sidecar, "[colors]\n'--blur-text-color' = 'muted'").unwrap();
        let custom = render();
        for (((_, before), (_, after)), muted) in
            baseline.iter().zip(&custom).zip(["#52655B", "#A3B5A9"])
        {
            assert!(!before.contains("--blur-text-color:"));
            let declaration = format!("  --blur-text-color: {muted};\n");
            assert!(after.contains(&declaration));
            assert_eq!(*before, after.replacen(&declaration, "", 1));
        }
        fs::write(&sidecar, "[colors]\n'--blur-text-color' = 'unknown'").unwrap();
        let error = load(&path).err().unwrap();
        assert!(error.contains("typora.toml") && error.contains("--blur-text-color"));
    }

    #[test]
    fn typora_live_edit_colors_are_optional_and_keep_url_separate_from_markers() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let baseline = render();
        assert!(!baseline[0].1.contains("[md-inline="));

        fs::write(&sidecar, "[colors]\n'--lbrx-live-edit-marker' = 'muted'").unwrap();
        for (_, css) in render().iter().take(2) {
            assert!(css.contains(
                "> .md-meta > :is(.md-url, .md-link-title) { color: var(--md-char-color, #BBB); }"
            ));
            assert!(!css.contains(" > a { color:"));
        }

        fs::write(
            &sidecar,
            "[colors]\n'--lbrx-live-edit-link-text' = 'accent'\n'--lbrx-live-edit-link-url' = 'function'",
        )
        .unwrap();
        let [(_, light), (_, dark), _] = render();
        for (css, text, url) in [(light, "#246958", "#43617B"), (dark, "#84C7AC", "#97BBD6")] {
            assert!(css.contains(&format!("[md-inline=\"link\"]:is(.md-expand, .md-expand-on-selection) > a {{ color: {text}; }}")));
            assert!(css.contains(&format!("[md-inline=\"link\"]:is(.md-expand, .md-expand-on-selection) > .md-meta > .md-url {{ color: {url}; }}")));
            assert!(!css.contains("var(--md-char-color, #BBB)"));
        }

        fs::write(
            &sidecar,
            "[colors]\n'--lbrx-live-edit-link-url' = 'unknown'",
        )
        .unwrap();
        let error = load(&path).err().unwrap();
        assert!(error.contains("typora.toml") && error.contains("--lbrx-live-edit-link-url"));
    }

    #[test]
    fn typora_code_editing_overrides_are_independent_and_scoped_to_fences() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let baseline = render();
        let editor = "#write .md-fences .CodeMirror";
        for (option, role, suffix, property) in [
            (
                "selection-background",
                "surface",
                "",
                "--select-text-bg-color",
            ),
            ("selection-text", "text", "", "--select-text-font-color"),
            (
                "active-line-background",
                "surface",
                " .CodeMirror-activeline-background",
                "background-color",
            ),
            (
                "matching-bracket",
                "success",
                " .CodeMirror-matchingbracket",
                "color",
            ),
            (
                "nonmatching-bracket",
                "error",
                " .CodeMirror-nonmatchingbracket",
                "color",
            ),
        ] {
            fs::write(
                &sidecar,
                format!("[colors]\n'--lbrx-code-{option}' = '{role}'"),
            )
            .unwrap();
            let resolved = resolve::resolve(&load(&path).unwrap()).unwrap();
            let custom = crate::typora::render(&resolved);
            for (((_, before), (_, after)), mode) in baseline
                .iter()
                .zip(&custom)
                .zip([&resolved.light, &resolved.dark])
            {
                let rule = format!("{editor}{suffix} {{ {property}: {}; }}\n", mode.roles[role]);
                assert!(!before.contains(&rule));
                assert!(after.contains(&rule));
                assert_eq!(*before, after.replacen(&rule, "", 1));
            }
        }
        fs::write(
            &sidecar,
            "[colors]\n'--lbrx-code-matching-bracket' = 'unknown'",
        )
        .unwrap();
        assert!(
            matches!(load(&path), Err(error) if error.contains("--lbrx-code-matching-bracket") && error.contains("unknown"))
        );
    }

    #[test]
    fn typora_source_mode_only_changes_the_source_editor() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let baseline = crate::typora::render(&resolve());
        let editor = "#typora-source .CodeMirror.cm-s-typora-default";

        fs::write(
            &sidecar,
            "[source_mode.colors]\nselection_background = 'surface'\nheading = 'accent'",
        )
        .unwrap();
        let resolved = resolve();
        for (((_, before), (_, after)), mode) in baseline
            .iter()
            .zip(crate::typora::render(&resolved))
            .zip([&resolved.light, &resolved.dark])
            .take(2)
        {
            let selection = format!(
                "{editor} {{ --select-text-bg-color: {}; }}\n",
                mode.roles["surface"]
            );
            let heading = format!(
                "{editor} :is(.cm-header, .cm-property) {{ color: {}; }}\n",
                mode.roles["accent"]
            );
            assert!(after.contains(&selection) && after.contains(&heading));
            assert_eq!(
                *before,
                after.replacen(&selection, "", 1).replacen(&heading, "", 1)
            );
        }

        fs::write(&sidecar, "[source_mode]\nfont = 'mono'\nsize_px = 15").unwrap();
        for ((_, before), (_, after)) in baseline
            .iter()
            .zip(crate::typora::render(&resolve()))
            .take(2)
        {
            let font = format!(
                "{editor} {{ font-family: var(--lbrx-mono-family); --monospace: var(--lbrx-mono-family); }}\n"
            );
            let size = format!("{editor} {{ font-size: 15px; }}\n");
            assert!(after.contains(&font) && after.contains(&size));
            assert_eq!(*before, after.replacen(&font, "", 1).replacen(&size, "", 1));
        }
    }

    #[test]
    fn typora_source_mode_rejects_invalid_settings() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");

        for (config, field) in [
            ("[source_mode]\nsize_px = 0", "source_mode.size_px"),
            ("[source_mode]\nfont = 'unknown'", "font"),
            (
                "[source_mode.colors]\nbackground = 'unknown'",
                "source_mode.colors.background",
            ),
            (
                "[source_mode.colors]\nunknown = 'text'",
                "source_mode.colors.unknown",
            ),
        ] {
            fs::write(&sidecar, config).unwrap();
            assert!(
                matches!(load(&path), Err(e) if e.contains("typora.toml") && e.contains(field))
            );
        }
    }

    #[test]
    fn typora_toc_overrides_only_target_the_document_toc() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let before = crate::typora::render(&resolve());

        fs::write(&sidecar, "[toc]\nmax_depth = 6\nnumbered = false").unwrap();
        assert_eq!(crate::typora::render(&resolve()), before);

        fs::write(
            &sidecar,
            "[toc]\nmax_depth = 3\nitem_gap_px = 4\nlink_color = 'accent'\n\
             hover_color = 'function'\nunderline = false\nnumbered = true\n\
             [toc.indent_px]\nh2 = 16\nh3 = 32",
        )
        .unwrap();
        let resolved = resolve();
        let rendered = crate::typora::render(&resolved);
        let content = "#write .md-toc > .md-toc-content";
        let item = format!("{content} > .md-toc-item");
        let link = format!("{item} > a.md-toc-inner");
        for (index, mode) in [&resolved.light, &resolved.dark].into_iter().enumerate() {
            let (_, css) = &rendered[index];
            let mut unchanged = css.clone();
            for rule in [
                format!("{item}:is(.md-toc-h4, .md-toc-h5, .md-toc-h6) {{ display: none; }}"),
                format!("{content}:not(:empty) {{ display: grid; row-gap: 4px; }}"),
                format!(
                    "{item}.md-toc-h2 > a.md-toc-inner {{ margin-left: 0; margin-inline-start: 16px; }}"
                ),
                format!(
                    "{item}.md-toc-h3 > a.md-toc-inner {{ margin-left: 0; margin-inline-start: 32px; }}"
                ),
                format!("{link} {{ color: {}; }}", mode.roles["accent"]),
                format!(
                    "{link}:is(:hover, :focus-visible) {{ color: {}; }}",
                    mode.roles["function"]
                ),
                format!("{link} {{ text-decoration: none; }}"),
                format!("{content} {{ counter-reset: lbrx-toc; }}"),
                format!("{item} {{ counter-increment: lbrx-toc; }}"),
                format!("{link}::before {{ content: counter(lbrx-toc) \". \"; }}"),
            ] {
                assert!(css.contains(&rule));
                unchanged = unchanged.replacen(&format!("{rule}\n"), "", 1);
            }
            assert_eq!(before[index].1, unchanged);
        }
    }

    #[test]
    fn typora_toc_rejects_invalid_depth_roles_and_levels() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for (config, field) in [
            ("[toc]\nmax_depth = 0", "toc.max_depth"),
            ("[toc]\nmax_depth = 7", "toc.max_depth"),
            ("[toc]\nlink_color = 'unknown'", "toc.link_color"),
            ("[toc]\nhover_color = 'unknown'", "toc.hover_color"),
            ("[toc.indent_px]\nh7 = 16", "h7"),
        ] {
            fs::write(&sidecar, config).unwrap();
            assert!(
                matches!(load(&path), Err(e) if e.contains("typora.toml") && e.contains(field))
            );
        }
    }

    #[test]
    fn typora_sidebar_keeps_outline_state_separate_from_file_tree_state() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let baseline = crate::typora::render(&resolve());

        fs::write(&sidecar, "[sidebar.outline]\nnumbered = false").unwrap();
        assert_eq!(crate::typora::render(&resolve()), baseline);

        fs::write(
            &sidecar,
            "[sidebar.outline]\nnumbered = true\n\
             [sidebar.outline.colors]\nactive_background = 'surface'\n\
             [sidebar.file_tree]\nactive_background = 'selection_background'\n\
             selection_border = 'focus'",
        )
        .unwrap();
        let resolved = resolve();
        let custom = crate::typora::render(&resolved);
        let panel = "#typora-sidebar #outline-content";
        let tree = "#typora-sidebar #file-library-tree";
        for (i, mode) in [&resolved.light, &resolved.dark].into_iter().enumerate() {
            let css = &custom[i].1;
            for (selector, property, role) in [
                (
                    format!("{panel} .outline-item.outline-item-active"),
                    "background-color",
                    "surface",
                ),
                (
                    format!("{tree} .file-tree-node.active > .file-node-background"),
                    "background-color",
                    "selection_background",
                ),
                (
                    format!(
                        "{tree} .file-tree-node.select:not(.file-node-root) > .file-node-background"
                    ),
                    "outline-color",
                    "focus",
                ),
            ] {
                assert!(css.contains(&format!(
                    "{selector} {{ {property}: {}; }}",
                    mode.roles[role]
                )));
            }
            assert!(css.contains(&format!(
                "{panel} .outline-item > .outline-label::before {{ content: counter(lbrx-outline) "
            )));
            let non_sidebar = css
                .split_inclusive('\n')
                .filter(|line| !line.starts_with("#typora-sidebar "))
                .collect::<String>();
            assert_eq!(baseline[i].1, non_sidebar);
        }
    }

    #[test]
    fn typora_sidebar_rejects_unknown_roles_and_panel_options() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for (config, field) in [
            ("[sidebar]\nbackground = 'unknown'", "sidebar.background"),
            (
                "[sidebar.file_tree]\nnumbered = 'text'",
                "sidebar.file_tree.numbered",
            ),
            (
                "[sidebar.outline.colors]\nactive_text = 'unknown'",
                "sidebar.outline.colors.active_text",
            ),
        ] {
            fs::write(&sidecar, config).unwrap();
            assert!(
                matches!(load(&path), Err(e) if e.contains("typora.toml") && e.contains(field))
            );
        }
    }

    #[test]
    fn typora_editing_aids_only_add_rules_for_enabled_targets() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let baseline = crate::typora::render(&resolve());

        fs::write(&sidecar, "[editing_aids.yaml]").unwrap();
        assert_eq!(crate::typora::render(&resolve()), baseline);
        fs::write(
            &sidecar,
            "[editing_aids.yaml]\nplaceholder = 'muted'\n\
             [editing_aids.search]\nmatch_background = 'prose_selection_background'\n\
             active_text = 'selection_text'\n\
             [editing_aids.link]\ndestination_background = 'surface'",
        )
        .unwrap();
        let theme = resolve();
        let css = crate::typora::render(&theme);
        for (index, mode) in [&theme.light, &theme.dark].into_iter().enumerate() {
            let mut unchanged = css[index].1.clone();
            for (selector, property, role) in [
                ("#write > pre.md-meta-block:empty::before", "color", "muted"),
                (
                    "#write .md-search-hit:not(.md-search-select)",
                    "background-color",
                    "prose_selection_background",
                ),
                (
                    "#write::highlight(search-hit)",
                    "background-color",
                    "prose_selection_background",
                ),
                (
                    "#write ::highlight(search-hit)",
                    "background-color",
                    "prose_selection_background",
                ),
                ("#write .md-search-select", "color", "selection_text"),
                (
                    "#write::highlight(search-select)",
                    "color",
                    "selection_text",
                ),
                (
                    "#write ::highlight(search-select)",
                    "color",
                    "selection_text",
                ),
                (
                    r#"#write [md-inline="link"]:is(.md-expand, .md-expand-on-selection) > .md-meta > .md-url"#,
                    "background-color",
                    "surface",
                ),
            ] {
                let rule = format!("{selector} {{ {property}: {}; }}\n", mode.roles[role]);
                assert!(unchanged.contains(&rule));
                unchanged = unchanged.replacen(&rule, "", 1);
            }
            assert_eq!(baseline[index].1, unchanged);
        }
    }

    #[test]
    fn typora_editing_aids_reject_unknown_roles_and_options() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for (config, field) in [
            (
                "[editing_aids.yaml]\ntext = 'unknown'",
                "editing_aids.yaml.text",
            ),
            (
                "[editing_aids.search]\nmatch_text = 'text'",
                "editing_aids.search.match_text",
            ),
        ] {
            fs::write(&sidecar, config).unwrap();
            assert!(
                matches!(load(&path), Err(e) if e.contains("typora.toml") && e.contains(field))
            );
        }
    }

    #[test]
    fn typora_diagram_options_emit_native_values_and_scope_preview_alignment() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let baseline = render();

        fs::write(
            &sidecar,
            "[diagrams.mermaid]\nlight_theme = 'neutral'\nalignment = 'end'\n\
             sequence_numbers = false\nflowchart_curve = 'basis'\ngantt_left_padding_px = 0\n\
             [diagrams.sequence]\ntheme = 'hand'\nalignment = 'center'\n\
             [diagrams.flow]\nalignment = 'start'",
        )
        .unwrap();
        let configured = render();
        for (index, (_, css)) in configured[..2].iter().enumerate() {
            let mut unchanged = css.clone();
            let mut declarations = vec![
                "  --mermaid-sequence-numbers: off;\n",
                "  --mermaid-flowchart-curve: basis;\n",
                "  --mermaid--gantt-left-padding: 0;\n",
                "  --sequence-theme: hand;\n",
            ];
            if index == 0 {
                declarations.push("  --mermaid-theme: neutral;\n");
            }
            for declaration in declarations {
                assert!(unchanged.contains(declaration), "{declaration}");
                unchanged = unchanged.replacen(declaration, "", 1);
            }
            for (language, align, margins) in [
                ("mermaid", "end", "auto 0"),
                ("sequence", "center", "auto"),
                ("flow", "start", "0 auto"),
            ] {
                let preview =
                    format!("#write .md-fences[lang=\"{language}\"] .md-diagram-panel-preview");
                let rules = format!(
                    "{preview} {{ text-align: {align}; }}\n\
                     {preview} > svg {{ display: block; margin-inline: {margins}; }}\n"
                );
                assert!(unchanged.contains(&rules), "{rules}");
                unchanged = unchanged.replacen(&rules, "", 1);
            }
            assert_eq!(baseline[index].1, unchanged);
        }

        fs::write(
            &sidecar,
            "[diagrams.mermaid]\ndark_theme = 'dark'\nsequence_numbers = true",
        )
        .unwrap();
        let [(_, light), (_, dark), _] = render();
        assert!(!light.contains("--mermaid-theme:"));
        assert!(dark.contains("--mermaid-theme: dark;"));
        assert!(dark.contains("--mermaid-sequence-numbers: on;"));
    }

    #[test]
    fn typora_diagram_options_reject_values_the_renderer_does_not_support() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for (table, key, value) in [
            ("mermaid", "dark_theme", "'hand'"),
            ("mermaid", "flowchart_curve", "'curvy'"),
            ("mermaid", "gantt_left_padding_px", "-1"),
            ("sequence", "theme", "'neutral'"),
            ("flow", "alignment", "'justify'"),
        ] {
            fs::write(&sidecar, format!("[diagrams.{table}]\n{key} = {value}")).unwrap();
            assert!(matches!(load(&path), Err(e) if e.contains("typora.toml") && e.contains(key)));
        }
    }

    #[test]
    fn typora_print_uses_light_colors_and_only_changes_print_media() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let baseline = crate::typora::render(&resolve());

        fs::write(&sidecar, "[print]\nkeep_heading_with_next = false").unwrap();
        assert_eq!(crate::typora::render(&resolve()), baseline);

        fs::write(
            &sidecar,
            "[print]\npaper_color = '#FFFFFF'\ntext_color = 'text'\nlink_color = 'link'\n\
             keep_heading_with_next = true\nkeep_figure_with_caption = true",
        )
        .unwrap();
        let theme = resolve();
        let configured = crate::typora::render(&theme);
        let mut light_rules = None;
        for (i, (_, css)) in configured[..2].iter().enumerate() {
            let (screen, rules) = css.rsplit_once("\n@media print {\n").unwrap();
            assert_eq!(screen, baseline[i].1);
            assert!(rules.ends_with("}\n"));
            assert!(rules.contains("background-color: #FFFFFF;"));
            assert!(rules.contains(&format!("color: {};", theme.light.roles["text"])));
            assert!(rules.contains(&format!(
                "#write a {{ color: {}; }}",
                theme.light.roles["link"]
            )));
            assert!(rules.contains(":not(:last-child) { break-after: avoid-page; }"));
            assert!(rules.contains(":has(> figcaption) { break-inside: avoid-page; }"));
            assert!(rules.contains("figure:not(.table-figure):has(> img"));
            if let Some(light_rules) = light_rules {
                assert_eq!(rules, light_rules);
            } else {
                light_rules = Some(rules);
            }
        }
    }

    #[test]
    fn typora_print_rejects_invalid_colors() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../themes/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for (key, value) in [("paper_color", "#fff"), ("link_color", "unknown")] {
            fs::write(&sidecar, format!("[print]\n{key} = '{value}'")).unwrap();
            assert!(
                matches!(load(&path), Err(e) if e.contains("typora.toml") && e.contains(&format!("print.{key}")))
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
