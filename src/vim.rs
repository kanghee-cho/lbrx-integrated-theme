use crate::resolve::{ResolvedMode, ResolvedTheme, parse_rgb};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "vim/colors";
pub const FILES: &[&str] = &["lbrx_light.vim", "lbrx_dark.vim"];

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
];

pub fn language_groups(language: &str, token: &str) -> Option<&'static str> {
    LANGUAGE_GROUPS
        .iter()
        .find_map(|&(lang, kind, groups)| (lang == language && kind == token).then_some(groups))
}

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 2] {
    [
        (FILES[0], colorscheme(theme, "light", &theme.light)),
        (FILES[1], colorscheme(theme, "dark", &theme.dark)),
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
        writeln!(
            script,
            "highlight {group} guifg={guifg} guibg={guibg} ctermfg={ctermfg} ctermbg={ctermbg} gui=NONE cterm=NONE"
        )
        .unwrap();
    }
    script.push_str("highlight! link Conditional Keyword\nhighlight! link Repeat Keyword\nhighlight! link Float Number\n");
    for &(lang, token, groups) in LANGUAGE_GROUPS {
        let (role, source) = theme.token_role(lang, token);
        if source == crate::resolve::ColorSource::Language {
            let hex = &mode.roles[role];
            for group in groups.split_whitespace() {
                writeln!(
                    script,
                    "highlight {group} guifg={hex} ctermfg={}",
                    cterm(hex)
                )
                .unwrap();
            }
        }
    }
    script
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
