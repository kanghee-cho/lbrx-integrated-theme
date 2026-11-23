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
    ("StorageClass", "keyword.declaration"),
    ("Structure", "keyword.declaration"),
    ("Typedef", "keyword.declaration"),
    ("SpecialChar", "string.escape"),
    ("Boolean", "constant.builtin"),
    ("Type", "type"),
    ("Identifier", "variable"),
    ("Operator", "operator"),
    ("Delimiter", "punctuation"),
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
    script.push_str("highlight! link Float Number\n");
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
