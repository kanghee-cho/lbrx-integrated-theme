use crate::model::Theme;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, fs, path::Path};

pub type Mapping = BTreeMap<String, String>;
pub const DOCUMENT_ALERT_ROLES: [(&str, &str); 5] = [
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
        &[
            "#write > pre.md-meta-block",
            "#write > pre.md-meta-block :is(.CodeMirror, .CodeMirror-gutters)",
        ],
        "background-color",
    ),
    (
        "yaml.text",
        &[
            "#write > pre.md-meta-block",
            "#write > pre.md-meta-block .CodeMirror",
        ],
        "color",
    ),
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
    ("lbrx-studio", crate::lbrx_studio::COLOR_KEYS),
    ("windows-terminal", &["cursorColor", "selectionBackground"]),
    ("iterm2", crate::iterm2::COLOR_KEYS),
    ("macos-terminal", crate::macos_terminal::COLOR_KEYS),
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
    (
        "obsidian",
        &[
            "--link-color",
            "--link-color-hover",
            "--link-external-color",
            "--link-external-color-hover",
            "--text-muted",
            "--h1-color",
            "--h2-color",
            "--h3-color",
            "--h4-color",
            "--h5-color",
            "--h6-color",
            "--inline-title-color",
            crate::obsidian::INLINE_CODE_COLOR,
            crate::obsidian::FOOTNOTE_REFERENCE_COLOR,
        ],
    ),
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
    pub obsidian_prose: Option<ObsidianLineHeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_prose_detail: Option<ObsidianProseDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_code: Option<ObsidianCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_layout: Option<DocumentLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_paragraph: Option<DocumentParagraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_headings: Option<DocumentHeadings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_links: Option<ObsidianLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_blockquote: Option<ObsidianBlockquote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_emphasis: Option<DocumentEmphasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_footnotes: Option<DocumentFootnotes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_horizontal_rule: Option<DocumentHorizontalRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_table: Option<DocumentTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_lists: Option<DocumentLists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_task_lists: Option<DocumentTaskLists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_images: Option<DocumentImages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_math: Option<DocumentMath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_callouts: Option<DocumentAlerts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_live_preview: Option<ObsidianLivePreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_source_mode: Option<ObsidianSourceMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_internal_links: Option<ObsidianInternalLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_tags_and_properties: Option<ObsidianTagsAndProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_navigation_sidebar: Option<ObsidianNavigationSidebar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_workspace: Option<ObsidianWorkspace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_dialogs: Option<ObsidianDialogs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_search: Option<ObsidianSearch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_graph: Option<ObsidianGraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_canvas: Option<ObsidianCanvas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_bases: Option<ObsidianBases>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_mermaid: Option<ObsidianMermaid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_icons: Option<ObsidianIcons>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_mobile: Option<ObsidianMobile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian_print: Option<ObsidianPrint>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub helix_styles: BTreeMap<String, HelixStyle>,
    #[serde(skip_serializing_if = "toml::Table::is_empty")]
    pub helix_config: toml::Table,
    #[serde(skip_serializing_if = "toml::Table::is_empty")]
    pub helix_languages: toml::Table,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub vim_highlights: BTreeMap<String, VimHighlight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_layout: Option<DocumentLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_paragraph: Option<DocumentParagraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_headings: Option<DocumentHeadings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_emphasis: Option<DocumentEmphasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_footnotes: Option<DocumentFootnotes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_horizontal_rule: Option<DocumentHorizontalRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_table: Option<DocumentTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_lists: Option<DocumentLists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_task_lists: Option<DocumentTaskLists>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_images: Option<DocumentImages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_math: Option<DocumentMath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typora_alerts: Option<DocumentAlerts>,
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
    pub text: Option<String>,
    pub muted: Option<String>,
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
        for (key, role) in [("text", &self.text), ("muted", &self.muted)] {
            if let Some(role) = role {
                validate_role(path, &format!("sidebar.{key}"), role, theme)?;
            }
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
pub struct DocumentAlerts {
    pub shaded_background: bool,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    pub colors: Mapping,
}

impl DocumentAlerts {
    fn validate(&self, path: &Path, theme: &Theme, section: &str) -> Result<(), String> {
        for (kind, role) in &self.colors {
            if !DOCUMENT_ALERT_ROLES.iter().any(|(key, _)| key == kind) {
                return Err(format!(
                    "{}: {section}.colors.{kind}: 지원하지 않는 알림 유형",
                    path.display()
                ));
            }
            validate_role(path, &format!("{section}.colors.{kind}"), role, theme)?;
        }
        Ok(())
    }

    pub fn role<'a>(&'a self, kind: &str, fallback: &'a str) -> &'a str {
        self.colors.get(kind).map_or(fallback, String::as_str)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentMath {
    pub padding_block_px: u16,
    pub padding_inline_px: u16,
    pub shaded_background: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentImages {
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
pub struct DocumentTaskLists {
    pub strike_completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkbox_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLists {
    pub indent_px: std::num::NonZeroU16,
    pub item_gap_px: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markers: Option<DocumentListMarkers>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentListMarkers {
    level_1: Option<ListMarker>,
    level_2: Option<ListMarker>,
    level_3: Option<ListMarker>,
}

impl DocumentListMarkers {
    pub fn styles(&self) -> [&'static str; 3] {
        self.shapes().map(ListMarker::css_value)
    }

    pub fn shapes(&self) -> [ListMarker; 3] {
        [
            self.level_1.unwrap_or(ListMarker::Disc),
            self.level_2.unwrap_or(ListMarker::Circle),
            self.level_3.unwrap_or(ListMarker::Square),
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListMarker {
    Disc,
    Circle,
    Square,
}

impl ListMarker {
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
pub struct DocumentTable {
    pub cell_padding_block_px: u16,
    pub cell_padding_inline_px: u16,
    pub row_rule_px: std::num::NonZeroU16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentFootnotes {
    pub body_size_percent: std::num::NonZeroU16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHorizontalRule {
    pub thickness_px: std::num::NonZeroU16,
    pub margin_block_px: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentEmphasis {
    pub light: DocumentEmphasisColors,
    pub dark: DocumentEmphasisColors,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentEmphasisColors {
    pub strong: String,
    pub italic: String,
    pub strikethrough: String,
    pub highlight_background: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHeadings {
    h1: DocumentHeading,
    h2: DocumentHeading,
    h3: DocumentHeading,
    h4: DocumentHeading,
    h5: DocumentHeading,
    h6: DocumentHeading,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentHeading {
    pub size_percent: u16,
    pub weight: u16,
    pub margin_before_px: u16,
    pub margin_after_px: u16,
    pub divider: bool,
}

impl DocumentHeadings {
    pub fn levels(&self) -> [(&'static str, &DocumentHeading); 6] {
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
pub struct DocumentParagraph {
    pub gap_px: Option<u16>,
    pub alignment: Option<ParagraphAlignment>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParagraphAlignment {
    Start,
    Justify,
}

impl ParagraphAlignment {
    pub const fn css_value(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Justify => "justify",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentLayout {
    pub max_width_px: u16,
    pub padding_inline_min_px: u16,
    pub padding_inline_max_px: u16,
}

impl DocumentLayout {
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObsidianLineHeight {
    #[serde(default)]
    pub use_shared_line_height: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObsidianCode {
    #[serde(default)]
    pub use_shared_line_height: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reading_border_width_px: Option<u16>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub reading_horizontal_scroll: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObsidianProseDetail {
    pub letter_spacing_em: Option<f64>,
    pub common_ligatures: Option<bool>,
    pub small_caps: Option<bool>,
    pub text_transform: Option<ProseTextTransform>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProseTextTransform {
    None,
    Capitalize,
    Uppercase,
    Lowercase,
}

impl ProseTextTransform {
    pub const fn css_value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Capitalize => "capitalize",
            Self::Uppercase => "uppercase",
            Self::Lowercase => "lowercase",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObsidianLinks {
    pub internal_underline: Option<bool>,
    pub internal_hover_underline: Option<bool>,
    pub external_underline: Option<bool>,
    pub external_hover_underline: Option<bool>,
    #[serde(default)]
    pub keyboard_focus_outline: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObsidianBlockquote {
    pub text_role: Option<String>,
    pub border_role: Option<String>,
    pub border_width_px: Option<u16>,
    pub padding_start_em: Option<f64>,
}

impl ObsidianBlockquote {
    fn validate(&self, file: &Path, theme: &Theme) -> Result<(), String> {
        for (key, role) in [
            ("blockquote.text_role", &self.text_role),
            ("blockquote.border_role", &self.border_role),
        ] {
            if let Some(role) = role {
                validate_role(file, key, role, theme)?;
            }
        }
        if self
            .padding_start_em
            .is_some_and(|padding| !padding.is_finite() || padding < 0.0)
        {
            return Err(format!(
                "{}: blockquote.padding_start_em: 0 이상의 유한한 값이어야 합니다",
                file.display()
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianLivePreview {
    pub highlight_active_line: bool,
    pub use_focus_caret: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianSourceMode {
    pub mute_markdown_markers: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianInternalLinks {
    pub distinguish_unresolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_embed_border_width_px: Option<u16>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianTagsAndProperties {
    pub style_inline_tags: bool,
    pub style_properties: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianNavigationSidebar {
    pub file_explorer: bool,
    pub outline: bool,
    pub backlinks: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianWorkspace {
    pub style_active_tabs: bool,
    pub shade_ribbon: bool,
    pub shade_status_bar: bool,
    pub style_split_dividers: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianDialogs {
    pub style_settings: bool,
    pub style_prompts: bool,
    pub style_menus: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianSearch {
    pub highlight_note_matches: bool,
    pub highlight_vault_matches: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianGraph {
    pub use_theme_palette: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianCanvas {
    pub style_default_colors: bool,
    pub highlight_selection: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianBases {
    pub style_table: bool,
    pub style_cards: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianMermaid {
    pub use_theme_palette: bool,
    pub scroll_wide_diagrams: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianIcons {
    pub style_ribbon: bool,
    pub style_note_header: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_px: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianMobile {
    pub style_toolbar: bool,
    pub style_drawers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_padding_max_px: Option<u16>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ObsidianPrint {
    pub wrap_code: bool,
    pub paginate_tables: bool,
}

impl ObsidianIcons {
    fn validate(&self, file: &Path) -> Result<(), String> {
        if self.size_px.is_some_and(|size| !(12..=24).contains(&size)) {
            return Err(format!(
                "{}: icons.size_px: 12~24px 범위여야 합니다",
                file.display()
            ));
        }
        if self
            .stroke_width
            .is_some_and(|width| !(1.0..=3.0).contains(&width))
        {
            return Err(format!(
                "{}: icons.stroke_width: 1~3 범위의 유한한 값이어야 합니다",
                file.display()
            ));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObsidianFile {
    #[serde(default)]
    colors: Mapping,
    prose: Option<ObsidianLineHeight>,
    prose_detail: Option<ObsidianProseDetail>,
    code: Option<ObsidianCode>,
    layout: Option<DocumentLayout>,
    paragraph: Option<DocumentParagraph>,
    headings: Option<DocumentHeadings>,
    links: Option<ObsidianLinks>,
    blockquote: Option<ObsidianBlockquote>,
    emphasis: Option<DocumentEmphasis>,
    footnotes: Option<DocumentFootnotes>,
    horizontal_rule: Option<DocumentHorizontalRule>,
    table: Option<DocumentTable>,
    lists: Option<DocumentLists>,
    task_lists: Option<DocumentTaskLists>,
    images: Option<DocumentImages>,
    math: Option<DocumentMath>,
    callouts: Option<DocumentAlerts>,
    live_preview: Option<ObsidianLivePreview>,
    source_mode: Option<ObsidianSourceMode>,
    internal_links: Option<ObsidianInternalLinks>,
    tags_and_properties: Option<ObsidianTagsAndProperties>,
    navigation_sidebar: Option<ObsidianNavigationSidebar>,
    workspace: Option<ObsidianWorkspace>,
    dialogs: Option<ObsidianDialogs>,
    search: Option<ObsidianSearch>,
    graph: Option<ObsidianGraph>,
    canvas: Option<ObsidianCanvas>,
    bases: Option<ObsidianBases>,
    mermaid: Option<ObsidianMermaid>,
    icons: Option<ObsidianIcons>,
    mobile: Option<ObsidianMobile>,
    print: Option<ObsidianPrint>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HelixStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub underline: Option<HelixUnderline>,
    pub modifiers: Option<Vec<HelixModifier>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HelixUnderline {
    pub color: Option<String>,
    pub style: Option<HelixUnderlineStyle>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HelixUnderlineStyle {
    Line,
    Curl,
    Dashed,
    Dotted,
    DoubleLine,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HelixModifier {
    Bold,
    Dim,
    Italic,
    SlowBlink,
    RapidBlink,
    Reversed,
    Hidden,
    CrossedOut,
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct HelixFile {
    colors: Mapping,
    styles: BTreeMap<String, HelixStyle>,
    config: toml::Table,
    languages: toml::Table,
}

impl HelixFile {
    fn validate(&self, file: &Path, theme: &Theme) -> Result<(), String> {
        for (scope, style) in &self.styles {
            let key = format!("styles.{scope}");
            if !crate::helix::is_supported_scope(scope) {
                return Err(format!(
                    "{}: 지원하지 않는 Helix 스코프 '{key}'",
                    file.display()
                ));
            }
            if *style == HelixStyle::default() {
                return Err(format!(
                    "{}: {key}: 하나 이상의 속성을 지정하세요",
                    file.display()
                ));
            }
            if style.underline.as_ref() == Some(&HelixUnderline::default()) {
                return Err(format!(
                    "{}: {key}.underline: color 또는 style을 지정하세요",
                    file.display()
                ));
            }
            for (field, role) in [
                ("fg", style.fg.as_deref()),
                ("bg", style.bg.as_deref()),
                (
                    "underline.color",
                    style.underline.as_ref().and_then(|u| u.color.as_deref()),
                ),
            ] {
                if let Some(role) = role.filter(|&role| role != "default") {
                    validate_role(file, &format!("{key}.{field}"), role, theme)?;
                }
            }
        }
        Ok(())
    }
}

const VIM_HIGHLIGHT_GROUPS: &[&str] = &[
    "ColorColumn",
    "Conceal",
    "lCursor",
    "CursorIM",
    "CursorColumn",
    "EndOfBuffer",
    "VertSplit",
    "Folded",
    "FoldColumn",
    "LineNrAbove",
    "LineNrBelow",
    "NonText",
    "SpecialKey",
    "Terminal",
    "VisualNOS",
    "Directory",
    "QuickFixLine",
    "TabLine",
    "TabLineFill",
    "TabLineSel",
    "WildMenu",
    "MessageWindow",
    "ModeMsg",
    "MoreMsg",
    "PopupNotification",
    "Question",
    "Title",
    "SpellBad",
    "SpellCap",
    "SpellLocal",
    "SpellRare",
    "User1",
    "User2",
    "User3",
    "User4",
    "User5",
    "User6",
    "User7",
    "User8",
    "User9",
];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VimHighlight {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub sp: Option<String>,
    pub styles: Option<Vec<VimHighlightStyle>>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VimHighlightStyle {
    Bold,
    Italic,
    Underline,
    Undercurl,
    Reverse,
    Strikethrough,
}

impl VimHighlightStyle {
    pub fn name(self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::Underline => "underline",
            Self::Undercurl => "undercurl",
            Self::Reverse => "reverse",
            Self::Strikethrough => "strikethrough",
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct VimFile {
    colors: Mapping,
    highlights: BTreeMap<String, VimHighlight>,
}

impl VimFile {
    fn validate(&self, file: &Path, theme: &Theme) -> Result<(), String> {
        for (group, highlight) in &self.highlights {
            let key = format!("highlights.{group}");
            if !VIM_HIGHLIGHT_GROUPS.contains(&group.as_str()) {
                return Err(format!("{}: 지원하지 않는 키 '{key}'", file.display()));
            }
            let colors = [
                ("fg", &highlight.fg),
                ("bg", &highlight.bg),
                ("sp", &highlight.sp),
            ];
            if colors.iter().all(|(_, role)| role.is_none()) && highlight.styles.is_none() {
                return Err(format!(
                    "{}: {key}: 하나 이상의 속성을 지정하세요",
                    file.display()
                ));
            }
            for (field, role) in colors {
                if let Some(role) = role.as_deref().filter(|role| *role != "none") {
                    validate_role(file, &format!("{key}.{field}"), role, theme)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TyporaFile {
    #[serde(default)]
    colors: Mapping,
    layout: Option<DocumentLayout>,
    paragraph: Option<DocumentParagraph>,
    headings: Option<DocumentHeadings>,
    emphasis: Option<DocumentEmphasis>,
    footnotes: Option<DocumentFootnotes>,
    horizontal_rule: Option<DocumentHorizontalRule>,
    table: Option<DocumentTable>,
    lists: Option<DocumentLists>,
    task_lists: Option<DocumentTaskLists>,
    images: Option<DocumentImages>,
    math: Option<DocumentMath>,
    alerts: Option<DocumentAlerts>,
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
                        alerts.validate(&file, &theme, "alerts")?;
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
                } else if id == "obsidian" {
                    let config = read::<ObsidianFile>(&file)?;
                    if let Some(callouts) = &config.callouts {
                        callouts.validate(&file, &theme, "callouts")?;
                    }
                    if let Some(role) = config
                        .task_lists
                        .as_ref()
                        .and_then(|tasks| tasks.checkbox_color.as_deref())
                    {
                        validate_role(&file, "task_lists.checkbox_color", role, &theme)?;
                    }
                    if let Some(blockquote) = &config.blockquote {
                        blockquote.validate(&file, &theme)?;
                    }
                    if let Some(layout) = &config.layout {
                        layout.validate(&file)?;
                    }
                    if let Some(headings) = &config.headings {
                        headings.validate(&file)?;
                    }
                    if config
                        .prose_detail
                        .as_ref()
                        .and_then(|detail| detail.letter_spacing_em)
                        .is_some_and(|spacing| !(-0.5..=0.5).contains(&spacing))
                    {
                        return Err(format!(
                            "{}: prose_detail.letter_spacing_em: -0.5~0.5 범위의 유한한 값이어야 합니다",
                            file.display()
                        ));
                    }
                    if let Some(icons) = &config.icons {
                        icons.validate(&file)?;
                    }
                    theme.layers.obsidian_prose = config.prose;
                    theme.layers.obsidian_prose_detail = config.prose_detail;
                    theme.layers.obsidian_code = config.code;
                    theme.layers.obsidian_layout = config.layout;
                    theme.layers.obsidian_paragraph = config.paragraph;
                    theme.layers.obsidian_headings = config.headings;
                    theme.layers.obsidian_links = config.links;
                    theme.layers.obsidian_blockquote = config.blockquote;
                    theme.layers.obsidian_emphasis = config.emphasis;
                    theme.layers.obsidian_footnotes = config.footnotes;
                    theme.layers.obsidian_horizontal_rule = config.horizontal_rule;
                    theme.layers.obsidian_table = config.table;
                    theme.layers.obsidian_lists = config.lists;
                    theme.layers.obsidian_task_lists = config.task_lists;
                    theme.layers.obsidian_images = config.images;
                    theme.layers.obsidian_math = config.math;
                    theme.layers.obsidian_callouts = config.callouts;
                    theme.layers.obsidian_live_preview = config.live_preview;
                    theme.layers.obsidian_source_mode = config.source_mode;
                    theme.layers.obsidian_internal_links = config.internal_links;
                    theme.layers.obsidian_tags_and_properties = config.tags_and_properties;
                    theme.layers.obsidian_navigation_sidebar = config.navigation_sidebar;
                    theme.layers.obsidian_workspace = config.workspace;
                    theme.layers.obsidian_dialogs = config.dialogs;
                    theme.layers.obsidian_search = config.search;
                    theme.layers.obsidian_graph = config.graph;
                    theme.layers.obsidian_canvas = config.canvas;
                    theme.layers.obsidian_bases = config.bases;
                    theme.layers.obsidian_mermaid = config.mermaid;
                    theme.layers.obsidian_icons = config.icons;
                    theme.layers.obsidian_mobile = config.mobile;
                    theme.layers.obsidian_print = config.print;
                    config.colors
                } else if id == "helix" {
                    let config = read::<HelixFile>(&file)?;
                    config.validate(&file, &theme)?;
                    theme.layers.helix_styles = config.styles;
                    theme.layers.helix_config = config.config;
                    theme.layers.helix_languages = config.languages;
                    config.colors
                } else if id == "vim" {
                    let config = read::<VimFile>(&file)?;
                    config.validate(&file, &theme)?;
                    theme.layers.vim_highlights = config.highlights;
                    config.colors
                } else {
                    read::<EditorFile>(&file)?.colors
                };
                (colors, *keys)
            };
            for (key, role) in &map {
                let workbench = !language && id == "vscode";
                if !(allowed.contains(&key.as_str())
                    || workbench && crate::vscode::is_workbench_color(key))
                {
                    return Err(format!("{}: 지원하지 않는 키 '{key}'", file.display()));
                }
                let role = if workbench {
                    crate::vscode::workbench_color_value(role)
                        .map_err(|err| format!("{}: {key}: {err}", file.display()))?
                        .0
                } else {
                    role
                };
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
    fn obsidian_prose_line_height_is_scoped_and_uses_common_value() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        let source = include_str!("../tests/fixtures/lbrx.toml");
        let config = editors.join("obsidian.toml");
        let render = || crate::obsidian::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        fs::write(&path, source).unwrap();
        fs::write(&config, "[colors]\n\"--text-muted\" = \"muted\"\n").unwrap();
        let baseline = render();
        fs::write(&config, "[prose]\nuse_shared_line_height = false\n").unwrap();
        assert_eq!(render()[1], baseline[1]);

        fs::write(
            &config,
            include_str!("../tests/fixtures/editors/obsidian.toml"),
        )
        .unwrap();
        let output = render();
        assert!(output[1].1.contains(".markdown-rendered p,"));
        assert!(output[1].1.contains(
            r#".markdown-source-view.is-live-preview .cm-line:not([class*="HyperMD-"]) {"#
        ));
        assert!(output[1].1.contains("line-height: 1.65;"));
        assert!(!output[1].1.contains("--font-text-size"));

        fs::write(
            &path,
            source.replacen("line_height = 1.65", "line_height = 1.8", 1),
        )
        .unwrap();
        assert!(render()[1].1.contains("line-height: 1.8;"));
    }

    #[test]
    fn obsidian_number_and_error_use_common_live_colors_and_rendered_language_overrides() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let languages = tmp.path().join("languages");
        fs::create_dir(&languages).unwrap();
        fs::write(
            languages.join("common.toml"),
            "[tokens]\nnumber = 'function'\nerror = 'error'",
        )
        .unwrap();
        fs::write(
            languages.join("python.toml"),
            "[tokens]\nnumber = 'keyword'\nerror = 'comment'",
        )
        .unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [_, (_, css), _] = crate::obsidian::render(&theme);
        for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let rendered = format!(
                ".theme-{class} :is(.markdown-reading-view, .markdown-source-view.is-live-preview) .markdown-rendered pre"
            );
            for (token, common, python) in [
                ("number", "function", "keyword"),
                ("error", "error", "comment"),
            ] {
                let base_rule = format!(
                    "{rendered} code .token.{token} {{ color: {};",
                    mode.roles[common]
                );
                let override_rule = format!(
                    "{rendered} code.language-python .token.{token} {{ color: {};",
                    mode.roles[python]
                );
                assert!(css.find(&base_rule).unwrap() < css.find(&override_rule).unwrap());
                assert!(css.contains(&format!(
                    ".theme-{class} .markdown-source-view.is-live-preview .HyperMD-codeblock:not(.HyperMD-codeblock-begin, .HyperMD-codeblock-end) span.cm-{token}:not(.cm-formatting) {{ color: {};",
                    mode.roles[common]
                )));
            }
        }
        assert!(!css.contains("--code-value:"));
    }

    #[test]
    fn obsidian_code_border_and_scroll_only_change_reading_view_blocks() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let block = ".markdown-reading-view .markdown-rendered pre:has(> code)";
        let border = format!(
            "{block} {{ --code-border-width: 1px; --code-border-color: var(--text-muted); }}\n"
        );
        let scroll = format!(
            "{block} {{ --code-white-space: pre; overflow-x: auto; }}\n{block} > code {{ white-space: pre; }}\n"
        );
        let enabled = render(stock);
        assert!(enabled.contains(&border) && enabled.contains(&scroll));
        let native_wrap = stock.replacen(
            "reading_horizontal_scroll = true",
            "reading_horizontal_scroll = false",
            1,
        );
        assert_eq!(render(&native_wrap), enabled.replacen(&scroll, "", 1));
        let native_border = stock.replacen("reading_border_width_px = 1", "", 1);
        assert_eq!(render(&native_border), enabled.replacen(&border, "", 1));
    }

    #[test]
    fn obsidian_code_line_height_uses_shared_mono_value_without_overriding_code_size() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        let source = include_str!("../tests/fixtures/lbrx.toml");
        let config = editors.join("obsidian.toml");
        let css = || {
            crate::obsidian::render(&resolve::resolve(&load(&path).unwrap()).unwrap())[1]
                .1
                .clone()
        };
        let prose_only = "[prose]\nuse_shared_line_height = true\n";
        fs::write(&path, source).unwrap();
        fs::write(&config, prose_only).unwrap();
        let baseline = css();
        fs::write(
            &config,
            format!("{prose_only}\n[code]\nuse_shared_line_height = false\n"),
        )
        .unwrap();
        assert_eq!(css(), baseline);

        let selectors = concat!(
            "\n.markdown-rendered pre,\n",
            ".markdown-rendered pre > code,\n",
            ".markdown-source-view.is-live-preview .HyperMD-codeblock {\n"
        );
        fs::write(
            &config,
            format!("{prose_only}\n[code]\nuse_shared_line_height = true\n"),
        )
        .unwrap();
        assert_eq!(
            css(),
            format!("{baseline}{selectors}  line-height: 1.45;\n}}\n")
        );
        fs::write(
            &path,
            source.replacen("line_height = 1.45", "line_height = 1.6", 1),
        )
        .unwrap();
        assert_eq!(
            css(),
            format!("{baseline}{selectors}  line-height: 1.6;\n}}\n")
        );
    }

    #[test]
    fn obsidian_layout_uses_native_width_gate_and_validates_padding() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let css = || {
            crate::obsidian::render(&resolve::resolve(&load(&path).unwrap()).unwrap())[1]
                .1
                .clone()
        };
        assert!(!css().contains("--file-line-width:"));
        let layout = "[layout]\nmax_width_px = 920\npadding_inline_min_px = 12\npadding_inline_max_px = 36\n";
        fs::write(&config, layout).unwrap();
        let output = css();
        assert_eq!(output.matches("--file-line-width:").count(), 1);
        assert!(output.contains(concat!(
            ".markdown-reading-view > .markdown-preview-view.is-readable-line-width,\n",
            ".markdown-source-view.is-live-preview.is-readable-line-width {\n",
            "  --file-line-width: calc(920px - var(--lbrx-file-padding-inline) * 2);"
        )));
        assert!(output.contains("--lbrx-file-padding-inline: clamp(12px, 4vw, 36px);"));
        assert!(output.contains(".markdown-source-view.is-live-preview > .cm-editor > .cm-scroller {\n  padding-inline: var(--lbrx-file-padding-inline);"));
        for invalid in [
            layout.replace("12", "40"),
            layout.replace("920", "72"),
            layout.replace("920", "0"),
        ] {
            fs::write(&config, invalid).unwrap();
            let err = load(&path)
                .err()
                .expect("invalid Obsidian layout was accepted");
            assert!(
                err.contains("obsidian.toml") && err.contains("layout"),
                "{err}"
            );
        }
    }

    #[test]
    fn obsidian_paragraph_gap_only_adds_rules_between_reading_view_paragraphs() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = || {
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let baseline = render();
        for gap in [12, 0] {
            fs::write(&config, format!("[paragraph]\ngap_px = {gap}\n")).unwrap();
            let output = render();
            let added = output.strip_prefix(&baseline).unwrap();
            assert!(added.contains(".el-p:has(+ .el-p) > p {\n  margin-block-end: 0;"));
            assert!(added.contains(&format!(
                ".el-p + .el-p > p {{\n  margin-block-start: {gap}px;"
            )));
            assert_eq!(added.matches(".markdown-reading-view").count(), 2);
            assert!(!added.contains(".markdown-source-view"));
        }
    }

    #[test]
    fn obsidian_paragraph_alignment_is_independent_of_gap_and_scoped_to_reading_view() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = || {
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let baseline = render();
        let selector =
            ".markdown-reading-view > .markdown-preview-view > .markdown-preview-sizer > .el-p > p";
        for alignment in ["start", "justify"] {
            fs::write(&config, format!("[paragraph]\nalignment = '{alignment}'\n")).unwrap();
            assert_eq!(
                render(),
                format!("{baseline}\n{selector} {{\n  text-align: {alignment};\n}}\n")
            );
        }
        fs::write(&config, "[paragraph]\ngap_px = 12\n").unwrap();
        let gap_only = render();
        assert!(gap_only.contains("margin-block-start: 12px;"));
        assert!(!gap_only.contains("text-align:"));
        fs::write(&config, "[paragraph]\nalignment = 'left'\n").unwrap();
        let err = load(&path)
            .err()
            .expect("unsupported alignment was accepted");
        assert!(
            err.contains("obsidian.toml") && err.contains("left"),
            "{err}"
        );
    }

    #[test]
    fn obsidian_print_options_only_affect_the_export_document() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let full = render(stock);
        let start = full.find("@media print {\n").unwrap();
        let end = start + full[start..].find("\n}\n").unwrap() + 3;
        let block = &full[start..end];
        let scope = "body > .print > .markdown-preview-view.markdown-rendered";
        assert!(
            block
                .lines()
                .skip(1)
                .take_while(|line| *line != "}")
                .all(|line| { line.trim_start().starts_with(scope) })
        );
        assert!(block.contains("white-space: pre-wrap; overflow-wrap: anywhere;"));
        assert!(block.contains("> thead { display: table-header-group;"));
        assert!(
            block.contains("min-inline-size: 0; white-space: normal; overflow-wrap: anywhere;")
        );
        let without_code = stock.replacen("\nwrap_code = true", "\nwrap_code = false", 1);
        let tables_only = render(&without_code);
        assert!(tables_only.contains("display: table-header-group;"));
        assert!(!tables_only.contains(&format!("{scope} pre > code")));
        let neither =
            without_code.replacen("\npaginate_tables = true", "\npaginate_tables = false", 1);
        assert_eq!(render(&neither), full.replacen(block, "", 1));
    }

    #[test]
    fn obsidian_mobile_caps_phone_padding_without_replacing_native_layout() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let css = render(stock);
        let configured_padding = css
            .split_once("--lbrx-file-padding-inline: clamp(")
            .unwrap()
            .1
            .split_once(';')
            .unwrap()
            .0;
        let start = css
            .find("@media screen {\n  .is-mobile.is-phone .markdown-reading-view")
            .unwrap();
        let end = start + css[start..].find("\n}\n").unwrap() + 3;
        let phone = &css[start..end];
        assert!(phone.contains(".is-mobile.is-phone .markdown-source-view.is-live-preview"));
        assert!(phone.contains(&format!("min(clamp({configured_padding}, 12px)")));
        assert!(phone.contains("var(--safe-area-inline-start, 0px)"));
        assert!(phone.contains("var(--safe-area-inline-end, 0px)"));
        assert!(!phone.contains("padding-block") && !phone.contains("keyboard-height"));
        let uncapped = render(&stock.replacen(
            "\nphone_padding_max_px = 12",
            "\n# phone_padding_max_px = 12",
            1,
        ));
        assert_eq!(uncapped, css.replacen(phone, "", 1));
        let without_layout = render("[mobile]\nphone_padding_max_px = 16");
        assert!(without_layout.contains("min(var(--file-margins-x, 24px), 16px)"));

        assert!(css.contains(".is-mobile:not(.is-floating-nav) .mobile-toolbar,"));
        assert!(css.contains(".is-mobile .workspace-drawer { --mobile-sidebar-background:"));
        for (setting, selector) in [
            (
                "style_toolbar",
                ".is-mobile:not(.is-floating-nav) .mobile-toolbar",
            ),
            ("style_drawers", ".is-mobile .workspace-drawer {"),
        ] {
            let start = css.find(selector).unwrap();
            let end = start + css[start..].find("}\n").unwrap() + 2;
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            assert_eq!(disabled, css.replacen(&css[start..end], "", 1));
        }
    }

    #[test]
    fn obsidian_icons_scope_state_colors_and_keep_native_dimensions_by_default() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let base = render(stock);
        for (setting, scope) in [
            ("style_ribbon", ".workspace-ribbon .side-dock-ribbon-action"),
            ("style_note_header", "> .view-header .view-action"),
        ] {
            let matches_scope = |line: &str| line.contains(&format!("{scope}.clickable-icon"));
            for mode in ["light", "dark"] {
                let prefix = format!(".theme-{mode}:not(.is-mobile)");
                assert!(base.lines().any(|line| {
                    line.starts_with(&prefix)
                        && matches_scope(line)
                        && line.contains(":focus-visible")
                        && line.contains("color: var(--icon-color-focused)")
                }));
            }
            assert!(
                !base.lines().filter(|line| matches_scope(line)).any(|line| {
                    line.contains("--icon-size:") || line.contains("--icon-stroke:")
                })
            );
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            assert_eq!(
                disabled.lines().collect::<Vec<_>>(),
                base.lines()
                    .filter(|line| !matches_scope(line))
                    .collect::<Vec<_>>()
            );
        }
        let sized = render("[icons]\nstyle_ribbon = true\nsize_px = 18\nstroke_width = 1.75");
        assert_eq!(
            sized
                .lines()
                .filter(|line| line
                    .contains("> svg.svg-icon { --icon-size: 18px; --icon-stroke: 1.75; }"))
                .count(),
            2
        );
        for (key, value) in [("size_px", "25"), ("stroke_width", "nan")] {
            fs::write(&config, format!("[icons]\n{key} = {value}")).unwrap();
            let error = load(&path).err().expect("invalid icon dimension accepted");
            assert!(error.contains(&format!("icons.{key}")), "{error}");
        }
    }

    #[test]
    fn obsidian_mermaid_palette_and_screen_scroll_are_independent() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let base = render("");
        let palette = render("[mermaid]\nuse_theme_palette = true");
        let scroll = render("[mermaid]\nscroll_wide_diagrams = true");
        let both = render("[mermaid]\nuse_theme_palette = true\nscroll_wide_diagrams = true");
        let scope = ":is(.markdown-reading-view .markdown-rendered, .markdown-source-view.is-live-preview .cm-embed-block) .mermaid";
        let start = palette.find(scope).unwrap();
        let end = start + palette[start..].find("}\n").unwrap() + 2;
        let palette_rule = &palette[start..end];
        assert!(palette_rule.contains("--mermaid-text-color: var(--text-normal)"));
        assert!(!palette_rule.contains("fill:") && !palette_rule.contains("stroke:"));
        let scroll_rule = format!(
            "@media screen {{\n  {scope} {{ max-inline-size: 100%; overflow-x: auto; }}\n}}\n"
        );
        assert!(scroll.contains(&scroll_rule));
        assert_eq!(palette.replacen(palette_rule, "", 1), base);
        assert_eq!(scroll.replacen(&scroll_rule, "", 1), base);
        assert_eq!(both.replacen(palette_rule, "", 1), scroll);
    }

    #[test]
    fn obsidian_bases_options_only_change_their_own_views() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let stock_css = render(stock);
        for (setting, selector, expected) in [
            (
                "style_table",
                ".bases-view[data-view-type=\"table\"]",
                ".bases-table-container:focus .bases-table-selection",
            ),
            (
                "style_cards",
                ".bases-view .bases-cards-container",
                "--bases-cards-shadow-hover:",
            ),
        ] {
            let rules: Vec<_> = stock_css
                .split_inclusive("}\n")
                .filter(|rule| rule.contains(selector))
                .collect();
            for scheme in ["light", "dark"] {
                let scope = format!(".theme-{scheme} {selector}");
                assert!(
                    rules
                        .iter()
                        .any(|rule| { rule.starts_with(&scope) && rule.contains(expected) })
                );
            }
            let unchanged: String = stock_css
                .split_inclusive("}\n")
                .filter(|rule| !rule.contains(selector))
                .collect();
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            assert_eq!(disabled, unchanged);
        }
    }

    #[test]
    fn obsidian_canvas_preserves_explicit_colors_and_can_toggle_selection_separately() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let css = render(stock);
        for scheme in ["light", "dark"] {
            let scope = format!(".theme-{scheme} .canvas-wrapper .canvas");
            let rules: Vec<_> = css
                .lines()
                .filter(|line| line.starts_with(&scope))
                .collect();
            for rule in &rules {
                if rule.contains("--canvas-color:") {
                    let selectors = rule.split_once(" {").unwrap().0;
                    assert!(
                        selectors
                            .split(", ")
                            .all(|selector| selector.contains(":not(.is-themed)")),
                        "{rule}"
                    );
                }
            }
            assert!(rules.iter().any(|rule| {
                rule.contains(":not(.is-themed, .canvas-node-group) > .canvas-node-container")
                    && rule.contains("background-color: var(--background-secondary)")
            }));
            assert!(rules.iter().any(|rule| {
                rule.contains(":is(.is-selected, .is-focused) > .canvas-node-container")
                    && rule.contains("outline: 2px solid")
                    && !rule.contains("border-color:")
            }));
        }
        for (setting, declarations) in [
            (
                "style_default_colors",
                ["--canvas-color: var(--text-muted)", "background-color:", ""],
            ),
            (
                "highlight_selection",
                [
                    "outline: 2px solid",
                    "border-color:",
                    "g:not(.is-themed).is-focused",
                ],
            ),
        ] {
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            let remaining: Vec<_> = css
                .lines()
                .filter(|rule| {
                    !rule.contains(".canvas-wrapper .canvas")
                        || !declarations
                            .iter()
                            .any(|value| !value.is_empty() && rule.contains(value))
                })
                .collect();
            assert_eq!(disabled.lines().collect::<Vec<_>>(), remaining);
        }
    }

    #[test]
    fn obsidian_graph_palette_targets_shared_body_probes_and_is_optional() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let (theme, css) = render(stock);
        let mut without_graph = css.clone();
        for (scheme, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let probe = format!("body.theme-{scheme} > .graph-view");
            let start = css.find(&format!("{probe} {{")).unwrap();
            let last_rule = format!("{probe}:is(.color-fill-highlight, .color-line-highlight)");
            let last = css.find(&last_rule).unwrap();
            let end = last + css[last..].find("}\n").unwrap() + 2;
            let rules = &css[start..end];
            assert!(rules.contains("--graph-text: var(--text-normal);"));
            assert!(rules.contains(&format!("--graph-node-focused: {};", mode.roles["accent"])));
            assert!(css[last..end].contains(&format!("color: {};", mode.roles["accent"])));
            assert!(!rules.contains("color-fill-1") && !rules.contains("workspace-leaf"));
            without_graph = without_graph.replacen(rules, "", 1);
        }
        let (_, disabled) = render(&stock.replacen(
            "\nuse_theme_palette = true",
            "\nuse_theme_palette = false",
            1,
        ));
        assert_eq!(disabled, without_graph);
    }

    #[test]
    fn obsidian_search_distinguishes_native_matches_without_changing_selection() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let base = render("[search]");
        let note = render("[search]\nhighlight_note_matches = true");
        let vault = render("[search]\nhighlight_vault_matches = true");
        let both =
            render("[search]\nhighlight_note_matches = true\nhighlight_vault_matches = true");
        let note_rules = |line: &str| {
            line.contains(".search-highlight > div")
                || line.contains("span.obsidian-search-match-highlight")
        };
        let vault_rules =
            |line: &str| line.contains(".mod-global-search .search-result-file-matched-text");
        for scheme in ["light", "dark"] {
            let reading = format!(
                ".theme-{scheme} .markdown-reading-view .markdown-rendered .search-highlight > div"
            );
            assert!(note.lines().any(|line| {
                line.starts_with(&format!("{reading} {{")) && !line.contains("inset")
            }));
            assert!(note.lines().any(|line| {
                line.starts_with(&format!("{reading}.is-active {{"))
                    && line.contains("box-shadow: inset 0 -2px")
            }));
            assert!(note.lines().any(|line| {
                line.starts_with(&format!(
                    ".theme-{scheme} .markdown-source-view.mod-cm6:has(> .document-search-container) "
                )) && line.contains("span.obsidian-search-match-highlight {")
                    && !line.contains("inset")
            }));
            assert!(vault.lines().any(|line| {
                line.starts_with(&format!(".theme-{scheme} .search-result-container"))
                    && vault_rules(line)
            }));
        }
        assert_eq!(
            note.lines()
                .filter(|line| !note_rules(line))
                .collect::<Vec<_>>(),
            base.lines().collect::<Vec<_>>()
        );
        assert_eq!(
            vault
                .lines()
                .filter(|line| !vault_rules(line))
                .collect::<Vec<_>>(),
            base.lines().collect::<Vec<_>>()
        );
        assert_eq!(
            both.lines()
                .filter(|line| !vault_rules(line))
                .collect::<Vec<_>>(),
            note.lines().collect::<Vec<_>>()
        );
    }

    #[test]
    fn obsidian_dialogs_are_desktop_scoped_and_independently_configurable() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let enabled = render(stock);
        for (setting, scope, state) in [
            (
                "style_settings",
                ".modal.mod-settings",
                ".vertical-tab-nav-item.is-active",
            ),
            (
                "style_prompts",
                ".prompt:has(",
                ".suggestion-item.is-selected",
            ),
            (
                "style_menus",
                ".menu:has(",
                ".menu-item.selected:not(.is-label)",
            ),
        ] {
            for scheme in ["light", "dark"] {
                let prefix = format!(".theme-{scheme}:not(.is-mobile) {scope}");
                let rules: Vec<_> = enabled
                    .lines()
                    .filter(|line| line.starts_with(&prefix))
                    .collect();
                assert!(rules.iter().any(|line| {
                    line.contains(state)
                        && line.contains(r#" [aria-disabled="true"]"#)
                        && line.contains("linear-gradient(")
                }));
                assert!(rules.iter().any(|line| {
                    line.contains(":focus-visible") && line.contains("outline: 2px solid")
                }));
                assert!(
                    rules
                        .iter()
                        .any(|line| line.contains("color: var(--text-muted)"))
                );
            }
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            let remaining: Vec<_> = enabled
                .lines()
                .filter(|line| !line.contains(&format!(":not(.is-mobile) {scope}")))
                .collect();
            assert_eq!(disabled.lines().collect::<Vec<_>>(), remaining);
        }
    }

    #[test]
    fn obsidian_workspace_parts_can_be_disabled_independently() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let enabled = render(stock);
        let rules = [
            (
                "style_active_tabs",
                ".workspace .workspace-tab-header-container .workspace-tab-header.is-active",
                "linear-gradient(to top, var(--interactive-accent) 2px, transparent 2px)",
            ),
            (
                "shade_ribbon",
                ".workspace > .workspace-ribbon",
                "--ribbon-background-collapsed: var(--background-secondary);",
            ),
            (
                "shade_status_bar",
                ".app-container .status-bar",
                "--status-bar-background: var(--background-secondary);",
            ),
            (
                "style_split_dividers",
                ".workspace .workspace-leaf-resize-handle",
                "--divider-color: color-mix(in srgb, var(--text-muted) 24%, var(--background-secondary));",
            ),
        ];
        for (setting, selector, declaration) in rules {
            let start = enabled.find(selector).unwrap();
            let end = start + enabled[start..].find("}\n").unwrap() + 2;
            let rule = &enabled[start..end];
            assert!(rule.contains(declaration));
            if setting == "style_active_tabs" {
                assert!(rule.contains("--tab-font-weight: 700;"));
            }
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            assert_eq!(disabled, enabled.replacen(rule, "", 1));
        }
    }

    #[test]
    fn obsidian_navigation_sidebar_can_disable_each_pane_independently() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            crate::obsidian::render(&theme)[1].1.clone()
        };
        let enabled = render(stock);
        for (setting, pane, row) in [
            ("file_explorer", "file-explorer", ".tree-item-self"),
            ("outline", "outline", ".tree-item-self"),
            (
                "backlinks",
                "backlink",
                ":is(.search-result-file-title, .search-result-file-match)",
            ),
        ] {
            for scheme in ["light", "dark"] {
                let scope = format!(
                    ".theme-{scheme} :is(.mod-left-split, .mod-right-split) .workspace-leaf-content[data-type=\"{pane}\"] .view-content {row}"
                );
                assert!(enabled.contains(&format!("{scope}:hover:not(")));
                assert!(enabled.contains(&format!("{scope}:is(.is-active, .is-selected,")));
                assert!(enabled.contains(&format!("{scope}:focus-visible {{ outline: 2px solid ")));
            }
            let disabled = render(&stock.replacen(
                &format!("\n{setting} = true"),
                &format!("\n{setting} = false"),
                1,
            ));
            assert!(!disabled.contains(&format!("[data-type=\"{pane}\"]")));
            for other in ["file-explorer", "outline", "backlink"]
                .into_iter()
                .filter(|&other| other != pane)
            {
                assert!(disabled.contains(&format!("[data-type=\"{other}\"]")));
            }
        }
    }

    #[test]
    fn obsidian_tags_and_properties_are_independently_scoped() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let (theme, enabled) = render(stock);
        let tag_selector = "a.tag:not(.metadata-container *)";
        assert!(enabled.contains(tag_selector));
        assert!(enabled.contains(".cm-line:not(.HyperMD-codeblock, .HyperMD-frontmatter) .cm-hashtag:not(.cm-inline-code)"));
        let (_, tags_disabled) =
            render(&stock.replacen("style_inline_tags = true", "style_inline_tags = false", 1));
        assert!(!tags_disabled.contains("--tag-background:"));
        let (_, properties_disabled) =
            render(&stock.replacen("style_properties = true", "style_properties = false", 1));
        assert!(properties_disabled.contains(tag_selector));
        let mut expected = enabled;
        for (scheme, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let rule = format!(
                ".theme-{scheme} :is(.markdown-reading-view .markdown-preview-view, .markdown-source-view.is-live-preview) .metadata-container {{\n  --metadata-label-text-color: var(--text-muted);\n  --metadata-property-box-shadow-focus: 0 0 0 2px {};\n}}\n",
                mode.roles["focus"]
            );
            assert!(expected.contains(&rule));
            assert!(tags_disabled.contains(&rule));
            expected = expected.replacen(&rule, "", 1);
        }
        assert_eq!(properties_disabled, expected);
    }

    #[test]
    fn obsidian_internal_link_and_note_embed_styles_can_be_disabled_independently() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let css = render(stock);
        let unresolved_start =
            ".markdown-reading-view .markdown-rendered a.internal-link.is-unresolved,";
        let start = css.find(unresolved_start).unwrap();
        let end = start + css[start..].find("}\n").unwrap() + 2;
        let unresolved = &css[start..end];
        assert!(unresolved.contains(".is-live-preview :is(.is-unresolved.cm-hmd-internal-link,"));
        assert!(unresolved.contains("--link-unresolved-decoration-style: dotted;"));
        assert!(unresolved.contains("--link-unresolved-color: var(--text-muted);"));
        assert_eq!(
            render(&stock.replacen(
                "distinguish_unresolved = true",
                "distinguish_unresolved = false",
                1
            )),
            css.replacen(unresolved, "", 1)
        );

        let embed = ":is(.markdown-reading-view .markdown-rendered, .markdown-source-view.is-live-preview) .internal-embed.markdown-embed { --embed-border-start: 2px solid var(--interactive-accent); }\n";
        assert!(css.contains(embed));
        assert_eq!(
            render(&stock.replacen("note_embed_border_width_px = 2", "", 1)),
            css.replacen(embed, "", 1)
        );
        assert_eq!(
            render(&stock.replacen(
                "note_embed_border_width_px = 2",
                "note_embed_border_width_px = 0",
                1
            )),
            css.replacen(embed, &embed.replace("2px", "0px"), 1)
        );
    }

    #[test]
    fn obsidian_source_mode_mutes_only_markdown_markers() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let rule = concat!(
            ".markdown-source-view.mod-cm6:not(.is-live-preview) .cm-content ",
            ".cm-line:not(.HyperMD-codeblock, .HyperMD-frontmatter) span.cm-formatting {\n",
            "  color: var(--text-muted);\n",
            "}\n"
        );
        let enabled = render(stock);
        assert!(enabled.contains(rule));
        let disabled = render(&stock.replacen(
            "mute_markdown_markers = true",
            "mute_markdown_markers = false",
            1,
        ));
        assert_eq!(disabled, enabled.replacen(rule, "", 1));
        assert_eq!(
            disabled,
            render(&stock.replacen("mute_markdown_markers = true", "", 1))
        );
    }

    #[test]
    fn obsidian_live_preview_settings_are_independent_and_preserve_other_views() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let (theme, enabled) = render(stock);
        let active = ".markdown-source-view.is-live-preview .cm-editor.cm-focused .cm-line:is(.cm-active, .cm-activeLine) {";
        let (_, without_highlight) = render(&stock.replacen(
            "highlight_active_line = true",
            "highlight_active_line = false",
            1,
        ));
        let (_, without_caret) =
            render(&stock.replacen("use_focus_caret = true", "use_focus_caret = false", 1));
        let start = enabled.find(active).unwrap();
        let end = start + enabled[start..].find("}\n").unwrap() + 2;
        assert_eq!(
            without_highlight,
            format!("{}{}", &enabled[..start], &enabled[end..])
        );
        let mut expected = enabled;
        for (scheme, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let editor =
                format!(".theme-{scheme} .markdown-source-view.is-live-preview .cm-editor");
            let color = &mode.roles["focus"];
            let rule = format!(
                "{editor} .cm-content {{ caret-color: {color}; }}\n{editor} .cm-cursor {{ --caret-color: {color}; }}\n"
            );
            assert!(expected.contains(&rule));
            expected = expected.replacen(&rule, "", 1);
        }
        assert_eq!(without_caret, expected);
    }

    #[test]
    fn obsidian_callouts_scope_aliases_and_validate_custom_title_contrast() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let (theme, css) = render(include_str!("../tests/fixtures/editors/obsidian.toml"));
        for (scheme, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let scope = format!(
                ".theme-{scheme} :is(.markdown-reading-view .markdown-rendered, .markdown-source-view.is-live-preview)"
            );
            for (types, role) in [
                (r#"[data-callout="tip"], [data-callout="hint"]"#, "success"),
                (
                    r#"[data-callout="warning"], [data-callout="attention"]"#,
                    "warning",
                ),
                (r#"[data-callout="caution"]"#, "error"),
            ] {
                let callout = format!("{scope} .callout:is({types})");
                let color = &mode.roles[role];
                assert!(css.contains(&format!(
                    "{callout} {{ border-inline-start: 3px solid {color}; background-color: var(--background-secondary); }}"
                )));
                assert!(css.contains(&format!(
                    "{callout} > .callout-title,\n{callout} > .callout-title > .callout-icon .svg-icon {{ color: {color}; }}"
                )));
            }
        }
        assert!(!css.contains(r#"data-callout="bug""#));
        let (theme, custom) =
            render("[callouts]\nshaded_background = false\n[callouts.colors]\nnote = 'accent'");
        assert!(custom.contains(&format!(
            r#"[data-callout="note"]) {{ border-inline-start: 3px solid {}; }}"#,
            theme.light.roles["accent"]
        )));
        assert!(!render("").1.contains("[data-callout="));

        fs::write(
            &config,
            "[callouts]\nshaded_background = true\n[callouts.colors]\nnote = 'surface'",
        )
        .unwrap();
        let Err(error) = resolve::check(&load(&path).unwrap()) else {
            panic!("a callout title matching its background must fail contrast validation");
        };
        assert!(error.contains("callouts.colors.note") && error.contains("최소 4.5:1"));
    }

    #[test]
    fn obsidian_math_styles_only_reading_view_blocks_and_scrolls_on_screen() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let (before, remainder) = stock.split_once("[math]").unwrap();
        let (_, after) = remainder.split_once("[images]").unwrap();
        let disabled = format!("{before}[images]{after}");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let block = ".markdown-reading-view .markdown-rendered .math-block";
        let spacing = format!(
            "{block} {{ box-sizing: border-box; max-inline-size: 100%; padding-block: 12px; padding-inline: 16px; }}\n"
        );
        let shade = format!("{block} {{ background-color: var(--background-secondary); }}\n");
        let scrolling = format!(
            "@media screen {{\n  {block} > mjx-container {{ max-inline-size: 100%; overflow-x: auto; }}\n}}\n"
        );
        let enabled = render(stock);
        let rule = format!("{spacing}{shade}{scrolling}");
        assert!(enabled.contains(&rule));
        assert_eq!(render(&disabled), enabled.replacen(&rule, "", 1));

        let customized = render(
            "[math]\npadding_block_px = 8\npadding_inline_px = 12\nshaded_background = false",
        );
        assert!(customized.contains(&spacing.replace(
            "padding-block: 12px; padding-inline: 16px;",
            "padding-block: 8px; padding-inline: 12px;"
        )));
        assert!(customized.contains(&scrolling));
        assert!(!customized.contains(&shade));
    }

    #[test]
    fn obsidian_images_preserve_width_and_only_style_explicit_captions() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let stock = render(include_str!("../tests/fixtures/editors/obsidian.toml"));
        let scope = ".markdown-reading-view .markdown-rendered";
        assert!(stock.contains(&format!(
            "{scope} .el-p > p > .image-embed:only-child > img,"
        )));
        assert!(stock.contains(
            "img { display: block; max-inline-size: 100%; block-size: auto; margin-inline: auto; }"
        ));
        let caption = format!(
            "{scope} figure:has(> img, > :is(a, picture, .image-embed) > img) > figcaption"
        );
        assert!(stock.contains(&format!(
            "{caption} {{ margin-block-start: 8px; font-size: 90%; color: var(--text-muted); text-align: center; }}"
        )));
        assert!(!stock.contains("attr(alt)"));
        let custom =
            render("[images]\nalignment = 'end'\ncaption_size_percent = 100\ncaption_gap_px = 4");
        assert!(custom.contains("margin-inline: auto 0;"));
        assert!(custom.contains(&format!(
            "{caption} {{ margin-block-start: 4px; font-size: 100%;"
        )));
        assert!(!render("").contains(&caption));
    }

    #[test]
    fn obsidian_task_lists_keep_subtasks_out_of_completed_decoration() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let (theme, css) = render("[task_lists]\nstrike_completed = true");
        let task = ".markdown-reading-view .markdown-rendered li.task-list-item[data-task=\"x\" i]";
        for (scheme, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            assert!(css.contains(&format!(
                ".theme-{scheme} {task} > p > input.task-list-item-checkbox:checked {{ --checkbox-color: {}; }}",
                mode.roles["accent"]
            )));
        }
        assert!(css.contains(&format!("{task} {{ text-decoration: none;")));
        assert!(css.contains(&format!("{task} > p,\n{task}:not(:has(> p, > :is(ul, ol,")));
        assert!(css.contains(&format!("{task}:not(:has(> p)) > :is(ul, ol,")));
        assert!(css.contains("text-decoration-line: line-through;"));

        let (theme, css) =
            render("[task_lists]\nstrike_completed = false\ncheckbox_color = 'warning'");
        assert!(!css.contains("text-decoration-line: line-through;"));
        assert!(css.contains(&format!(
            "--checkbox-color: {};",
            theme.dark.roles["warning"]
        )));
        let (_, disabled) = render("");
        assert!(!disabled.contains("input.task-list-item-checkbox"));
        assert!(!disabled.contains(task));
    }

    #[test]
    fn obsidian_lists_style_only_ordinary_items_and_support_native_markers() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let scope = ".markdown-reading-view .markdown-rendered :is(.el-ul, .el-ol)";
        let item = "li:not(:where(.task-list-item, .task-list-item *, .footnotes *))";
        let stock = render(include_str!("../tests/fixtures/editors/obsidian.toml"));
        assert!(stock.contains(&format!(
            "{scope} :is(ol, ul) > {item} {{ margin-inline-start: 24px; }}"
        )));
        assert!(stock.contains(&format!(
            "{scope} :is(ol, ul) > {item} + {item} {{ margin-block-start: 4px; }}"
        )));
        for (ancestors, radius, border) in [
            ("", "50%", "none"),
            (":is(ol, ul) ", "50%", "1px solid currentColor"),
            (":is(ol, ul) :is(ol, ul) ", "0", "none"),
        ] {
            assert!(stock.contains(&format!(
                "{scope} {ancestors}ul > {item} > .list-bullet {{ --list-bullet-radius: {radius}; --list-bullet-border: {border}; }}"
            )));
        }
        assert!(stock.contains(
            ":not(:where(.is-collapsed, .is-collapsed *)) > .list-bullet::after { background-color: transparent; }"
        ));
        let spacing = "[lists]\nindent_px = 20\nitem_gap_px = 0";
        let spacing_only = render(spacing);
        assert!(spacing_only.contains("margin-inline-start: 20px;"));
        assert!(!spacing_only.contains(".list-bullet"));
        let customized = render(&format!("{spacing}\n[lists.markers]\nlevel_1 = 'square'"));
        assert!(customized.contains(&format!(
            "{scope} ul > {item} > .list-bullet {{ --list-bullet-radius: 0;"
        )));
        assert!(!render("").contains(scope));
    }

    #[test]
    fn obsidian_table_styles_are_opt_in_and_scoped_to_rendered_markdown_tables() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let (without_table, _) = stock.split_once("[table]").unwrap();
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let wrapper = ".markdown-reading-view .markdown-rendered .el-table";
        let expected = format!(
            "{wrapper} > table > :is(thead, tbody) > tr > :is(th, td) {{\n  padding: 8px 12px;\n  border: 0;\n  border-block-end: 1px solid var(--text-muted);\n}}\n\
             {wrapper} > table > thead > tr > th {{ background-color: var(--background-secondary); font-weight: 700; }}\n\
             @media screen {{\n  {wrapper}:has(> table) {{ max-width: 100%; overflow-x: auto; }}\n}}\n"
        );
        let enabled = render(stock);
        assert!(enabled.contains(&expected));
        assert_eq!(render(without_table), enabled.replacen(&expected, "", 1));
        let customized = render(
            "[table]\ncell_padding_block_px = 4\ncell_padding_inline_px = 8\nrow_rule_px = 2",
        );
        assert!(
            customized.contains(
                &expected
                    .replace("8px 12px", "4px 8px")
                    .replace("1px solid", "2px solid")
            )
        );
    }

    #[test]
    fn obsidian_footnotes_and_rules_are_reading_only_and_independent() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = dir.join("obsidian.toml");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let (theme, css) = render(stock);
        for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let scope = format!(".theme-{class} .markdown-reading-view .markdown-rendered");
            assert!(css.contains(&format!(
                "{scope} a.footnote-ref {{ color: {}; }}",
                mode.roles["function"]
            )));
            assert!(css.contains(&format!(
                "{scope} .footnotes {{ --footnote-size: 90%; color: var(--text-muted); }}"
            )));
            assert!(css.contains(&format!(
                "{scope} hr:not(.footnotes *) {{ --hr-color: var(--text-muted); --hr-thickness: 1px; margin-block: 24px; }}"
            )));
        }
        assert!(!css.contains("footnote-backref"));
        let (_, disabled) = render(stock.split_once("[footnotes]").unwrap().0);
        assert!(!disabled.contains("footnote-ref") && !disabled.contains("--hr-thickness:"));
        let (theme, footnotes_only) = render(
            "[colors]\n'--lbrx-footnote-reference' = 'link'\n[footnotes]\nbody_size_percent = 100",
        );
        assert!(footnotes_only.contains(&format!(
            "a.footnote-ref {{ color: {}; }}",
            theme.light.roles["link"]
        )));
        assert!(footnotes_only.contains("--footnote-size: 100%;"));
        assert!(!footnotes_only.contains("--hr-thickness:"));
        let (_, rule_only) = render("[horizontal_rule]\nthickness_px = 2\nmargin_block_px = 16");
        assert!(rule_only.contains("hr:not(.footnotes *)"));
        assert!(rule_only.contains("--hr-thickness: 2px; margin-block: 16px;"));
        assert!(!rule_only.contains("footnote-ref"));
    }

    #[test]
    fn obsidian_emphasis_is_scoped_and_validates_highlight_contrast() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        fs::write(&config, stock).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [_, (_, css), _] = crate::obsidian::render(&theme);
        for (class, strong, italic, strike, highlight) in [
            ("light", "#A84B16", "#245E8A", "#6D5A4B", "#FFE8CB"),
            ("dark", "#FFC695", "#B4D9F5", "#C0AC9B", "#503927"),
        ] {
            let rules: Vec<_> = css
                .lines()
                .filter(|line| line.starts_with(&format!(".theme-{class} ")))
                .filter(|line| line.contains("cm-formatting-"))
                .collect();
            assert_eq!(rules.len(), 4);
            for (rule, (selector, color)) in rules.iter().zip([
                (".cm-highlight", highlight),
                (".cm-strong", strong),
                (".cm-em", italic),
                (".cm-strikethrough", strike),
            ]) {
                assert!(rule.contains(".markdown-source-view.is-live-preview"));
                assert!(rule.contains(selector) && rule.contains(color));
                assert!(rule.contains(".cm-inline-code") && rule.contains(".HyperMD-codeblock *"));
            }
            assert!(css.contains(&format!(
                ".theme-{class} :is(.markdown-reading-view, .markdown-source-view.is-live-preview) .markdown-rendered mark:not([data-highlight]):not(:where(code *, pre *)) {{ background-color: {highlight}; color: inherit; }}"
            )));
        }

        fs::write(&config, stock.split_once("[emphasis.light]").unwrap().0).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [_, (_, css), _] = crate::obsidian::render(&theme);
        assert!(!css.contains(".cm-strikethrough") && !css.contains("mark:not([data-highlight])"));

        fs::write(&config, stock.replacen("#FFE8CB", "#A84B16", 1)).unwrap();
        let Err(error) = resolve::check(&load(&path).unwrap()) else {
            panic!(
                "highlight background matching an emphasis color should fail contrast validation"
            );
        };
        assert!(error.contains("editors/obsidian.toml: emphasis.highlight_background"));
        assert!(error.contains("최소 4.5:1"));
    }

    #[test]
    fn obsidian_inline_code_color_keeps_code_blocks_and_source_mode_unchanged() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let active = r#""--lbrx-inline-code-color" = "inline_code""#;
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let (theme, mut enabled) = render(stock);
        let (_, disabled) = render(&stock.replacen(active, "", 1));
        let (_, mut overridden) =
            render(&stock.replacen(active, r#""--lbrx-inline-code-color" = "text""#, 1));
        for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            for (css, role) in [(&mut enabled, "inline_code"), (&mut overridden, "text")] {
                let rule = format!(
                    concat!(
                        ".theme-{0} .markdown-reading-view .markdown-rendered code:not(pre *),\n",
                        ".theme-{0} .markdown-source-view.is-live-preview .markdown-rendered code:not(pre *),\n",
                        ".theme-{0} .markdown-source-view.is-live-preview .cm-inline-code:not(.HyperMD-codeblock *) {{\n",
                        "  color: {1};\n}}\n"
                    ),
                    class, mode.roles[role]
                );
                assert!(css.contains(&rule));
                *css = css.replacen(&rule, "", 1);
            }
        }
        assert_eq!(enabled, disabled);
        assert_eq!(overridden, disabled);

        fs::write(
            &config,
            stock.replacen(active, r#""--lbrx-inline-code-color" = "background""#, 1),
        )
        .unwrap();
        let Err(error) = resolve::check(&load(&path).unwrap()) else {
            panic!("inline code color matching the background should fail contrast validation");
        };
        assert!(error.contains("colors.--lbrx-inline-code-color"), "{error}");
        assert!(error.contains("최소 4.5:1"), "{error}");
    }

    #[test]
    fn obsidian_blockquote_styles_are_reading_only_and_independently_configurable() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let (before, quote_section) = stock.split_once("[blockquote]").unwrap();
        let (_, after) = quote_section.split_once("[colors]").unwrap();
        let without_quote = format!("{before}[colors]{after}");
        let render = |source: &str| {
            fs::write(&config, source).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        let (theme, mut stock_css) = render(stock);
        let (_, base_css) = render(&without_quote);
        let (_, mut width_css) = render(&format!(
            "{without_quote}\n[blockquote]\nborder_width_px = 0"
        ));
        for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let selector = format!(
                ".theme-{class} .markdown-reading-view .markdown-rendered blockquote:not(.callout *)"
            );
            let color = &mode.roles["quote"];
            let default = format!(
                "{selector} {{\n  color: {color};\n  border-inline-start-color: {color};\n  border-inline-start-width: 3px;\n  padding-inline-start: 0.9em;\n}}\n:where({selector}) .callout:not(:where(.callout *)) {{ color: var(--text-normal); }}\n"
            );
            let width_only = format!("{selector} {{\n  border-inline-start-width: 0px;\n}}\n");
            assert!(stock_css.contains(&default));
            assert!(width_css.contains(&width_only));
            stock_css = stock_css.replacen(&default, "", 1);
            width_css = width_css.replacen(&width_only, "", 1);
        }
        assert_eq!(stock_css, base_css);
        assert_eq!(width_css, base_css);

        for (setting, key) in [
            ("padding_start_em = -0.5", "padding_start_em"),
            ("padding_start_em = inf", "padding_start_em"),
            ("border_role = \"unknown_role\"", "border_role"),
        ] {
            fs::write(&config, format!("{without_quote}\n[blockquote]\n{setting}")).unwrap();
            let Err(error) = load(&path) else {
                panic!("{setting} should fail validation");
            };
            assert!(error.contains(&format!("blockquote.{key}")), "{error}");
        }
    }

    #[test]
    fn obsidian_keyboard_focus_outline_is_reading_only_and_can_be_disabled() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("lbrx.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        let render = || {
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            (theme, css)
        };
        fs::write(&config, stock).unwrap();
        let (theme, enabled) = render();
        fs::write(
            &config,
            stock.replace(
                "keyboard_focus_outline = true",
                "keyboard_focus_outline = false",
            ),
        )
        .unwrap();
        let (_, disabled) = render();
        let mut expected = enabled;
        for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let rule = format!(
                ".theme-{class} .markdown-reading-view .markdown-rendered a:is(.internal-link, .external-link):focus-visible {{\n  outline: 2px solid {};\n  outline-offset: 2px;\n}}\n",
                mode.roles["focus"]
            );
            assert!(expected.contains(&rule));
            expected = expected.replacen(&rule, "", 1);
        }
        assert_eq!(disabled, expected);
    }

    #[test]
    fn obsidian_link_styles_keep_hover_colors_optional_and_underline_states_independent() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        fs::write(
            &config,
            include_str!("../tests/fixtures/editors/obsidian.toml"),
        )
        .unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [_, (_, css), _] = crate::obsidian::render(&theme);
        for key in [
            "link-decoration",
            "link-decoration-hover",
            "link-external-decoration",
            "link-external-decoration-hover",
        ] {
            assert_eq!(
                css.split('}')
                    .filter(|rule| {
                        rule.contains(".internal-link:not(.is-unresolved)")
                            || rule.contains("a.external-link,")
                    })
                    .filter(|rule| rule.contains(&format!("--{key}: underline;")))
                    .count(),
                2
            );
        }
        assert!(!css.contains("--link-color-hover:"));
        assert!(!css.contains("--link-external-color-hover:"));

        fs::write(
            &config,
            r#"
[links]
internal_underline = false
external_hover_underline = true
[colors]
"--link-color" = "function"
"--link-external-color-hover" = "muted"
"#,
        )
        .unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [_, (_, css), _] = crate::obsidian::render(&theme);
        for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
            let internal = format!(
                ".theme-{class} :is(.markdown-reading-view .markdown-rendered a.internal-link:not(.is-unresolved),"
            );
            let internal = css
                .split_once(&internal)
                .unwrap()
                .1
                .split_once("\n}")
                .unwrap()
                .0;
            assert!(internal.contains(&format!("--link-color: {};", mode.roles["function"])));
            assert!(internal.contains("--link-decoration: none;"));
            assert!(!internal.contains("--link-decoration-hover:"));
            let external = format!(
                ".theme-{class} :is(.markdown-reading-view .markdown-rendered a.external-link,"
            );
            let external = css
                .split_once(&external)
                .unwrap()
                .1
                .split_once("\n}")
                .unwrap()
                .0;
            assert!(external.contains(&format!(
                "--link-external-color-hover: {};",
                mode.roles["muted"]
            )));
            assert!(external.contains("--link-external-decoration-hover: underline;"));
            assert!(!external.contains("--link-external-decoration:"));
        }
    }

    #[test]
    fn obsidian_heading_layout_applies_native_sizes_and_reading_only_spacing() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = || {
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        assert!(!render().contains("--h1-size:"));
        let stock = include_str!("../tests/fixtures/editors/obsidian.toml");
        fs::write(&config, stock).unwrap();
        let css = render();
        for (i, size) in [180, 150, 130, 115, 105, 100].into_iter().enumerate() {
            let level = i + 1;
            assert!(css.contains(&format!("--h{level}-size: {size}%;")));
        }
        assert!(
            css.contains(
                " > .el-h1 > h1,\n.markdown-source-view.is-live-preview .HyperMD-header-1,"
            )
        );
        assert!(css.contains(".cm-header-1 {\n  --h1-size: 180%;\n  --h1-weight: 700;\n}"));
        assert!(css.contains(" > .el-h1 > h1 {\n  margin-block: 32px 16px;\n  border-block-end: 1px solid var(--text-muted);"));
        assert_eq!(
            css.split('}')
                .filter(|rule| rule.contains(" > .el-h") && rule.contains("border-block-end:"))
                .count(),
            1
        );
        assert!(!css.contains("--inline-title-size:"));
        fs::write(&config, stock.replacen("weight = 700", "weight = 0", 1)).unwrap();
        let err = load(&path)
            .err()
            .expect("invalid heading weight was accepted");
        assert!(err.contains("obsidian.toml") && err.contains("headings.h1"));
    }

    #[test]
    fn obsidian_heading_colors_default_to_shared_role_and_override_independently() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        for overrides in [false, true] {
            let settings = if overrides {
                "[colors]\n\"--h3-color\" = \"accent\"\n\"--inline-title-color\" = \"muted\"\n"
            } else {
                include_str!("../tests/fixtures/editors/obsidian.toml")
            };
            fs::write(&config, settings).unwrap();
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
                let (_, block) = css.split_once(&format!(".theme-{class} {{\n")).unwrap();
                let (block, _) = block.split_once("\n}").unwrap();
                for level in 1..=6 {
                    let role = if overrides && level == 3 {
                        "accent"
                    } else {
                        "heading"
                    };
                    assert!(block.contains(&format!("--h{level}-color: {};", mode.roles[role])));
                }
                if overrides {
                    assert!(
                        block.contains(&format!("--inline-title-color: {};", mode.roles["muted"]))
                    );
                } else {
                    assert!(!block.contains("--inline-title-color:"));
                }
            }
        }
    }

    #[test]
    fn obsidian_prose_detail_is_opt_in_and_leaves_code_paragraphs_untouched() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("obsidian.toml");
        let render = || {
            let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
            let [_, (_, css), _] = crate::obsidian::render(&theme);
            css
        };
        let shipped = include_str!("../tests/fixtures/editors/obsidian.toml");
        fs::write(&config, shipped).unwrap();
        let baseline = render();
        assert!(!baseline.contains("font-variant-"));
        fs::write(&config, format!("{shipped}\n[prose_detail]\n")).unwrap();
        assert_eq!(render(), baseline);

        fs::write(
            &config,
            format!(
                "{shipped}\n[prose_detail]\nletter_spacing_em = 0.025\ncommon_ligatures = false\nsmall_caps = true\ntext_transform = 'uppercase'\n"
            ),
        )
        .unwrap();
        let output = render();
        let selector = "\n.markdown-reading-view > .markdown-preview-view > .markdown-preview-sizer > .el-p > p:not(:has(code)) {";
        let (before, rule) = output.split_once(selector).unwrap();
        let (added, after) = rule.split_once("\n}\n").unwrap();
        assert_eq!(format!("{before}{after}"), baseline);
        assert!(!added.contains(".markdown-source-view"));
        for declaration in [
            "letter-spacing: 0.025em;",
            "font-variant-ligatures: no-common-ligatures;",
            "font-variant-caps: small-caps;",
            "text-transform: uppercase;",
        ] {
            assert!(added.contains(declaration), "{declaration}");
        }

        for value in ["nan", "0.51", "-0.51"] {
            fs::write(
                &config,
                format!("[prose_detail]\nletter_spacing_em = {value}\n"),
            )
            .unwrap();
            let err = load(&path)
                .err()
                .expect("invalid letter spacing was accepted");
            assert!(err.contains("obsidian.toml") && err.contains("letter_spacing_em"));
        }
    }

    #[test]
    fn helix_styles_override_only_selected_fields_and_inherit_generated_parent() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("helix.toml");
        let render =
            || crate::helix::render(&resolve::resolve(&load(&path).unwrap()).unwrap()).unwrap();
        fs::write(&config, "[colors]\n\"ui.linenr.selected\" = \"text\"").unwrap();
        let baseline = render();
        fs::write(
            &config,
            include_str!("../themes/green-type/editors/helix.toml"),
        )
        .unwrap();
        assert_eq!(render()[..2], baseline[..2]);

        fs::write(
            &config,
            r#"
[colors]
"ui.linenr.selected" = "text"
[styles]
"type" = { fg = "syntax_type", modifiers = ["bold"] }
"type.builtin" = { bg = "default", modifiers = [] }
"diagnostic.warning" = { underline = { style = "line" } }
"ui.linenr.selected" = { fg = "focus" }
"diff.plus.gutter" = { fg = "success" }
"markup.heading.1" = { modifiers = ["bold"] }
"#,
        )
        .unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let files = crate::helix::render(&theme).unwrap();
        for (index, mode) in [&theme.light, &theme.dark].into_iter().enumerate() {
            let output: toml::Table = toml::from_str(&files[index].1).unwrap();
            let base: toml::Table = toml::from_str(&baseline[index].1).unwrap();
            let builtin = &output["type.builtin"];
            assert_eq!(
                builtin["fg"].as_str(),
                Some(mode.roles["syntax_type"].as_str())
            );
            assert_eq!(builtin["bg"].as_str(), Some("default"));
            assert!(builtin["modifiers"].as_array().unwrap().is_empty());
            let underline = &output["diagnostic.warning"]["underline"];
            assert_eq!(underline["style"].as_str(), Some("line"));
            assert_eq!(
                underline["color"],
                base["diagnostic.warning"]["underline"]["color"]
            );
            assert_eq!(
                output["ui.linenr.selected"]["fg"].as_str(),
                Some(mode.roles["focus"].as_str())
            );
            assert_eq!(
                output["diff.plus.gutter"]["fg"].as_str(),
                Some(mode.roles["success"].as_str())
            );
            assert_eq!(output["markup.heading.1"]["fg"], base["markup.heading"]);
            assert_eq!(output["ui.selection"], base["ui.selection"]);
            assert!(!output.contains_key("ui.cursor.match"));
        }
    }

    #[test]
    fn helix_styles_reject_unknown_scopes_roles_and_styles() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("helix.toml");
        for (setting, expected) in [
            (
                "\"ui.cursor.unknown\" = { fg = 'focus' }",
                "ui.cursor.unknown",
            ),
            ("\"type.unknown\" = { fg = 'focus' }", "type.unknown"),
            ("type = { fg = 'missing' }", "styles.type.fg"),
            ("type = { underline = { style = 'wave' } }", "wave"),
            ("type = { modifiers = ['regular'] }", "regular"),
            ("type = { underline = {} }", "styles.type.underline"),
        ] {
            fs::write(&config, format!("[styles]\n{setting}")).unwrap();
            let err = load(&path).err().expect("invalid Helix style was accepted");
            assert!(
                err.contains("helix.toml") && err.contains(expected),
                "{err}"
            );
        }
    }

    #[test]
    fn helix_exclusive_examples_generate_each_scope_in_both_modes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let config = editors.join("helix.toml");
        let source = include_str!("../themes/green-type/editors/helix.toml");
        let settings = source
            .lines()
            .skip_while(|line| *line != "# Helix 전용 예시")
            .filter_map(|line| {
                line.strip_prefix("# ")
                    .filter(|entry| entry.starts_with('"'))
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&config, source).unwrap();
        let baseline =
            crate::helix::render(&resolve::resolve(&load(&path).unwrap()).unwrap()).unwrap();
        fs::write(&config, format!("{source}\n{settings}\n")).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let files = crate::helix::render(&theme).unwrap();
        let ledger = include_str!("../Docs/17-helix-options.md");
        for (index, mode) in [&theme.light, &theme.dark].into_iter().enumerate() {
            let output: toml::Table = toml::from_str(&files[index].1).unwrap();
            let base: toml::Table = toml::from_str(&baseline[index].1).unwrap();
            for (scope, _) in ledger
                .lines()
                .filter_map(|line| line.strip_prefix("| `")?.split_once("` | Helix 전용 | "))
            {
                assert!(output.contains_key(scope), "missing scope: {scope}");
                assert!(!base.contains_key(scope), "not opt-in: {scope}");
            }
            assert_eq!(
                output["ui.cursor.primary.insert"]["fg"].as_str(),
                Some(mode.roles["background"].as_str())
            );
            assert_eq!(
                output["ui.cursor.primary.insert"]["bg"].as_str(),
                Some(mode.roles["success"].as_str())
            );
            assert_eq!(
                output["diagnostic.hint"]["underline"]["color"].as_str(),
                Some(mode.roles["muted"].as_str())
            );
            assert_eq!(
                output["diagnostic.warning"]["underline"]["style"].as_str(),
                Some("curl")
            );
        }
    }

    #[test]
    fn vscode_workbench_colors_are_opt_in_and_preserve_overlay_alpha() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        let config = editors.join("vscode.toml");
        let output = tmp.path().join("dist");
        fs::write(
            &config,
            "[colors]\n\"tab.activeBackground\" = \"surface\"\n\
             \"editor.findMatchHighlightBackground\" = \"focus\"\n\
             \"diffEditor.removedTextBackground\" = \"error\"\n\
             \"editorLineNumber.activeForeground\" = \"focus\"\n\
             \"terminal.inactiveSelectionBackground\" = \"selection_background\"\n\
             \"quickInput.background\" = \"surface\"\n\
             \"editor.wordHighlightBackground\" = \"focus@20\"\n\
             \"editor.findMatchBackground\" = \"warning@50\"",
        )
        .unwrap();
        let theme = load(&path).unwrap();
        let resolved = resolve::resolve(&theme).unwrap();
        build::build(&theme, &output, &[App::VsCode]).unwrap();
        let file =
            fs::File::open(App::VsCode.output_path(&output).join(crate::vscode::VSIX)).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        for (name, mode) in [("light", &resolved.light), ("dark", &resolved.dark)] {
            let parsed: serde_json::Value = serde_json::from_reader(
                zip.by_name(&format!("extension/themes/lbrx-{name}.json"))
                    .unwrap(),
            )
            .unwrap();
            let colors = parsed["colors"].as_object().unwrap();
            assert_eq!(colors["tab.activeBackground"], mode.roles["surface"]);
            assert_eq!(
                colors["editor.findMatchHighlightBackground"],
                format!("{}33", mode.roles["focus"])
            );
            assert_eq!(
                colors["diffEditor.removedTextBackground"],
                format!("{}4D", mode.roles["error"])
            );
            assert_eq!(
                colors["editorLineNumber.activeForeground"],
                mode.roles["focus"]
            );
            assert_eq!(
                colors["terminal.inactiveSelectionBackground"],
                format!("{}80", mode.roles["selection_background"])
            );
            assert_eq!(colors["quickInput.background"], mode.roles["surface"]);
            assert_eq!(
                colors["editor.wordHighlightBackground"],
                format!("{}33", mode.roles["focus"])
            );
            assert_eq!(
                colors["editor.findMatchBackground"],
                format!("{}80", mode.roles["warning"])
            );
            assert!(!colors.contains_key("tab.inactiveBackground"));
        }
        for (key, role) in [
            ("tab.activeBackground", "missing"),
            ("tab.unknownBackground", "surface"),
            ("editor.wordHighlightBackground", "focus@101"),
            ("editor.wordHighlightBackground", "missing@20"),
        ] {
            fs::write(&config, format!("[colors]\n\"{key}\" = \"{role}\"")).unwrap();
            let err = load(&path).err().expect("invalid override was accepted");
            assert!(err.contains("vscode.toml") && err.contains(key), "{err}");
        }
    }

    #[test]
    fn mac_terminal_overrides_stay_app_specific_and_archive_mode_colors() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("theme.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let editors = tmp.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(
            editors.join("iterm2.toml"),
            "[colors]\n'Link Color' = 'link'\n'Bold Color' = 'text'",
        )
        .unwrap();
        fs::write(
            editors.join("macos-terminal.toml"),
            "[colors]\nTextColor = 'muted'\nTextBoldColor = 'focus'",
        )
        .unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let iterm = crate::iterm2::render(&theme).unwrap();
        let terminal = crate::macos_terminal::render(&theme).unwrap();
        let parse = |xml: &str| {
            plist::Value::from_reader_xml(xml.as_bytes())
                .unwrap()
                .into_dictionary()
                .unwrap()
        };
        for (index, mode) in [&theme.light, &theme.dark].into_iter().enumerate() {
            let colors = parse(&iterm[index].1);
            let link = colors["Link Color"].as_dictionary().unwrap();
            let [r, _, _] = resolve::parse_rgb(&mode.roles["link"]).unwrap();
            assert_eq!(link["Red Component"].as_real(), Some(f64::from(r) / 255.0));
            assert!(colors.contains_key("Bold Color"));
            assert!(!colors.contains_key("Cursor Text Color"));

            let profile = parse(&terminal[index].1);
            assert_eq!(profile["type"].as_string(), Some("Window Settings"));
            assert_eq!(profile.len(), 24);
            assert_eq!(
                profile.keys().filter(|key| key.starts_with("ANSI")).count(),
                16
            );
            assert!(!profile.contains_key("Link Color"));
            for (key, hex) in [
                ("TextColor", &mode.roles["muted"]),
                ("TextBoldColor", &mode.roles["focus"]),
                ("ANSIBrightMagentaColor", &mode.ansi[13]),
            ] {
                let archive = plist::Value::from_reader(std::io::Cursor::new(
                    profile[key].as_data().unwrap(),
                ))
                .unwrap();
                let graph = archive.as_dictionary().unwrap();
                assert_eq!(graph["$archiver"].as_string(), Some("NSKeyedArchiver"));
                let root = graph["$top"].as_dictionary().unwrap()["root"]
                    .as_uid()
                    .unwrap();
                assert_eq!(root.get(), 1);
                let object = graph["$objects"].as_array().unwrap()[1]
                    .as_dictionary()
                    .unwrap();
                let rgb = std::str::from_utf8(object["NSRGB"].as_data().unwrap()).unwrap();
                let channels: Vec<_> = rgb
                    .trim_end_matches('\0')
                    .split_whitespace()
                    .map(|part| part.parse::<f64>().unwrap())
                    .collect();
                assert_eq!(
                    channels,
                    resolve::parse_rgb(hex)
                        .unwrap()
                        .map(|c| f64::from(c) / 255.0)
                );
            }
        }
    }

    #[test]
    fn vim_highlights_are_opt_in_and_use_mode_roles() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let path = root.join("theme.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        fs::create_dir(root.join("editors")).unwrap();
        let vim_path = root.join("editors/vim.toml");
        let render = || crate::vim::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        fs::write(&vim_path, "[colors]\nLineNr = 'muted'").unwrap();
        let before = render();
        let template = include_str!("../themes/green-type/editors/vim.toml");
        fs::write(&vim_path, template).unwrap();
        assert_eq!(render(), before);

        let enabled = template
            .lines()
            .map(|line| {
                line.strip_prefix("# ")
                    .filter(|line| line.contains(" = {"))
                    .unwrap_or(line)
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&vim_path, enabled).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        for ((_, script), mode) in crate::vim::render(&theme)
            .iter()
            .zip([&theme.light, &theme.dark])
        {
            assert!(script.contains(&format!(
                "highlight ColorColumn guibg={} ctermbg=",
                mode.roles["surface"]
            )));
            assert!(script.lines().any(|line| {
                line.starts_with(&format!(
                    "highlight SpellBad guisp={} ctermul=",
                    mode.roles["error"]
                )) && line.ends_with(" gui=undercurl cterm=undercurl")
            }));
            assert!(
                script.contains("highlight Conceal guifg=")
                    && script.contains("guibg=NONE ctermbg=NONE")
            );
            assert!(script.contains("highlight User9 "));
        }
    }

    #[test]
    fn vim_highlights_reject_unknown_groups_fields_styles_and_roles() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let path = root.join("theme.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        fs::create_dir(root.join("editors")).unwrap();
        let vim_path = root.join("editors/vim.toml");
        for (line, expected) in [
            ("Menu = { fg = 'text' }", "highlights.Menu"),
            ("Folded = { background = 'surface' }", "background"),
            ("SpellBad = { styles = ['unknown'] }", "unknown"),
            ("SpellBad = { sp = 'missing' }", "highlights.SpellBad.sp"),
            ("User1 = {}", "highlights.User1"),
        ] {
            fs::write(&vim_path, format!("[highlights]\n{line}")).unwrap();
            let err = load(&path)
                .err()
                .expect("invalid Vim override was accepted");
            assert!(err.contains("vim.toml") && err.contains(expected), "{err}");
        }
    }

    #[test]
    fn studio_sidecar_accepts_roles_and_rejects_unknown_keys_without_publishing() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let source = root.join("theme.toml");
        fs::write(&source, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        fs::create_dir(root.join("editors")).unwrap();
        let config = root.join("editors/lbrx-studio.toml");
        fs::write(&config, "[colors]\n'--accent' = 'function'\n").unwrap();
        let theme = load(&source).unwrap();
        assert_eq!(theme.layers.editors["lbrx-studio"]["--accent"], "function");
        let out = root.join("dist");
        build::build(&theme, &out, &[App::LbrxStudio]).unwrap();
        let output = App::LbrxStudio.output_path(&out).join("theme.css");
        let baseline = fs::read(&output).unwrap();
        for (text, message) in [
            ("[colors]\n'--typo' = 'accent'", "지원하지 않는 키"),
            ("[colors]\n'--accent' = 'missing'", "missing"),
            ("[colors]\n'--accent' = '#123456'", "#123456"),
            ("[layout]\nwidth = 240", "unknown field"),
        ] {
            fs::write(&config, text).unwrap();
            let error = load(&source)
                .err()
                .expect("invalid Studio config was accepted");
            assert!(error.contains("lbrx-studio.toml"), "{error}");
            assert!(error.contains(message), "{error}");
            assert_eq!(fs::read(&output).unwrap(), baseline);
        }
    }

    #[test]
    fn sparse_languages_override_only_matching_outputs_and_editors_are_isolated() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let path = root.join("theme.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let config = include_str!("../tests/fixtures/editors/typora.toml");
        fs::write(&sidecar, config).unwrap();
        let theme = resolve::resolve(&load(&path).unwrap()).unwrap();
        let [(_, light), (_, dark), _] = crate::typora::render(&theme);
        for (css, strong, italic, strike, highlight) in [
            (light, "#A84B16", "#245E8A", "#6D5A4B", "#FFE8CB"),
            (dark, "#FFC695", "#B4D9F5", "#C0AC9B", "#503927"),
        ] {
            assert!(css.contains(&format!("font-weight: 700; color: {strong};")));
            assert!(css.contains(&format!("font-style: italic; color: {italic};")));
            assert!(css.contains(&format!(
                "text-decoration-line: line-through; color: {strike};"
            )));
            assert!(css.contains(&format!("background-color: {highlight}; color: inherit;")));
        }
        fs::write(&sidecar, config.replace("#6D5A4B", "#707070")).unwrap();
        let error = resolve::resolve(&load(&path).unwrap()).unwrap_err();
        assert!(error.contains("emphasis.strikethrough"));
        assert!(error.contains("emphasis.highlight_background"));
    }

    #[test]
    fn typora_footnotes_and_rule_render_only_when_configured() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        for (css, blue) in [(light, "#245E8A"), (dark, "#B4D9F5")] {
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        for (css, surface) in [(light, "#FFEAD6"), (dark, "#38313A")] {
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
            for (css, accent) in [(light, "#A84B16"), (dark, "#FFC695")] {
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
            .zip([("#A84B16", "#795814"), ("#FFC695", "#E2C080")])
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        assert!(!render()[0].1.contains(".md-alert"));
        let sidecar = dir.join("typora.toml");
        for shaded in [true, false] {
            fs::write(&sidecar, format!("[alerts]\nshaded_background = {shaded}")).unwrap();
            let [(_, light), (_, dark), _] = render();
            for (css, note) in [(light, "#245E8A"), (dark, "#B4D9F5")] {
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
            .zip([("#245E8A", "#A84B16"), ("#B4D9F5", "#FFC695")])
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let render = || crate::typora::render(&resolve::resolve(&load(&path).unwrap()).unwrap());
        let baseline = render();
        fs::write(&sidecar, "[colors]\n'--blur-text-color' = 'muted'").unwrap();
        let custom = render();
        for (((_, before), (_, after)), muted) in
            baseline.iter().zip(&custom).zip(["#6D5A4B", "#C0AC9B"])
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        for (css, text, url) in [(light, "#A84B16", "#245E8A"), (dark, "#FFC695", "#B4D9F5")] {
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
    fn typora_sidebar_file_list_uses_mode_colors_and_removes_nested_opacity() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        fs::create_dir(tmp.path().join("editors")).unwrap();
        let config = tmp.path().join("editors/typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let baseline = crate::typora::render(&resolve());
        fs::write(
            &config,
            "[sidebar]\nbackground = 'surface'\ntext = 'text'\nmuted = 'muted'",
        )
        .unwrap();
        let theme = resolve();
        let output = crate::typora::render(&theme);
        for (index, mode) in [&theme.light, &theme.dark].into_iter().enumerate() {
            let css = &output[index].1;
            assert!(css.contains(&format!(
                "#typora-sidebar {{ --side-bar-bg-color: {}; --active-file-bg-color: {}; --active-file-text-color: {};",
                mode.roles["surface"], mode.roles["selection_background"], mode.roles["selection_text"]
            )));
            assert!(css.contains(&format!(
                ".file-list-item) {{ color: {}; opacity: 1; }}",
                mode.roles["text"]
            )));
            assert!(css.contains(&format!(
                ".file-list-item-file-ext-part) {{ color: {}; opacity: 1; }}",
                mode.roles["muted"]
            )));
            assert!(css.contains(".file-list-item:hover:not(.active)"));
            let without_sidebar = css
                .split_inclusive('\n')
                .filter(|line| !line.starts_with("#typora-sidebar"))
                .collect::<String>();
            assert_eq!(without_sidebar, baseline[index].1);
        }
    }

    #[test]
    fn typora_sidebar_rejects_unknown_roles_and_panel_options() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("input.toml");
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        for (config, field) in [
            ("[sidebar]\nbackground = 'unknown'", "sidebar.background"),
            ("[sidebar]\ntext = 'unknown'", "sidebar.text"),
            ("[sidebar]\nmuted = 'unknown'", "sidebar.muted"),
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        let sidecar = dir.join("typora.toml");
        let resolve = || resolve::resolve(&load(&path).unwrap()).unwrap();
        let baseline = crate::typora::render(&resolve());

        fs::write(&sidecar, "[editing_aids.yaml]").unwrap();
        assert_eq!(crate::typora::render(&resolve()), baseline);
        fs::write(
            &sidecar,
            "[editing_aids.yaml]\nbackground = 'surface'\ntext = 'text'\nplaceholder = 'muted'\n\
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
                ("#write > pre.md-meta-block", "background-color", "surface"),
                (
                    "#write > pre.md-meta-block :is(.CodeMirror, .CodeMirror-gutters)",
                    "background-color",
                    "surface",
                ),
                ("#write > pre.md-meta-block", "color", "text"),
                ("#write > pre.md-meta-block .CodeMirror", "color", "text"),
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
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
        fs::write(&path, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let dir = tmp.path().join("editors");
        fs::create_dir(&dir).unwrap();
        for (key, role) in [("unknown", "text"), ("LineNr", "typo")] {
            fs::write(dir.join("vim.toml"), format!("[colors]\n{key}='{role}'")).unwrap();
            let err = load(&path).err().unwrap();
            assert!(err.contains("vim.toml") && err.contains(key));
        }
    }
}
