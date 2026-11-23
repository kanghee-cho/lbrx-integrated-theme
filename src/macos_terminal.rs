use crate::resolve::{ResolvedMode, ResolvedTheme, parse_rgb};
use plist::{Dictionary, Uid, Value};

pub const DIRECTORY: &str = "macos-terminal";
pub const FILES: [&str; 3] = [
    "LBRX Light.terminal",
    "LBRX Dark.terminal",
    crate::build::GUIDE,
];
pub const COLOR_KEYS: &[&str] = &[
    "TextColor",
    "BackgroundColor",
    "CursorColor",
    "SelectionColor",
    "TextBoldColor",
];
const ROLES: &[&str] = &[
    "terminal_foreground",
    "terminal_background",
    "terminal_cursor",
    "selection_background",
];

pub fn render(theme: &ResolvedTheme) -> Result<[(&'static str, String); 3], String> {
    Ok([
        (FILES[0], encode(theme, &theme.light, "LBRX Light")?),
        (FILES[1], encode(theme, &theme.dark, "LBRX Dark")?),
        (
            FILES[2],
            include_str!("../Docs/14-macos-terminal.md").into(),
        ),
    ])
}

fn encode(theme: &ResolvedTheme, mode: &ResolvedMode, name: &str) -> Result<String, String> {
    let mut profile = dictionary([
        ("name", Value::String(name.into())),
        ("type", Value::String("Window Settings".into())),
        ("ProfileCurrentVersion", Value::Real(2.04)),
    ]);
    let mut insert = |key: &str, hex: &str| -> Result<(), String> {
        let value =
            archived_color(hex).map_err(|err| format!("{DIRECTORY}: {name}: {key}: {err}"))?;
        profile.insert(key.into(), value);
        Ok(())
    };
    for (&key, &role) in COLOR_KEYS.iter().zip(ROLES) {
        insert(
            key,
            theme.editor_color(mode, DIRECTORY, key, &mode.roles[role]),
        )?;
    }
    if let Some(role) = theme.editor_role(DIRECTORY, "TextBoldColor") {
        insert("TextBoldColor", &mode.roles[role])?;
    }
    for (slot, hex) in mode.ansi.iter().enumerate() {
        let names = [
            "Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan", "White",
        ];
        let brightness = if slot < 8 { "" } else { "Bright" };
        insert(&format!("ANSI{brightness}{}Color", names[slot % 8]), hex)?;
    }
    let mut xml = Vec::new();
    Value::Dictionary(profile)
        .to_writer_xml(&mut xml)
        .map_err(|err| format!("{DIRECTORY}: {name}: {err}"))?;
    String::from_utf8(xml).map_err(|err| format!("{DIRECTORY}: {name}: {err}"))
}

fn archived_color(hex: &str) -> Result<Value, plist::Error> {
    let [r, g, b] = parse_rgb(hex)
        .expect("resolved color")
        .map(|x| f64::from(x) / 255.0);
    let objects = vec![
        Value::String("$null".into()),
        Value::Dictionary(dictionary([
            ("NSRGB", Value::Data(format!("{r} {g} {b}\0").into_bytes())),
            ("NSColorSpace", Value::Integer(2.into())),
            ("$class", Value::Uid(Uid::new(2))),
        ])),
        Value::Dictionary(dictionary([
            ("$classname", Value::String("NSColor".into())),
            (
                "$classes",
                Value::Array(vec![
                    Value::String("NSColor".into()),
                    Value::String("NSObject".into()),
                ]),
            ),
        ])),
    ];
    let mut data = Vec::new();
    Value::Dictionary(dictionary([
        ("$archiver", Value::String("NSKeyedArchiver".into())),
        ("$version", Value::Integer(100000.into())),
        ("$objects", Value::Array(objects)),
        (
            "$top",
            Value::Dictionary(dictionary([("root", Value::Uid(Uid::new(1)))])),
        ),
    ]))
    .to_writer_binary(&mut data)?;
    Ok(Value::Data(data))
}

fn dictionary<const N: usize>(entries: [(&str, Value); N]) -> Dictionary {
    entries
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect()
}
