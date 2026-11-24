use crate::{
    css::font_stack,
    layers::{
        DOCUMENT_ALERT_ROLES, DocumentListMarkers, TYPORA_EDITING_AID_COLORS,
        TYPORA_SIDEBAR_COLORS, TYPORA_SOURCE_COLORS, TyporaDiagrams, TyporaPrint, TyporaSidebar,
        TyporaSourceFont, TyporaToc,
    },
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
    if let Some(role) = theme.editor_role(DIRECTORY, "--blur-text-color") {
        writeln!(css, "  --blur-text-color: {};", mode.roles[role]).unwrap();
    }
    if let Some(diagrams) = &theme.layers.typora_diagrams {
        write_diagram_variables(&mut css, diagrams, dark);
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
    if let Some(diagrams) = &theme.layers.typora_diagrams {
        write_diagram_alignment(&mut css, diagrams);
    }
    for (key, suffix, property) in [
        (
            "--lbrx-code-selection-background",
            "",
            "--select-text-bg-color",
        ),
        ("--lbrx-code-selection-text", "", "--select-text-font-color"),
        (
            "--lbrx-code-active-line-background",
            " .CodeMirror-activeline-background",
            "background-color",
        ),
        (
            "--lbrx-code-matching-bracket",
            " .CodeMirror-matchingbracket",
            "color",
        ),
        (
            "--lbrx-code-nonmatching-bracket",
            " .CodeMirror-nonmatchingbracket",
            "color",
        ),
    ] {
        if let Some(role) = theme.editor_role(DIRECTORY, key) {
            writeln!(
                css,
                "#write .md-fences .CodeMirror{suffix} {{ {property}: {}; }}",
                mode.roles[role]
            )
            .unwrap();
        }
    }
    if let Some(source) = &theme.layers.typora_source_mode {
        let editor = "#typora-source .CodeMirror.cm-s-typora-default";
        if let Some(font) = source.font {
            let name = match font {
                TyporaSourceFont::Prose => "prose",
                TyporaSourceFont::Mono => "mono",
            };
            writeln!(
                css,
                "{editor} {{ font-family: var(--lbrx-{name}-family); --monospace: var(--lbrx-{name}-family); }}"
            )
            .unwrap();
        }
        if let Some(size) = source.size_px {
            writeln!(css, "{editor} {{ font-size: {size}px; }}").unwrap();
        }
        for &(key, selector, property) in TYPORA_SOURCE_COLORS {
            if let Some(role) = source.colors.get(key) {
                writeln!(
                    css,
                    "{editor}{selector} {{ {property}: {}; }}",
                    mode.roles[role]
                )
                .unwrap();
            }
        }
    }
    let live_link = r#"#write [md-inline="link"]:is(.md-expand, .md-expand-on-selection)"#;
    if let Some(role) = theme.editor_role(DIRECTORY, "--lbrx-live-edit-marker") {
        writeln!(
            css,
            "#write [md-inline]:is(.md-expand, .md-expand-on-selection):is([md-inline=\"strong\"], [md-inline=\"em\"], [md-inline=\"del\"], [md-inline=\"highlight\"], [md-inline=\"link\"]) > .md-meta {{ color: {}; }}\n\
             {live_link} > .md-meta > :is(.md-url, .md-link-title) {{ color: var(--md-char-color, #BBB); }}",
            mode.roles[role]
        )
        .unwrap();
    }
    if let Some(role) = theme.editor_role(DIRECTORY, "--lbrx-live-edit-link-text") {
        writeln!(css, "{live_link} > a {{ color: {}; }}", mode.roles[role]).unwrap();
    }
    if let Some(role) = theme.editor_role(DIRECTORY, "--lbrx-live-edit-link-url") {
        writeln!(
            css,
            "{live_link} > .md-meta > .md-url {{ color: {}; }}",
            mode.roles[role]
        )
        .unwrap();
    }
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
        let defaults = DocumentListMarkers::default();
        let markers = lists.markers.as_ref().unwrap_or(&defaults).styles();
        for (ancestors, marker) in ["", ":is(ol, ul) ", ":is(ol, ul) :is(ol, ul) "]
            .into_iter()
            .zip(markers)
        {
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
            mode.roles[tasks.checkbox_color.as_deref().unwrap_or("accent")]
        )
        .unwrap();
    }
    if let Some(images) = &theme.layers.typora_images {
        let margins = images
            .alignment
            .map_or("auto", |alignment| alignment.margins());
        writeln!(
            css,
            "#write figure > img,\n\
             #write figure > :is(a, picture) > img,\n\
             #write p > .md-image-multi-line:only-child:not(.md-img-error) > img {{\n  display: block;\n  max-inline-size: 100%;\n  block-size: auto;\n  margin-inline: {margins};\n}}\n\
             #write figure:has(> img, > :is(a, picture) > img) > figcaption {{\n  margin-block-start: {}px;\n  font-size: {}%;\n  color: var(--lbrx-muted);\n  text-align: center;\n}}",
            images.caption_gap_px, images.caption_size_percent
        )
        .unwrap();
        if images.alignment.is_some() {
            writeln!(
                css,
                "#write p > .md-image:only-child:not(.md-img-error) img,\n\
                 #write p > img:only-child {{ margin-inline: {margins}; }}\n\
                 #write.first-line-indent p > .md-image:only-child:not(.md-img-error) img {{ left: auto; }}"
            )
            .unwrap();
        }
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
        for (kind, default_role) in DOCUMENT_ALERT_ROLES {
            let role = alerts.role(kind, default_role);
            let color = &mode.roles[role];
            writeln!(
                css,
                "#write .md-alert.md-alert-{kind} {{ border-left-color: {color}; }}\n\
                 #write .md-alert .md-alert-text-{kind} {{ color: {color}; }}"
            )
            .unwrap();
        }
    }
    if let Some(toc) = &theme.layers.typora_toc {
        write_toc(&mut css, toc, mode);
    }
    if let Some(sidebar) = &theme.layers.typora_sidebar {
        write_sidebar(&mut css, sidebar, mode);
    }
    if let Some(aids) = &theme.layers.typora_editing_aids {
        for (group, colors) in aids.groups() {
            for &(key, targets, property) in TYPORA_EDITING_AID_COLORS {
                if let Some(role) = key
                    .strip_prefix(group)
                    .and_then(|name| name.strip_prefix('.'))
                    .and_then(|name| colors.get(name))
                {
                    // Keep legacy wrappers separate from the custom highlight pseudo-elements.
                    for target in targets {
                        writeln!(css, "{target} {{ {property}: {}; }}", mode.roles[role]).unwrap();
                    }
                }
            }
        }
    }
    for lang in theme.language_ids() {
        for token in [
            "comment", "keyword", "string", "function", "number", "error",
        ] {
            if !theme.has_token_override(lang, token) {
                continue;
            }
            for label in std::iter::once(lang).chain(
                crate::layers::language_labels(lang)
                    .iter()
                    .copied()
                    .filter(|&s| s != lang),
            ) {
                writeln!(
                    css,
                    "#write .md-fences[lang=\"{label}\"] {{ --lbrx-{token}: {}; }}",
                    theme.token(mode, lang, token)
                )
                .unwrap();
            }
        }
    }
    write_code_mirror_tokens(&mut css, theme, mode);
    if dark {
        css.push_str(include_str!("typora-preferences-dark.css"));
    } else {
        css.push_str(include_str!("typora-preferences-light.css"));
    }
    if let Some(print) = &theme.layers.typora_print {
        write_print(&mut css, print, &theme.light);
    }
    css
}

