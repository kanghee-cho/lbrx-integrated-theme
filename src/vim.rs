use crate::resolve::{ResolvedMode, ResolvedTheme, parse_rgb};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "vim/colors";
pub const FILES: &[&str] = &["lbrx_light.vim", "lbrx_dark.vim", crate::build::GUIDE];

const GROUPS: &[(&str, &str, Option<&str>)] = &[
    ("Normal", "text", Some("background")),
    ("LineNr", "muted", None),
    ("CursorLine", "NONE", Some("surface")),
    ("Visual", "selection_text", Some("selection_background")),
    ("Search", "background", Some("warning")),
    ("IncSearch", "background", Some("focus")),
    ("StatusLine", "text", Some("surface")),
    ("ErrorMsg", "error", None),
    ("WarningMsg", "warning", None),
    ("Comment", "comment", None),
    ("Keyword", "keyword", None),
    ("String", "string", None),
    ("Function", "function", None),
    ("Number", "number", None),
    ("Error", "error", None),
];

const SHARED_GROUPS: &[(&str, &str, &str, &str)] = &[
    ("Cursor", "background", "focus", "NONE"),
    ("DiffAdd", "NONE", "surface", "NONE"),
    ("DiffChange", "NONE", "selection_background", "NONE"),
    ("DiffDelete", "error", "surface", "NONE"),
    ("DiffText", "background", "warning", "bold"),
    ("SignColumn", "muted", "NONE", "NONE"),
    ("CursorLineNr", "focus", "NONE", "bold"),
    ("CursorLineFold", "muted", "surface", "NONE"),
    ("CursorLineSign", "muted", "surface", "NONE"),
    ("MatchParen", "focus", "NONE", "bold,underline"),
    ("Pmenu", "text", "surface", "NONE"),
    ("PmenuSel", "selection_text", "selection_background", "bold"),
    ("PmenuKind", "focus", "surface", "NONE"),
    (
        "PmenuKindSel",
        "selection_text",
        "selection_background",
        "bold",
    ),
    ("PmenuExtra", "muted", "surface", "NONE"),
    (
        "PmenuExtraSel",
        "selection_text",
        "selection_background",
        "bold",
    ),
    ("PmenuSbar", "NONE", "selection_background", "NONE"),
    ("PmenuThumb", "NONE", "focus", "NONE"),
    ("CurSearch", "background", "focus", "bold"),
    ("StatusLineNC", "muted", "surface", "NONE"),
    ("StatusLineTerm", "text", "surface", "NONE"),
    ("StatusLineTermNC", "muted", "surface", "NONE"),
    ("Underlined", "link", "NONE", "underline"),
    ("Todo", "warning", "NONE", "bold"),
];

const LANGUAGE_GROUPS: &[(&str, &str, &str)] = &[
    ("python", "function", "pythonFunction"),
    (
        "python",
        "string",
        "pythonString pythonRawString pythonFString",
    ),
    ("python", "number", "pythonNumber"),
    ("python", "comment", "pythonComment"),
    ("rust", "keyword", "rustKeyword"),
    ("rust", "function", "rustFunction rustFuncName rustFuncCall"),
    ("rust", "comment", "rustCommentLine"),
    ("rust", "string", "rustString"),
    ("rust", "number", "rustNumber"),
    ("toml", "number", "tomlInteger"),
    ("toml", "string", "tomlString"),
    ("toml", "comment", "tomlComment"),
    ("powershell", "function", "ps1Cmdlet"),
    ("powershell", "keyword", "ps1Keyword"),
    ("powershell", "string", "ps1String"),
    ("powershell", "number", "ps1Number"),
    ("powershell", "comment", "ps1Comment"),
    ("typescript", "number", "typescriptNumber"),
    ("typescript", "comment", "typescriptLineComment"),
    ("rust", "function.macro", "rustMacro"),
    ("rust", "attribute", "rustLifetime rustLabel rustAttribute"),
    ("rust", "keyword.declaration", "rustMutability rustStorage"),
    ("rust", "punctuation", "rustModPathSep"),
    ("python", "attribute", "pythonDecorator pythonDottedName"),
    ("python", "string.escape", "pythonEscape"),
    ("powershell", "variable", "ps1Variable"),
    ("powershell", "variable.parameter", "ps1Switch"),
    ("json", "property", "jsonKeyword jsonQuote"),
    ("json", "constant.builtin", "jsonBoolean jsonNull"),
    ("jsonc", "property", "jsoncKeyword"),
    ("yaml", "property", "yamlBlockMappingKey yamlFlowMappingKey"),
    ("yaml", "attribute", "yamlAnchor yamlAlias"),
    ("yaml", "punctuation", "yamlDocumentStart yamlDocumentEnd"),
    ("toml", "type", "tomlTable tomlTableArray"),
    ("toml", "property", "tomlKey"),
    (
        "toml",
        "constant.builtin",
        "tomlBoolean tomlDate tomlDateTime tomlTime",
    ),
];

pub(crate) fn has_language_groups(lang: &str, theme: &ResolvedTheme) -> bool {
    LANGUAGE_GROUPS
        .iter()
        .any(|&(id, token, _)| id == lang && theme.has_token_override(lang, token))
}

