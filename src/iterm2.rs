use crate::resolve::{ResolvedMode, ResolvedTheme, parse_rgb};
use plist::{Dictionary, Value};

pub const DIRECTORY: &str = "iterm2";
pub const FILES: [&str; 2] = ["LBRX Light.itermcolors", "LBRX Dark.itermcolors"];

const ROLES: [(&str, &str); 5] = [
    ("Foreground Color", "terminal_foreground"),
    ("Background Color", "terminal_background"),
    ("Cursor Color", "terminal_cursor"),
    ("Selection Color", "selection_background"),
    ("Selected Text Color", "selection_text"),
];

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, String); 2], String> {
    Ok([
        (FILES[0], encode(theme, &theme.light, FILES[0])?),
        (FILES[1], encode(theme, &theme.dark, FILES[1])?),
    ])
}

fn encode(theme: &ResolvedTheme, mode: &ResolvedMode, filename: &str) -> Result<String, String> {
    let mut colors = Dictionary::new();
    for (name, role) in ROLES {
        colors.insert(
            name.into(),
            color(theme.editor_color(mode, DIRECTORY, name, &mode.roles[role])),
        );
    }
    for (slot, hex) in mode.ansi.iter().enumerate() {
        colors.insert(format!("Ansi {slot} Color"), color(hex));
    }
    let mut xml = Vec::new();
    Value::Dictionary(colors)
        .to_writer_xml(&mut xml)
        .map_err(|error| format!("{DIRECTORY}\\{filename}: plist 생성 실패: {error}"))?;
    String::from_utf8(xml).map_err(|error| format!("{DIRECTORY}\\{filename}: {error}"))
}

fn color(hex: &str) -> Value {
    let [red, green, blue] = parse_rgb(hex).expect("resolved color");
    Value::Dictionary(
        [
            ("Color Space", Value::String("sRGB".into())),
            ("Alpha Component", Value::Real(1.0)),
            ("Red Component", Value::Real(f64::from(red) / 255.0)),
            ("Green Component", Value::Real(f64::from(green) / 255.0)),
            ("Blue Component", Value::Real(f64::from(blue) / 255.0)),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect(),
    )
}