fn write_code_mirror_tokens(css: &mut String, theme: &ResolvedTheme, mode: &ResolvedMode) {
    for (selector, token, variable) in [
        (".cm-comment", "comment", true),
        (".cm-keyword", "keyword", true),
        (".cm-string", "string", true),
        (".cm-def", "function", true),
        (".cm-number", "number", true),
        (".cm-atom", "constant.builtin", false),
        (".cm-variable", "variable", false),
        (".cm-property", "property", false),
        (".cm-operator", "operator", false),
        (".cm-attribute", "attribute", false),
        (".cm-meta", "attribute", false),
        (".cm-type", "type", false),
    ] {
        let color = if variable {
            format!("var(--lbrx-{token})")
        } else {
            theme.token(mode, "", token).to_owned()
        };
        writeln!(
            css,
            "#write .md-fences .cm-s-inner {selector} {{ color: {color}; {} }}",
            theme.token_style("", token).css()
        )
        .unwrap();
        for lang in theme
            .language_ids()
            .filter(|lang| theme.has_token_override(lang, token))
        {
            for label in std::iter::once(lang).chain(
                crate::layers::language_labels(lang)
                    .iter()
                    .copied()
                    .filter(|&s| s != lang),
            ) {
                writeln!(css, "#write .md-fences[lang=\"{label}\"] .cm-s-inner {selector} {{ color: {}; {} }}", theme.token(mode, lang, token), theme.token_style(lang, token).css()).unwrap();
            }
        }
    }
    for (lang, selector, token) in [
        ("javascript", ".cm-string-2", "string.escape"),
        ("typescript", ".cm-string-2", "string.escape"),
        ("toml", ".cm-header", "type"),
        ("html", ".cm-tag", "keyword.declaration"),
        ("xml", ".cm-tag", "keyword.declaration"),
    ] {
        if theme.layers.languages.contains_key(lang) {
            for label in std::iter::once(lang).chain(
                crate::layers::language_labels(lang)
                    .iter()
                    .copied()
                    .filter(|&s| s != lang),
            ) {
                writeln!(css, "#write .md-fences[lang=\"{label}\"] .cm-s-inner {selector} {{ color: {}; {} }}", theme.token(mode, lang, token), theme.token_style(lang, token).css()).unwrap();
            }
        }
    }
}