const SYNTAX_GROUPS: &[(&str, &str)] = &[
    ("Conditional", "keyword.control"),
    ("Repeat", "keyword.control"),
    ("Statement", "keyword.control"),
    ("Label", "keyword.control"),
    ("Exception", "keyword.control"),
    ("StorageClass", "keyword.declaration"),
    ("Structure", "keyword.declaration"),
    ("Typedef", "keyword.declaration"),
    ("Define", "keyword.declaration"),
    ("Character", "string"),
    ("SpecialChar", "string.escape"),
    ("Constant", "constant.builtin"),
    ("Boolean", "constant.builtin"),
    ("Macro", "function.macro"),
    ("Type", "type"),
    ("Identifier", "variable"),
    ("Operator", "operator"),
    ("PreProc", "attribute"),
    ("Include", "attribute"),
    ("PreCondit", "attribute"),
    ("Special", "attribute"),
    ("Tag", "attribute"),
    ("Delimiter", "punctuation"),
    ("SpecialComment", "comment"),
];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 3] {
    [
        (FILES[0], colorscheme(theme, "light", &theme.light)),
        (FILES[1], colorscheme(theme, "dark", &theme.dark)),
        (
            crate::build::GUIDE,
            include_str!("../Docs/12-vim.md").into(),
        ),
    ]
}

fn colorscheme(theme: &ResolvedTheme, background: &str, mode: &ResolvedMode) -> String {
    let mut script = format!(
        "\" LBRX - generated from the theme source.\n\
         unlet! g:colors_name\n\
         set background={background}\n\
         highlight clear\n\
         if exists('syntax_on')\n  syntax reset\nendif\n\
         let g:colors_name = 'lbrx_{background}'\n\n"
    );
    let cterm = |hex: &str| {
        parse_rgb(hex).map_or_else(|| "NONE".to_owned(), |rgb| xterm_index(rgb).to_string())
    };
    for &(group, fg, bg) in GROUPS {
        let guifg = match group {
            "Comment" | "Keyword" | "String" | "Function" | "Number" | "Error" => {
                theme.token(mode, "", fg)
            }
            _ if fg == "NONE" => "NONE",
            _ => &mode.roles[fg],
        };
        let guifg = theme.editor_color(mode, "vim", group, guifg);
        let guibg = bg.map_or("NONE", |role| mode.roles[role].as_str());
        let (ctermfg, ctermbg) = (cterm(guifg), cterm(guibg));
        let style = if matches!(
            group,
            "Comment" | "Keyword" | "String" | "Function" | "Number" | "Error"
        ) {
            vim_style(theme, "", fg)
        } else {
            "NONE"
        };
        writeln!(
            script,
            "highlight {group} guifg={guifg} guibg={guibg} ctermfg={ctermfg} ctermbg={ctermbg} gui={style} cterm={style}"
        )
        .unwrap();
    }
    let color = |role: &str| {
        if role == "NONE" {
            "NONE"
        } else {
            mode.roles[role].as_str()
        }
    };
    for &(group, fg, bg, style) in SHARED_GROUPS {
        let (guifg, guibg) = (color(fg), color(bg));
        writeln!(
            script,
            "highlight {group} guifg={guifg} guibg={guibg} ctermfg={} ctermbg={} gui={style} cterm={style}",
            cterm(guifg),
            cterm(guibg)
        )
        .unwrap();
    }
    script.push_str("highlight! link Float Number\nhighlight! link Debug Function\n");
    for &(group, token) in SYNTAX_GROUPS {
        let hex = theme.token(mode, "", token);
        let style = vim_style(theme, "", token);
        writeln!(
            script,
            "highlight {group} guifg={hex} ctermfg={} gui={style} cterm={style}",
            cterm(hex)
        )
        .unwrap();
    }
    for &(lang, token, groups) in LANGUAGE_GROUPS {
        if theme.has_token_override(lang, token) {
            let hex = theme.token(mode, lang, token);
            let style = vim_style(theme, lang, token);
            for group in groups.split_whitespace() {
                writeln!(
                    script,
                    "highlight {group} guifg={hex} ctermfg={} gui={style} cterm={style}",
                    cterm(hex)
                )
                .unwrap();
            }
        }
    }
    for (group, highlight) in &theme.layers.vim_highlights {
        write!(script, "highlight {group}").unwrap();
        for (role, gui, term) in [
            (&highlight.fg, "guifg", "ctermfg"),
            (&highlight.bg, "guibg", "ctermbg"),
            (&highlight.sp, "guisp", "ctermul"),
        ] {
            if let Some(role) = role {
                let hex = if role == "none" {
                    "NONE"
                } else {
                    &mode.roles[role]
                };
                write!(script, " {gui}={hex} {term}={}", cterm(hex)).unwrap();
            }
        }
        if let Some(styles) = &highlight.styles {
            let styles = if styles.is_empty() {
                "NONE".into()
            } else {
                styles
                    .iter()
                    .map(|style| style.name())
                    .collect::<Vec<_>>()
                    .join(",")
            };
            write!(script, " gui={styles} cterm={styles}").unwrap();
        }
        script.push('\n');
    }
    script
}

fn vim_style(theme: &ResolvedTheme, lang: &str, token: &str) -> &'static str {
    match theme.token_style(lang, token).name() {
        "normal" => "NONE",
        value => value,
    }
}

fn xterm_index(rgb: [u8; 3]) -> u8 {
    const LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    // The first 16 slots are controlled by the terminal host.
    (16..=255)
        .min_by_key(|&index| {
            let candidate = if index < 232 {
                let cube = usize::from(index - 16_u8);
                [LEVELS[cube / 36], LEVELS[cube / 6 % 6], LEVELS[cube % 6]]
            } else {
                [8 + (index - 232) * 10; 3]
            };
            rgb.into_iter()
                .zip(candidate)
                .map(|(a, b)| (i32::from(a) - i32::from(b)).pow(2))
                .sum::<i32>()
        })
        .expect("xterm palette is not empty")
}
