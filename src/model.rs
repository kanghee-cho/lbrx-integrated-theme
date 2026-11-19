use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub schema_version: u32,
    pub light: Mode,
    pub dark: Mode,
    pub typography: Typography,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mode {
    pub palette: BTreeMap<String, String>,
    pub roles: BTreeMap<String, String>,
    pub ansi: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Typography {
    pub prose: FontStyle,
    pub mono: FontStyle,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FontStyle {
    pub families: Vec<String>,
    pub size_px: f64,
    pub line_height: f64,
}