fn write_print(css: &mut String, print: &TyporaPrint, light: &ResolvedMode) {
    let mut rules = String::new();
    if let Some(color) = &print.paper_color {
        writeln!(
            rules,
            "  html, body, content, #write {{ background-color: {color}; }}"
        )
        .unwrap();
    }
    if let Some(role) = &print.text_color {
        writeln!(
            rules,
            "  body, content, #write, #write :is(h1, h2, h3, h4, h5, h6) {{ color: {}; }}",
            light.roles[role]
        )
        .unwrap();
    }
    if let Some(role) = &print.link_color {
        writeln!(rules, "  #write a {{ color: {}; }}", light.roles[role]).unwrap();
    }
    if print.keep_heading_with_next {
        rules.push_str(
            "  #write > :is(h1, h2, h3, h4, h5, h6):not(:last-child) { break-after: avoid-page; }\n",
        );
    }
    if print.keep_figure_with_caption {
        rules.push_str(
            "  #write figure:not(.table-figure):has(> img, > :is(a, picture) > img):has(> figcaption) { break-inside: avoid-page; }\n",
        );
    }
    if !rules.is_empty() {
        writeln!(css, "\n@media print {{\n{rules}}}").unwrap();
    }
}

fn write_diagram_variables(css: &mut String, diagrams: &TyporaDiagrams, dark: bool) {
    if let Some(mermaid) = &diagrams.mermaid {
        let preset = if dark {
            mermaid.dark_theme
        } else {
            mermaid.light_theme
        };
        if let Some(preset) = preset {
            writeln!(css, "  --mermaid-theme: {};", preset.css_value()).unwrap();
        }
        if let Some(numbered) = mermaid.sequence_numbers {
            let value = if numbered { "on" } else { "off" };
            writeln!(css, "  --mermaid-sequence-numbers: {value};").unwrap();
        }
        if let Some(curve) = mermaid.flowchart_curve {
            writeln!(css, "  --mermaid-flowchart-curve: {};", curve.css_value()).unwrap();
        }
        if let Some(padding) = mermaid.gantt_left_padding_px {
            writeln!(css, "  --mermaid--gantt-left-padding: {padding};").unwrap();
        }
    }
    if let Some(preset) = diagrams
        .sequence
        .as_ref()
        .and_then(|sequence| sequence.theme)
    {
        writeln!(css, "  --sequence-theme: {};", preset.css_value()).unwrap();
    }
}

