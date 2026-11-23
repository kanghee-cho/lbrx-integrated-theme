use crate::resolve::{ResolvedMode, ResolvedTheme};
use serde_json::{Value, json};

pub const DIRECTORY: &str = "windows-terminal";
pub const SCHEMES: &str = "schemes.json";
pub const FONT: &str = "profile-font.json";
pub const GUIDE: &str = "README.md";
pub const FILES: [&str; 3] = [SCHEMES, FONT, GUIDE];

const ANSI_FIELDS: [&str; 16] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "purple",
    "cyan",
    "white",
    "brightBlack",
    "brightRed",
    "brightGreen",
    "brightYellow",
    "brightBlue",
    "brightPurple",
    "brightCyan",
    "brightWhite",
];

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, String); 3], String> {
    let schemes = json!({
        "schemes": [
            scheme(theme, "LBRX Light", &theme.light),
            scheme(theme, "LBRX Dark", &theme.dark)
        ]
    });
    let mono = &theme.typography.mono;
    let size_pt = mono.size_px * 0.75;
    if size_pt < 1.0 {
        return Err(format!(
            "{DIRECTORY}: typography.mono.size_px를 변환한 {size_pt}pt는 font.size 최소 1pt 미만"
        ));
    }
    let font = json!({ "font": { "face": mono.families[0], "size": size_pt } });

    Ok([
        (SCHEMES, encode(SCHEMES, &schemes)?),
        (FONT, encode(FONT, &font)?),
        (GUIDE, INCLUDE_GUIDE.into()),
    ])
}

#[cfg(windows)]
const INCLUDE_GUIDE: &str = include_str!("..\\Docs\\06-windows-terminal.md");
#[cfg(not(windows))]
const INCLUDE_GUIDE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Docs/06-windows-terminal.md"
));

fn scheme(theme: &ResolvedTheme, name: &str, mode: &ResolvedMode) -> Value {
    let mut scheme = serde_json::Map::new();
    scheme.insert("name".into(), json!(name));
    for (field, role) in [
        ("background", "terminal_background"),
        ("foreground", "terminal_foreground"),
        ("cursorColor", "terminal_cursor"),
        ("selectionBackground", "selection_background"),
    ] {
        scheme.insert(
            field.into(),
            json!(theme.editor_color(mode, DIRECTORY, field, &mode.roles[role])),
        );
    }
    for (field, color) in ANSI_FIELDS.into_iter().zip(&mode.ansi) {
        scheme.insert(field.into(), json!(color));
    }
    Value::Object(scheme)
}

fn encode(file: &str, value: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(value)
        .map(|contents| contents + "\n")
        .map_err(|error| format!("{DIRECTORY}\\{file}: JSON 생성 실패: {error}"))
}
