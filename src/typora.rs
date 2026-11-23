use crate::{
    css::font_stack,
    resolve::{ResolvedMode, ResolvedTheme},
};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "typora";
pub const LIGHT: &str = "lbrx-light.css";
pub const DARK: &str = "lbrx-dark.css";
pub const FILES: &[&str] = &[LIGHT, DARK, "README.md"];

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 3] {
    [
        (LIGHT, stylesheet(theme, &theme.light, false)),
        (DARK, stylesheet(theme, &theme.dark, true)),
        (
            "README.md",
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Docs/07-typora.md")).into(),
        ),
    ]
}

fn stylesheet(theme: &ResolvedTheme, mode: &ResolvedMode, dark: bool) -> String {
    let typography = &theme.typography;
    let mut css = String::from("/* LBRX — generated from the theme source. */\n:root {\n");
    writeln!(
        css,
        "  --lbrx-prose-selection-background: {};",
        mode.prose_selection_background()
    )
    .unwrap();
    writeln!(
        css,
        "  --lbrx-code-block-background: {};",
        mode.code_block_background()
    )
    .unwrap();
    for role in [
        "background",
        "text",
        "heading",
        "link",
        "quote",
        "inline_code",
        "muted",
        "focus",
        "selection_background",
        "selection_text",
        "comment",
        "keyword",
        "string",
        "function",
        "number",
        "error",
    ] {
        let key = format!("--lbrx-{}", role.replace('_', "-"));
        let fg = theme.token(mode, "", role);
        writeln!(
            css,
            "  {key}: {};",
            theme.editor_color(mode, DIRECTORY, &key, fg)
        )
        .unwrap();
    }
    if dark
        || theme.layers.typora_table.is_some()
        || theme
            .layers
            .typora_math
            .as_ref()
            .is_some_and(|math| math.shaded_background)
        || theme
            .layers
            .typora_alerts
            .as_ref()
            .is_some_and(|alerts| alerts.shaded_background)
    {
        writeln!(css, "  --lbrx-surface: {};", mode.roles["surface"]).unwrap();
    }
    css.push_str("  --bg-color: var(--lbrx-background);\n  --text-color: var(--lbrx-text);\n");
    for (name, font) in [("prose", &typography.prose), ("mono", &typography.mono)] {
        let stack = font_stack(font);
        writeln!(css, "  --lbrx-{name}-family: {stack};").unwrap();
        writeln!(css, "  --lbrx-{name}-size: {}px;", font.size_px).unwrap();
        writeln!(css, "  --lbrx-{name}-leading: {};", font.line_height).unwrap();
    }
    css.push_str("}\n");
    css.push_str(include_str!("typora.css"));
    if let Some(layout) = &theme.layers.typora_layout {
        writeln!(
            css,
            "#write {{\n  box-sizing: border-box;\n  max-width: min(100%, {}px);\n  padding-inline: clamp({}px, 4vw, {}px);\n  margin-inline: auto;\n}}",
            layout.max_width_px, layout.padding_inline_min_px, layout.padding_inline_max_px
        )
        .unwrap();
    }
    if let Some(paragraph) = &theme.layers.typora_paragraph {
        if let Some(gap_px) = paragraph.gap_px {
            writeln!(
                css,
                "#write > p:has(+ p) {{ margin-block-end: 0; }}\n#write > p + p {{ margin-block-start: {gap_px}px; }}"
            )
            .unwrap();
        }
        if let Some(alignment) = paragraph.alignment {
            writeln!(
                css,
                "#write > p {{ text-align: {}; }}",
                alignment.css_value()
            )
            .unwrap();
        }
    }
    if let Some(headings) = &theme.layers.typora_headings {
        for (level, style) in headings.levels() {
            let divider = if style.divider {
                "1px solid var(--lbrx-muted)"
            } else {
                "none"
            };
            writeln!(
                css,
                "#write {level} {{\n  font-size: {}%;\n  font-weight: {};\n  margin-block: {}px {}px;\n  border-block-end: {divider};\n}}",
                style.size_percent, style.weight, style.margin_before_px, style.margin_after_px
            )
            .unwrap();
        }
    }
    if let Some(emphasis) = &theme.layers.typora_emphasis {
        let colors = if dark {
            &emphasis.dark
        } else {
            &emphasis.light
        };
        let outside_code = ":not(:is(code, pre, .md-fences, .CodeMirror) *)";
        for (elements, properties) in [
            (
                ":is(strong, b)",
                format!("font-weight: 700; color: {};", colors.strong),
            ),
            (
                ":is(em, i)",
                format!("font-style: italic; color: {};", colors.italic),
            ),
            (
                ":is(del, s)",
                format!(
                    "text-decoration-line: line-through; color: {};",
                    colors.strikethrough
                ),
            ),
            (
                "mark",
                format!(
                    "background-color: {}; color: inherit;",
                    colors.highlight_background
                ),
            ),
        ] {
            writeln!(css, "#write {elements}{outside_code} {{ {properties} }}").unwrap();
        }
    }
    if let Some(footnotes) = &theme.layers.typora_footnotes {
        let reference = theme.editor_color(
            mode,
            DIRECTORY,
            "--lbrx-footnote-reference",
            &mode.roles["function"],
        );
        writeln!(
            css,
            "#write sup.md-footnote {{ color: {reference}; background-color: transparent; }}\n\
             #write sup.md-footnote a {{ color: inherit; }}\n\
             #write .footnotes {{ font-size: 100%; opacity: 1; }}\n\
             #write :is(.md-def-footnote, .footnote-line) {{ font-size: {}%; color: var(--lbrx-muted); }}",
            footnotes.body_size_percent
        )
        .unwrap();
    }
    if let Some(rule) = &theme.layers.typora_horizontal_rule {
        writeln!(
            css,
            "#write hr:not(.footnotes-area hr) {{\n  height: 0;\n  padding: 0;\n  border: 0;\n  border-block-start: {}px solid var(--lbrx-muted);\n  background: none;\n  margin-block: {}px;\n}}",
            rule.thickness_px, rule.margin_block_px
        )
        .unwrap();
    }
    if let Some(table) = &theme.layers.typora_table {
        writeln!(
            css,
            "#write figure.table-figure > table :is(th, td) {{\n  padding: {}px {}px;\n  border: 0;\n  border-block-end: {}px solid var(--lbrx-muted);\n}}\n\
             #write figure.table-figure > table > thead th {{\n  background-color: var(--lbrx-surface);\n  font-weight: 700;\n}}\n\
             @media screen {{\n  #write figure.table-figure {{ max-width: 100%; overflow-x: auto; }}\n}}",
            table.cell_padding_block_px, table.cell_padding_inline_px, table.row_rule_px
        )
        .unwrap();
    }
    if let Some(lists) = &theme.layers.typora_lists {
        let list = ":is(ol, ul)";
        let prose = ":not(:where(.md-toc, .footnotes-area, .md-fences) *)";
        let item = "li:not(:where(.md-task-list-item, .task-list-item))";
        writeln!(
            css,
            "#write {list}{prose}:has(> {item}) {{ padding-inline-start: {}px; }}\n\
             #write {list}{prose} > {item} + {item} {{ margin-block-start: {}px; }}",
            lists.indent_px, lists.item_gap_px
        )
        .unwrap();
        for (ancestors, marker) in [
            ("", "disc"),
            (":is(ol, ul) ", "circle"),
            (":is(ol, ul) :is(ol, ul) ", "square"),
        ] {
            writeln!(
                css,
                "#write {ancestors}ul{prose} > {item} {{ list-style-type: {marker}; }}"
            )
            .unwrap();
        }
    }
    if let Some(tasks) = &theme.layers.typora_task_lists {
        let task = "#write li:is(.md-task-list-item, .task-list-item)";
        let checkbox = r#"input[type="checkbox"]"#;
        let scheme = if dark { "dark" } else { "light" };
        let strike = if tasks.strike_completed {
            " text-decoration-line: line-through;"
        } else {
            ""
        };
        writeln!(
            css,
            "{task} > {checkbox} {{ color-scheme: {scheme}; accent-color: {}; }}\n\
             {task}:has(> {checkbox}:checked) > p {{ color: var(--lbrx-muted);{strike} }}",
            mode.roles["accent"]
        )
        .unwrap();
    }
    if let Some(images) = &theme.layers.typora_images {
        writeln!(
            css,
            "#write figure > img,\n\
             #write figure > :is(a, picture) > img,\n\
             #write p > .md-image-multi-line:only-child:not(.md-img-error) > img {{\n  display: block;\n  max-inline-size: 100%;\n  block-size: auto;\n  margin-inline: auto;\n}}\n\
             #write figure:has(> img, > :is(a, picture) > img) > figcaption {{\n  margin-block-start: {}px;\n  font-size: {}%;\n  color: var(--lbrx-muted);\n  text-align: center;\n}}",
            images.caption_gap_px, images.caption_size_percent
        )
        .unwrap();
    }
    if let Some(math) = &theme.layers.typora_math {
        writeln!(
            css,
            "#write .md-math-block {{\n  box-sizing: border-box;\n  max-inline-size: 100%;\n  padding-block: {}px;\n  padding-inline: {}px;\n}}",
            math.padding_block_px, math.padding_inline_px
        )
        .unwrap();
        if math.shaded_background {
            css.push_str("#write .md-math-block { background-color: var(--lbrx-surface); }\n");
        }
        css.push_str(
            "@media screen {\n  #write .md-math-block > .md-math-container { max-inline-size: 100%; overflow-x: auto; }\n}\n",
        );
    }
    if let Some(alerts) = &theme.layers.typora_alerts {
        if alerts.shaded_background {
            css.push_str("#write .md-alert { background-color: var(--lbrx-surface); }\n");
        }
        for (kind, role) in [
            ("note", "function"),
            ("tip", "success"),
            ("important", "keyword"),
            ("warning", "warning"),
            ("caution", "error"),
        ] {
            let color = &mode.roles[role];
            writeln!(
                css,
                "#write .md-alert.md-alert-{kind} {{ border-left-color: {color}; }}\n\
                 #write .md-alert .md-alert-text-{kind} {{ color: {color}; }}"
            )
            .unwrap();
        }
    }
    for (lang, tokens) in &theme.layers.languages {
        for token in tokens.keys() {
            writeln!(
                css,
                "#write .md-fences[lang=\"{lang}\"] {{ --lbrx-{token}: {}; }}",
                theme.token(mode, lang, token)
            )
            .unwrap();
        }
    }
    if dark {
        css.push_str(include_str!("typora-preferences-dark.css"));
    } else {
        css.push_str(include_str!("typora-preferences-light.css"));
    }
    css
}