fn write_diagram_alignment(css: &mut String, diagrams: &TyporaDiagrams) {
    for (language, alignment) in [
        (
            "mermaid",
            diagrams.mermaid.as_ref().and_then(|value| value.alignment),
        ),
        (
            "sequence",
            diagrams.sequence.as_ref().and_then(|value| value.alignment),
        ),
        (
            "flow",
            diagrams.flow.as_ref().and_then(|value| value.alignment),
        ),
    ] {
        if let Some(alignment) = alignment {
            let preview =
                format!("#write .md-fences[lang=\"{language}\"] .md-diagram-panel-preview");
            writeln!(
                css,
                "{preview} {{ text-align: {}; }}\n\
                 {preview} > svg {{ display: block; margin-inline: {}; }}",
                alignment.css_value(),
                alignment.margins()
            )
            .unwrap();
        }
    }
}

fn write_sidebar(css: &mut String, sidebar: &TyporaSidebar, mode: &ResolvedMode) {
    for &(key, suffix, property) in TYPORA_SIDEBAR_COLORS {
        if let Some(role) = sidebar.color(key) {
            writeln!(
                css,
                "#typora-sidebar{suffix} {{ {property}: {}; }}",
                mode.roles[role]
            )
            .unwrap();
        }
    }
    if let Some(outline) = &sidebar.outline {
        let panel = "#typora-sidebar #outline-content";
        if let Some(step) = outline.indent_step_px {
            for level in 1..=6 {
                let pixels = u32::from(step) * (level - 1);
                writeln!(
                    css,
                    "{panel} .outline-h{level} > .outline-item {{ padding-left: 0; padding-inline-start: {pixels}px; }}"
                )
                .unwrap();
            }
        }
        if outline.numbered == Some(true) {
            writeln!(
                css,
                "{panel} {{ counter-reset: lbrx-outline; }}\n\
                 {panel} .outline-item {{ counter-increment: lbrx-outline; }}\n\
                 {panel} .outline-item > .outline-label::before {{ content: counter(lbrx-outline) \". \"; }}"
            )
            .unwrap();
        }
    }
}

fn write_toc(css: &mut String, toc: &TyporaToc, mode: &ResolvedMode) {
    let content = "#write .md-toc > .md-toc-content";
    let item = format!("{content} > .md-toc-item");
    let link = format!("{item} > a.md-toc-inner");

    if let Some(depth) = toc.max_depth.filter(|&depth| depth < 6) {
        let hidden = (depth + 1..=6)
            .map(|level| format!(".md-toc-h{level}"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(css, "{item}:is({hidden}) {{ display: none; }}").unwrap();
    }
    if let Some(gap) = toc.item_gap_px {
        writeln!(
            css,
            "{content}:not(:empty) {{ display: grid; row-gap: {gap}px; }}"
        )
        .unwrap();
    }
    for (&level, &indent) in &toc.indent_px {
        writeln!(
            css,
            "{item}.md-toc-h{} > a.md-toc-inner {{ margin-left: 0; margin-inline-start: {indent}px; }}",
            level as u8
        )
        .unwrap();
    }
    if let Some(role) = &toc.link_color {
        writeln!(css, "{link} {{ color: {}; }}", mode.roles[role]).unwrap();
    }
    if let Some(role) = &toc.hover_color {
        writeln!(
            css,
            "{link}:is(:hover, :focus-visible) {{ color: {}; }}",
            mode.roles[role]
        )
        .unwrap();
    }
    if let Some(underline) = toc.underline {
        let decoration = if underline { "underline" } else { "none" };
        writeln!(css, "{link} {{ text-decoration: {decoration}; }}").unwrap();
    }
    if toc.numbered == Some(true) {
        writeln!(
            css,
            "{content} {{ counter-reset: lbrx-toc; }}\n\
             {item} {{ counter-increment: lbrx-toc; }}\n\
             {link}::before {{ content: counter(lbrx-toc) \". \"; }}"
        )
        .unwrap();
    }
}
