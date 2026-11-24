use crate::{
    css::font_stack,
    layers::{
        DOCUMENT_ALERT_ROLES, DocumentAlerts, DocumentImages, DocumentLayout, DocumentLists,
        ListMarker, ObsidianBases, ObsidianCanvas, ObsidianDialogs, ObsidianIcons, ObsidianMermaid,
        ObsidianMobile, ObsidianNavigationSidebar, ObsidianPrint, ObsidianSearch,
    },
    resolve::{ResolvedMode, ResolvedTheme, parse_rgb},
};
use std::fmt::Write as _;

pub const DIRECTORY: &str = "obsidian/LBRX";
pub const FILES: &[&str] = &["manifest.json", "theme.css", crate::build::GUIDE];
pub const INLINE_CODE_COLOR: &str = "--lbrx-inline-code-color";
pub const FOOTNOTE_REFERENCE_COLOR: &str = "--lbrx-footnote-reference";

pub fn render(theme: &ResolvedTheme) -> [(&'static str, String); 3] {
    let manifest = serde_json::json!({
        "name": "LBRX",
        "version": env!("CARGO_PKG_VERSION"),
        "minAppVersion": "1.10.6",
        "author": "LBRX"
    });
    let mut css = String::new();
    for (class, mode) in [("light", &theme.light), ("dark", &theme.dark)] {
        writeln!(css, ".theme-{class} {{").unwrap();
        let [hue, saturation, lightness] =
            rgb_to_hsl(parse_rgb(&mode.roles["accent"]).expect("resolved accent is valid RGB"));
        writeln!(css, "  --accent-h: {hue:.8};").unwrap();
        writeln!(css, "  --accent-s: {saturation:.8}%;").unwrap();
        writeln!(css, "  --accent-l: {lightness:.8}%;").unwrap();
        for (variable, role) in [
            ("background-primary", "background"),
            ("background-secondary", "surface"),
            ("text-normal", "text"),
            ("text-muted", "muted"),
            ("link-color", "link"),
            ("link-external-color", "link"),
            ("h1-color", "heading"),
            ("h2-color", "heading"),
            ("h3-color", "heading"),
            ("h4-color", "heading"),
            ("h5-color", "heading"),
            ("h6-color", "heading"),
            ("text-selection", "selection_background"),
            ("lbrx-selection-text", "selection_text"),
            ("code-background", "background"),
            ("code-normal", "text"),
            ("code-comment", "comment"),
            ("code-function", "function"),
            ("code-keyword", "keyword"),
            ("code-string", "string"),
        ] {
            let fg = theme.token(mode, "", role);
            let value = if variable == "link-color" {
                fg
            } else {
                theme.editor_color(mode, "obsidian", &format!("--{variable}"), fg)
            };
            writeln!(css, "  --{variable}: {};", value).unwrap();
        }
        if let Some(role) = theme.editor_role("obsidian", "--inline-title-color") {
            writeln!(css, "  --inline-title-color: {};", mode.roles[role]).unwrap();
        }
        writeln!(
            css,
            "  --lbrx-code-block-background: {};",
            mode.code_block_background()
        )
        .unwrap();
        writeln!(
            css,
            "  --lbrx-prose-selection-background: {};",
            mode.prose_selection_background()
        )
        .unwrap();
        css.push_str("}\n");
        if let Some(settings) = &theme.layers.obsidian_navigation_sidebar {
            navigation_sidebar_styles(&mut css, class, mode, settings);
        }
        if let Some(dialogs) = &theme.layers.obsidian_dialogs {
            dialog_styles(&mut css, class, mode, dialogs);
        }
        if let Some(search) = &theme.layers.obsidian_search {
            search_styles(&mut css, class, mode, search);
        }
        if theme
            .layers
            .obsidian_graph
            .as_ref()
            .is_some_and(|graph| graph.use_theme_palette)
        {
            graph_styles(&mut css, class, mode);
        }
        if let Some(canvas) = &theme.layers.obsidian_canvas {
            canvas_styles(&mut css, class, mode, canvas);
        }
        if let Some(bases) = &theme.layers.obsidian_bases {
            bases_styles(&mut css, class, mode, bases);
        }
        if let Some(icons) = &theme.layers.obsidian_icons {
            icon_styles(&mut css, class, mode, icons);
        }
        if theme
            .layers
            .obsidian_tags_and_properties
            .as_ref()
            .is_some_and(|settings| settings.style_properties)
        {
            writeln!(
                css,
                ".theme-{class} :is(.markdown-reading-view .markdown-preview-view, .markdown-source-view.is-live-preview) .metadata-container {{\n  --metadata-label-text-color: var(--text-muted);\n  --metadata-property-box-shadow-focus: 0 0 0 2px {};\n}}",
                mode.roles["focus"]
            )
            .unwrap();
        }
        if theme
            .layers
            .obsidian_live_preview
            .as_ref()
            .is_some_and(|settings| settings.use_focus_caret)
        {
            let editor = format!(".theme-{class} .markdown-source-view.is-live-preview .cm-editor");
            let color = &mode.roles["focus"];
            writeln!(
                css,
                "{editor} .cm-content {{ caret-color: {color}; }}\n\
                 {editor} .cm-cursor {{ --caret-color: {color}; }}"
            )
            .unwrap();
        }
        link_styles(&mut css, class, theme, mode);
        blockquote_styles(&mut css, class, theme, mode);
        footnote_and_rule_styles(&mut css, class, theme, mode);
        if let Some(callouts) = &theme.layers.obsidian_callouts {
            callout_styles(&mut css, class, mode, callouts);
        }
        if let Some(tasks) = &theme.layers.obsidian_task_lists {
            let role = tasks.checkbox_color.as_deref().unwrap_or("accent");
            let task = "li.task-list-item[data-task=\"x\" i]";
            writeln!(
                css,
                ".theme-{class} .markdown-reading-view .markdown-rendered {task} > input.task-list-item-checkbox:checked,\n\
                 .theme-{class} .markdown-reading-view .markdown-rendered {task} > p > input.task-list-item-checkbox:checked {{ --checkbox-color: {}; }}",
                mode.roles[role]
            )
            .unwrap();
        }
        if let Some(emphasis) = &theme.layers.obsidian_emphasis {
            emphasis_styles(
                &mut css,
                class,
                if class == "dark" {
                    &emphasis.dark
                } else {
                    &emphasis.light
                },
            );
        }
        if let Some(role) = theme.editor_role("obsidian", INLINE_CODE_COLOR) {
            writeln!(
                css,
                concat!(
                    ".theme-{0} .markdown-reading-view .markdown-rendered code:not(pre *),\n",
                    ".theme-{0} .markdown-source-view.is-live-preview .markdown-rendered code:not(pre *),\n",
                    ".theme-{0} .markdown-source-view.is-live-preview .cm-inline-code:not(.HyperMD-codeblock *) {{\n",
                    "  color: {1};\n}}"
                ),
                class, mode.roles[role]
            )
            .unwrap();
        }
        for token in ["number", "error"] {
            writeln!(
                css,
                ".theme-{class} .markdown-source-view.is-live-preview .HyperMD-codeblock:not(.HyperMD-codeblock-begin, .HyperMD-codeblock-end) span.cm-{token}:not(.cm-formatting) {{ color: {}; }}",
                theme.token(mode, "", token)
            )
            .unwrap();
        }
        for &(token, classes) in PRISM_TOKENS {
            let numeric_or_error = matches!(token, "number" | "error");
            let rendered = if numeric_or_error {
                ":is(.markdown-reading-view, .markdown-source-view.is-live-preview) .markdown-rendered"
            } else {
                ".markdown-rendered"
            };
            for native in classes.split_whitespace() {
                let color = match token {
                    "comment" => "var(--code-comment)",
                    "keyword" => "var(--code-keyword)",
                    "string" => "var(--code-string)",
                    "function" => "var(--code-function)",
                    _ => theme.token(mode, "", token),
                };
                if numeric_or_error {
                    writeln!(
                        css,
                        ".theme-{class} {rendered} pre code .token.{native} {{ color: {color}; }}"
                    )
                    .unwrap();
                } else {
                    writeln!(
                        css,
                        ".theme-{class} {rendered} pre code .token.{native} {{ color: {color}; {} }}",
                        theme.token_style("", token).css()
                    )
                    .unwrap();
                }
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
                        let code = if label == "c++" {
                            "code[class~=\"language-c++\"]".to_owned()
                        } else {
                            format!("code.language-{label}")
                        };
                        let code = if numeric_or_error {
                            format!("pre {code}")
                        } else {
                            code
                        };
                        writeln!(
                            css,
                            ".theme-{class} {rendered} {code} .token.{native} {{ color: {}; {} }}",
                            theme.token(mode, lang, token),
                            theme.token_style(lang, token).css()
                        )
                        .unwrap();
                    }
                }
            }
        }
    }

    const PRISM_TOKENS: &[(&str, &str)] = &[
        ("comment", "comment"),
        ("keyword", "keyword"),
        ("string", "string char"),
        ("function", "function"),
        ("number", "number"),
        ("error", "error"),
        ("constant.builtin", "boolean null"),
        ("string.escape", "regex escape"),
        ("type", "class-name"),
        ("variable", "variable"),
        ("property", "property literal-property"),
        ("operator", "operator"),
        ("attribute", "attr-name annotation decorator anchor alias"),
        ("punctuation", "punctuation"),
    ];
    css.push_str(concat!(
        ".markdown-rendered pre,\n",
        ".markdown-source-view.is-live-preview .HyperMD-codeblock {\n",
        "  --code-background: var(--lbrx-code-block-background);\n",
        "  background-color: var(--code-background);\n",
        "}\n"
    ));
    css.push_str(".theme-light, .theme-dark {\n");
    for (variable, style) in [
        ("font-text-theme", &theme.typography.prose),
        ("font-monospace-theme", &theme.typography.mono),
    ] {
        writeln!(css, "  --{variable}: {};", font_stack(style)).unwrap();
    }
    css.push_str(concat!(
        "}\n\n",
        ".markdown-rendered ::selection, .markdown-source-view ::selection {\n",
        "  background-color: var(--lbrx-prose-selection-background);\n",
        "  color: var(--lbrx-selection-text);\n",
        "}\n",
        ".markdown-rendered :is(pre, code)::selection,\n",
        ".markdown-rendered :is(pre, code) *::selection,\n",
        ".markdown-source-view :is(.HyperMD-codeblock, .cm-inline-code)::selection,\n",
        ".markdown-source-view :is(.HyperMD-codeblock, .cm-inline-code) *::selection {\n",
        "  background-color: var(--text-selection);\n",
        "  color: var(--lbrx-selection-text);\n",
        "}\n"
    ));
    if theme
        .layers
        .obsidian_prose
        .as_ref()
        .is_some_and(|prose| prose.use_shared_line_height)
    {
        writeln!(
            css,
            "\n.markdown-rendered p,\n\
             .markdown-source-view.is-live-preview .cm-line:not([class*=\"HyperMD-\"]) {{\n  \
             line-height: {};\n}}",
            theme.typography.prose.line_height
        )
        .unwrap();
    }
    if theme
        .layers
        .obsidian_live_preview
        .as_ref()
        .is_some_and(|settings| settings.highlight_active_line)
    {
        css.push_str(concat!(
            ".markdown-source-view.is-live-preview .cm-editor.cm-focused .cm-line:is(.cm-active, .cm-activeLine) {\n",
            "  --lbrx-active-line-tint: color-mix(in srgb, var(--background-secondary) 60%, transparent);\n",
            "  background-image: linear-gradient(var(--lbrx-active-line-tint), var(--lbrx-active-line-tint));\n",
            "}\n"
        ));
    }
    if theme
        .layers
        .obsidian_source_mode
        .as_ref()
        .is_some_and(|settings| settings.mute_markdown_markers)
    {
        css.push_str(concat!(
            ".markdown-source-view.mod-cm6:not(.is-live-preview) .cm-content ",
            ".cm-line:not(.HyperMD-codeblock, .HyperMD-frontmatter) span.cm-formatting {\n",
            "  color: var(--text-muted);\n",
            "}\n"
        ));
    }
    if theme
        .layers
        .obsidian_tags_and_properties
        .as_ref()
        .is_some_and(|settings| settings.style_inline_tags)
    {
        css.push_str(concat!(
            ":is(.markdown-reading-view, .markdown-source-view.is-live-preview) .markdown-rendered a.tag:not(.metadata-container *),\n",
            ".markdown-source-view.is-live-preview .cm-content .cm-line:not(.HyperMD-codeblock, .HyperMD-frontmatter) .cm-hashtag:not(.cm-inline-code) {\n",
            "  --tag-background: var(--background-secondary);\n",
            "  --tag-color: var(--interactive-accent);\n",
            "}\n"
        ));
    }
    if let Some(links) = &theme.layers.obsidian_internal_links {
        if links.distinguish_unresolved {
            css.push_str(concat!(
                ".markdown-reading-view .markdown-rendered a.internal-link.is-unresolved,\n",
                ".markdown-source-view.is-live-preview .markdown-rendered a.internal-link.is-unresolved,\n",
                ".markdown-source-view.is-live-preview :is(.is-unresolved.cm-hmd-internal-link, .is-unresolved:has(.cm-hmd-internal-link), .is-unresolved .cm-hmd-internal-link) {\n",
                "  --link-unresolved-color: var(--text-muted);\n",
                "  --link-unresolved-opacity: 1;\n",
                "  --link-unresolved-decoration-color: var(--text-muted);\n",
                "  --link-unresolved-decoration-style: dotted;\n",
                "  --link-decoration: underline;\n",
                "}\n"
            ));
        }
        if let Some(width) = links.note_embed_border_width_px {
            writeln!(
                css,
                ":is(.markdown-reading-view .markdown-rendered, .markdown-source-view.is-live-preview) .internal-embed.markdown-embed {{ --embed-border-start: {width}px solid var(--interactive-accent); }}"
            )
            .unwrap();
        }
    }
    if let Some(mermaid) = &theme.layers.obsidian_mermaid {
        mermaid_styles(&mut css, mermaid);
    }
    if let Some(workspace) = &theme.layers.obsidian_workspace {
        if workspace.style_active_tabs {
            css.push_str(concat!(
                ".workspace .workspace-tab-header-container .workspace-tab-header.is-active {\n",
                "  --tab-background-active: var(--background-secondary);\n",
                "  --tab-font-weight: 700;\n",
                "  background-color: var(--tab-background-active);\n",
                "  background-image: linear-gradient(to top, var(--interactive-accent) 2px, transparent 2px);\n",
                "}\n"
            ));
        }
        if workspace.shade_ribbon {
            css.push_str(
                ".workspace > .workspace-ribbon { --ribbon-background: var(--background-secondary); --ribbon-background-collapsed: var(--background-secondary); }\n",
            );
        }
        if workspace.shade_status_bar {
            css.push_str(
                ".app-container .status-bar { --status-bar-background: var(--background-secondary); }\n",
            );
        }
        if workspace.style_split_dividers {
            css.push_str(
                ".workspace .workspace-leaf-resize-handle { --divider-color: color-mix(in srgb, var(--text-muted) 24%, var(--background-secondary)); }\n",
            );
        }
    }
    if let Some(lists) = &theme.layers.obsidian_lists {
        list_styles(&mut css, lists);
    }
    if let Some(images) = &theme.layers.obsidian_images {
        image_styles(&mut css, images);
    }
    if let Some(math) = &theme.layers.obsidian_math {
        let block = ".markdown-reading-view .markdown-rendered .math-block";
        writeln!(
            css,
            "{block} {{ box-sizing: border-box; max-inline-size: 100%; padding-block: {}px; padding-inline: {}px; }}",
            math.padding_block_px, math.padding_inline_px
        )
        .unwrap();
        if math.shaded_background {
            writeln!(
                css,
                "{block} {{ background-color: var(--background-secondary); }}"
            )
            .unwrap();
        }
        writeln!(
            css,
            "@media screen {{\n  {block} > mjx-container {{ max-inline-size: 100%; overflow-x: auto; }}\n}}"
        )
        .unwrap();
    }
    if let Some(tasks) = &theme.layers.obsidian_task_lists {
        let task = ".markdown-reading-view .markdown-rendered li.task-list-item[data-task=\"x\" i]";
        let blocks = ":is(ul, ol, blockquote, pre, div, table, hr, dl, h1, h2, h3, h4, h5, h6)";
        let decoration = if tasks.strike_completed {
            "line-through"
        } else {
            "none"
        };
        // A decoration on the whole li would also paint across child lists.
        writeln!(
            css,
            "{task} {{ text-decoration: none; color: var(--text-normal); }}\n\
             {task}:not(:has(> p)) {{ color: var(--text-muted); }}\n\
             {task}:not(:has(> p)) > {blocks} {{ color: var(--text-normal); }}\n\
             {task} > p,\n\
             {task}:not(:has(> p, > {blocks})) {{ color: var(--text-muted); text-decoration-line: {decoration}; }}"
        )
        .unwrap();
    }
    if let Some(table) = &theme.layers.obsidian_table {
        let wrapper = ".markdown-reading-view .markdown-rendered .el-table";
        writeln!(
            css,
            "{wrapper} > table > :is(thead, tbody) > tr > :is(th, td) {{\n  padding: {}px {}px;\n  border: 0;\n  border-block-end: {}px solid var(--text-muted);\n}}\n\
             {wrapper} > table > thead > tr > th {{ background-color: var(--background-secondary); font-weight: 700; }}\n\
             @media screen {{\n  {wrapper}:has(> table) {{ max-width: 100%; overflow-x: auto; }}\n}}",
            table.cell_padding_block_px, table.cell_padding_inline_px, table.row_rule_px
        )
        .unwrap();
    }
    if let Some(code) = &theme.layers.obsidian_code {
        if code.use_shared_line_height {
            writeln!(
                css,
                "\n.markdown-rendered pre,\n\
                 .markdown-rendered pre > code,\n\
                 .markdown-source-view.is-live-preview .HyperMD-codeblock {{\n  \
                 line-height: {};\n}}",
                theme.typography.mono.line_height
            )
            .unwrap();
        }
        let block = ".markdown-reading-view .markdown-rendered pre:has(> code)";
        if let Some(width) = code.reading_border_width_px {
            writeln!(
                css,
                "{block} {{ --code-border-width: {width}px; --code-border-color: var(--text-muted); }}"
            )
            .unwrap();
        }
        if code.reading_horizontal_scroll {
            writeln!(
                css,
                "{block} {{ --code-white-space: pre; overflow-x: auto; }}\n\
                 {block} > code {{ white-space: pre; }}"
            )
            .unwrap();
        }
    }
    if let Some(layout) = &theme.layers.obsidian_layout {
        writeln!(
            css,
            concat!(
                "\n.markdown-reading-view > .markdown-preview-view,\n",
                ".markdown-source-view.is-live-preview {{\n",
                "  --lbrx-file-padding-inline: {};\n",
                "}}\n",
                ".markdown-reading-view > .markdown-preview-view.is-readable-line-width,\n",
                ".markdown-source-view.is-live-preview.is-readable-line-width {{\n",
                "  --file-line-width: calc({}px - var(--lbrx-file-padding-inline) * 2);\n",
                "}}\n",
                ".markdown-reading-view > .markdown-preview-view,\n",
                ".markdown-source-view.is-live-preview > .cm-editor > .cm-scroller {{\n",
                "  padding-inline: var(--lbrx-file-padding-inline);\n",
                "}}"
            ),
            file_padding(layout),
            layout.max_width_px
        )
        .unwrap();
    }
    if let Some(mobile) = &theme.layers.obsidian_mobile {
        mobile_styles(&mut css, mobile, theme.layers.obsidian_layout.as_ref());
    }
    if let Some(print) = &theme.layers.obsidian_print {
        print_styles(&mut css, print);
    }
    if let Some(paragraph) = &theme.layers.obsidian_paragraph {
        let scope = ".markdown-reading-view > .markdown-preview-view > .markdown-preview-sizer";
        if let Some(gap) = paragraph.gap_px {
            writeln!(
                css,
                "\n{scope} > .el-p:has(+ .el-p) > p {{\n  margin-block-end: 0;\n}}\n\
                 {scope} > .el-p + .el-p > p {{\n  margin-block-start: {gap}px;\n}}"
            )
            .unwrap();
        }
        if let Some(alignment) = paragraph.alignment {
            writeln!(
                css,
                "\n{scope} > .el-p > p {{\n  text-align: {};\n}}",
                alignment.css_value()
            )
            .unwrap();
        }
    }
    if let Some(detail) = &theme.layers.obsidian_prose_detail {
        let mut declarations = String::new();
        if let Some(spacing) = detail.letter_spacing_em {
            writeln!(declarations, "  letter-spacing: {spacing}em;").unwrap();
        }
        if let Some(enabled) = detail.common_ligatures {
            let value = if enabled {
                "common-ligatures"
            } else {
                "no-common-ligatures"
            };
            writeln!(declarations, "  font-variant-ligatures: {value};").unwrap();
        }
        if let Some(enabled) = detail.small_caps {
            let value = if enabled { "small-caps" } else { "normal" };
            writeln!(declarations, "  font-variant-caps: {value};").unwrap();
        }
        if let Some(transform) = detail.text_transform {
            writeln!(declarations, "  text-transform: {};", transform.css_value()).unwrap();
        }
        if !declarations.is_empty() {
            writeln!(
                css,
                "\n.markdown-reading-view > .markdown-preview-view > .markdown-preview-sizer > .el-p > p:not(:has(code)) {{\n{declarations}}}"
            )
            .unwrap();
        }
    }
    if let Some(headings) = &theme.layers.obsidian_headings {
        for (index, (level, style)) in headings.levels().into_iter().enumerate() {
            let number = index + 1;
            let reading = format!(
                ".markdown-reading-view > .markdown-preview-view > .markdown-preview-sizer > .el-{level} > {level}"
            );
            writeln!(
                css,
                concat!(
                    "\n{},\n",
                    ".markdown-source-view.is-live-preview .HyperMD-header-{},\n",
                    ".markdown-source-view.is-live-preview .HyperMD-list-line .cm-header-{} {{\n",
                    "  --{}-size: {}%;\n  --{}-weight: {};\n}}"
                ),
                reading, number, number, level, style.size_percent, level, style.weight
            )
            .unwrap();
            writeln!(
                css,
                "{reading} {{\n  margin-block: {}px {}px;",
                style.margin_before_px, style.margin_after_px
            )
            .unwrap();
            if style.divider {
                css.push_str("  border-block-end: 1px solid var(--text-muted);\n");
            }
            css.push_str("}\n");
        }
    }
    [
        (FILES[0], format!("{manifest:#}\n")),
        (FILES[1], css),
        (
            crate::build::GUIDE,
            include_str!("../Docs/09-obsidian.md").into(),
        ),
    ]
}

fn callout_styles(css: &mut String, class: &str, mode: &ResolvedMode, callouts: &DocumentAlerts) {
    let scope = format!(
        ".theme-{class} :is(.markdown-reading-view .markdown-rendered, .markdown-source-view.is-live-preview)"
    );
    for (kind, default_role) in DOCUMENT_ALERT_ROLES {
        let aliases = match kind {
            "tip" => ", [data-callout=\"hint\"]",
            "warning" => ", [data-callout=\"attention\"]",
            _ => "",
        };
        let callout = format!("{scope} .callout:is([data-callout=\"{kind}\"]{aliases})");
        let color = &mode.roles[callouts.role(kind, default_role)];
        let background = if callouts.shaded_background {
            " background-color: var(--background-secondary);"
        } else {
            ""
        };
        writeln!(
            css,
            "{callout} {{ border-inline-start: 3px solid {color};{background} }}\n\
             {callout} > .callout-title,\n\
             {callout} > .callout-title > .callout-icon .svg-icon {{ color: {color}; }}"
        )
        .unwrap();
    }
}

fn navigation_sidebar_styles(
    css: &mut String,
    scheme: &str,
    mode: &ResolvedMode,
    settings: &ObsidianNavigationSidebar,
) {
    for (pane, enabled, rows) in [
        ("file-explorer", settings.file_explorer, ".tree-item-self"),
        ("outline", settings.outline, ".tree-item-self"),
        (
            "backlink",
            settings.backlinks,
            ":is(.search-result-file-title, .search-result-file-match)",
        ),
    ] {
        if !enabled {
            continue;
        }
        let scope = format!(
            ".theme-{scheme} :is(.mod-left-split, .mod-right-split) .workspace-leaf-content[data-type=\"{pane}\"] .view-content {rows}"
        );
        let active = r#":is(.is-active, .is-selected, [aria-current]:not([aria-current="false"]), [aria-selected="true"])"#;
        let accent = &mode.roles["accent"];
        let focus = &mode.roles["focus"];
        writeln!(
            css,
            "@media (hover: hover) {{\n\
               {scope}:hover:not({active}) {{ background-color: color-mix(in srgb, {accent} 8%, var(--background-secondary)); }}\n\
             }}\n\
             {scope}{active} {{ background-color: color-mix(in srgb, {accent} 18%, var(--background-secondary)); }}\n\
             {scope}:focus-visible {{ outline: 2px solid {focus}; outline-offset: -2px; }}"
        )
        .unwrap();
    }
}

fn dialog_styles(css: &mut String, scheme: &str, mode: &ResolvedMode, settings: &ObsidianDialogs) {
    let theme = format!(".theme-{scheme}:not(.is-mobile)");
    let accent = &mode.roles["accent"];
    let focus = &mode.roles["focus"];
    let stripe = format!("linear-gradient(to top, {accent} 2px, transparent 2px)");
    let disabled = r#":is(:disabled, .is-disabled, [aria-disabled="true"])"#;
    let enabled = format!(":not({disabled})");
    let focus_ring = format!("outline: 2px solid {focus}; outline-offset: -2px;");
    if settings.style_settings {
        let modal = format!("{theme} .modal.mod-settings");
        let controls =
            ":is(input, select, textarea, button, [role=\"button\"], .vertical-tab-nav-item)";
        writeln!(
            css,
            "{modal} {{ --settings-background: var(--background-primary); --modal-sidebar-background: var(--background-secondary); color: var(--text-normal); }}\n\
             {modal} .vertical-tab-content {{ background-color: var(--background-primary); }}\n\
             {modal} .vertical-tab-header-group-title {{ color: var(--text-muted); }}\n\
             {modal} .vertical-tab-nav-item.is-active{enabled} {{ color: var(--text-normal); background-color: color-mix(in srgb, {accent} 18%, var(--background-secondary)); background-image: {stripe}; }}\n\
             {modal} :is(input:not([type]), input:is([type=\"text\"], [type=\"search\"], [type=\"number\"], [type=\"password\"], [type=\"email\"], [type=\"url\"]), select, textarea){enabled} {{ background-color: var(--background-primary); color: var(--text-normal); }}\n\
             {modal} {controls}{disabled}, {modal} .setting-item.is-disabled :is(.setting-item-name, .setting-item-description) {{ color: var(--text-muted); }}\n\
             {modal} {controls}{enabled}:focus-visible {{ {focus_ring} }}"
        )
        .unwrap();
    }
    if settings.style_prompts {
        let prompt = format!(
            "{theme} .prompt:has(> .prompt-input-container > input.prompt-input):has(> .prompt-results)"
        );
        writeln!(
            css,
            "{prompt} {{ --prompt-background: var(--background-primary); color: var(--text-normal); }}\n\
             {prompt} > .prompt-input-container > input.prompt-input{enabled} {{ background-color: var(--background-secondary); color: var(--text-normal); }}\n\
             {prompt} > .prompt-results .suggestion-item.is-selected{enabled} {{ color: var(--text-normal); background-color: color-mix(in srgb, {accent} 18%, var(--background-primary)); background-image: {stripe}; }}\n\
             {prompt} :is(input.prompt-input, button, .suggestion-item){disabled} {{ color: var(--text-muted); }}\n\
             {prompt} :is(input.prompt-input, button, [role=\"button\"], .suggestion-item){enabled}:focus-visible {{ {focus_ring} }}"
        )
        .unwrap();
    }
    if settings.style_menus {
        let menu = format!("{theme} .menu:has(> .menu-scroll)");
        let row = format!("{menu} > .menu-scroll .menu-item");
        writeln!(
            css,
            "{menu} {{ --menu-background: var(--background-secondary); color: var(--text-normal); }}\n\
             {row}.selected:not(.is-label){enabled} {{ background-color: color-mix(in srgb, {accent} 18%, var(--background-secondary)); background-image: {stripe}; }}\n\
             {row}{disabled}, {row}{disabled} .menu-item-icon {{ color: var(--text-muted); }}\n\
             {row}:not(.is-label){enabled}:focus-visible {{ {focus_ring} }}"
        )
        .unwrap();
    }
}

fn search_styles(css: &mut String, scheme: &str, mode: &ResolvedMode, settings: &ObsidianSearch) {
    let accent = &mode.roles["accent"];
    if settings.highlight_note_matches {
        let reading = format!(
            ".theme-{scheme} .markdown-reading-view .markdown-rendered .search-highlight > div"
        );
        let editing = format!(
            ".theme-{scheme} .markdown-source-view.mod-cm6:has(> .document-search-container) .cm-s-obsidian span.obsidian-search-match-highlight"
        );
        // The native editor uses one decoration for both Find Next and Find All.
        writeln!(
            css,
            "{reading} {{ background-color: color-mix(in srgb, {accent} 16%, transparent); box-shadow: none; opacity: 1; mix-blend-mode: normal; }}\n\
             {reading}.is-active {{ background-color: color-mix(in srgb, {accent} 30%, transparent); box-shadow: inset 0 -2px {accent}; }}\n\
             {editing} {{ background-color: color-mix(in srgb, {accent} 16%, transparent); box-shadow: none; mix-blend-mode: normal; }}"
        )
        .unwrap();
    }
    if settings.highlight_vault_matches {
        writeln!(
            css,
            ".theme-{scheme} .search-result-container.mod-global-search .search-result-file-matched-text {{ background-color: color-mix(in srgb, {accent} 18%, var(--background-primary)); color: var(--text-normal); }}"
        )
        .unwrap();
    }
}

fn graph_styles(css: &mut String, scheme: &str, mode: &ResolvedMode) {
    // Both renderers read colors from temporary probes attached directly to body.
    let probe = format!("body.theme-{scheme} > .graph-view");
    let accent = mode.roles["accent"].as_str();
    writeln!(css, "{probe} {{").unwrap();
    for (name, color) in [
        ("node", "var(--text-muted)"),
        ("line", "var(--text-muted)"),
        ("text", "var(--text-normal)"),
        ("node-focused", accent),
        ("node-tag", &mode.roles["keyword"]),
        ("node-attachment", &mode.roles["warning"]),
        ("node-unresolved", &mode.roles["function"]),
    ] {
        writeln!(css, "  --graph-{name}: {color};").unwrap();
    }
    writeln!(
        css,
        "}}\n\
         {probe}.color-line {{ opacity: 0.8; }}\n\
         {probe}.color-fill-unresolved {{ opacity: 0.85; }}\n\
         {probe}:is(.color-fill-highlight, .color-line-highlight) {{ color: {accent}; }}"
    )
    .unwrap();
}

fn canvas_styles(css: &mut String, scheme: &str, mode: &ResolvedMode, settings: &ObsidianCanvas) {
    let canvas = format!(".theme-{scheme} .canvas-wrapper .canvas");
    let node = format!("{canvas} .canvas-node:not(.is-dummy)");
    let edge = format!("{canvas} .canvas-edges g:not(.is-themed)");
    if settings.style_default_colors {
        writeln!(
            css,
            "{node}:not(.is-themed), {edge} {{ --canvas-color: var(--text-muted); }}\n\
             {node}:not(.is-themed, .canvas-node-group) > .canvas-node-container {{ background-color: var(--background-secondary); }}"
        )
        .unwrap();
    }
    if settings.highlight_selection {
        let focus = &mode.roles["focus"];
        writeln!(
            css,
            "{node}:is(.is-selected, .is-focused) > .canvas-node-container {{ outline: 2px solid {focus}; outline-offset: 2px; }}\n\
             {canvas} .canvas-selection {{ border-color: {focus}; }}\n\
             {canvas} .canvas-selection.mod-group-selection {{ border-color: color-mix(in srgb, {focus} 65%, transparent); }}\n\
             {edge}.is-focused {{ --canvas-color: {focus}; }}"
        )
        .unwrap();
    }
}

fn bases_styles(css: &mut String, scheme: &str, mode: &ResolvedMode, settings: &ObsidianBases) {
    if settings.style_table {
        let table = format!(".theme-{scheme} .bases-view[data-view-type=\"table\"]");
        let accent = &mode.roles["accent"];
        writeln!(
            css,
            "{table} {{\n  --bases-table-header-background: var(--background-secondary);\n  --bases-table-header-color: var(--text-normal);\n  --bases-table-header-weight: 700;\n  --bases-table-cell-background-selected: color-mix(in srgb, {accent} 22%, transparent);\n}}\n\
             {table} .bases-table-container:focus .bases-table-selection {{ mix-blend-mode: normal; }}"
        )
        .unwrap();
    }
    if settings.style_cards {
        writeln!(
            css,
            ".theme-{scheme} .bases-view .bases-cards-container {{\n  --bases-cards-shadow: 0 0 0 1px color-mix(in srgb, var(--text-muted) 65%, var(--background-primary));\n  --bases-cards-shadow-hover: 0 0 0 1px var(--text-muted);\n}}"
        )
        .unwrap();
    }
}

fn file_padding(layout: &DocumentLayout) -> String {
    format!(
        "clamp({}px, 4vw, {}px)",
        layout.padding_inline_min_px, layout.padding_inline_max_px
    )
}

fn mobile_styles(css: &mut String, settings: &ObsidianMobile, layout: Option<&DocumentLayout>) {
    if settings.style_toolbar {
        css.push_str(concat!(
            ".is-mobile:not(.is-floating-nav) .mobile-toolbar,\n",
            ".is-mobile .mobile-toolbar .mobile-toolbar-options-list,\n",
            ".is-mobile .mobile-toolbar-spacer {\n",
            "  background-color: var(--background-secondary);\n",
            "  color: var(--text-normal);\n",
            "}\n"
        ));
    }
    if settings.style_drawers {
        css.push_str(
            ".is-mobile .workspace-drawer { --mobile-sidebar-background: var(--background-secondary); color: var(--text-normal); }\n",
        );
    }
    if let Some(cap) = settings.phone_padding_max_px {
        let base = layout.map_or_else(|| "var(--file-margins-x, 24px)".to_owned(), file_padding);
        writeln!(
            css,
            "@media screen {{\n\
             \x20 .is-mobile.is-phone .markdown-reading-view > .markdown-preview-view,\n\
             \x20 .is-mobile.is-phone .markdown-source-view.is-live-preview {{ --lbrx-file-padding-inline: min({base}, {cap}px); }}\n\
             \x20 .is-mobile.is-phone .markdown-reading-view > .markdown-preview-view,\n\
             \x20 .is-mobile.is-phone .markdown-source-view.is-live-preview > .cm-editor > .cm-scroller {{\n\
             \x20   padding-inline-start: max(var(--lbrx-file-padding-inline), var(--safe-area-inline-start, 0px));\n\
             \x20   padding-inline-end: max(var(--lbrx-file-padding-inline), var(--safe-area-inline-end, 0px));\n\
             \x20 }}\n}}"
        )
        .unwrap();
    }
}

fn print_styles(css: &mut String, settings: &ObsidianPrint) {
    if !settings.wrap_code && !settings.paginate_tables {
        return;
    }
    let document = "body > .print > .markdown-preview-view.markdown-rendered";
    css.push_str("@media print {\n");
    if settings.wrap_code {
        writeln!(
            css,
            "  {document} pre:has(> code) {{ overflow: visible; break-inside: auto; }}\n  \
             {document} pre:has(> code),\n  \
             {document} pre > code {{ white-space: pre-wrap; overflow-wrap: anywhere; }}"
        )
        .unwrap();
    }
    if settings.paginate_tables {
        let wrapper = format!("{document} .el-table");
        let table = format!("{wrapper} > table");
        writeln!(
            css,
            "  {wrapper}:has(> table) {{ overflow: visible; break-inside: auto; }}\n  \
             {table} {{ max-inline-size: 100%; break-inside: auto; }}\n  \
             {table} > thead {{ display: table-header-group; break-inside: avoid; }}\n  \
             {table} > :is(thead, tbody) > tr > :is(th, td) {{ min-inline-size: 0; white-space: normal; overflow-wrap: anywhere; overflow: visible; text-overflow: clip; }}"
        )
        .unwrap();
    }
    css.push_str("}\n");
}

fn icon_styles(css: &mut String, scheme: &str, mode: &ResolvedMode, settings: &ObsidianIcons) {
    let accent = &mode.roles["accent"];
    let focus = &mode.roles["focus"];
    for (enabled, scope) in [
        (
            settings.style_ribbon,
            ".workspace-ribbon .side-dock-ribbon-action",
        ),
        (
            settings.style_note_header,
            ".workspace-leaf-content[data-type=\"markdown\"] > .view-header .view-action",
        ),
    ] {
        if !enabled {
            continue;
        }
        let button = format!(
            ".theme-{scheme}:not(.is-mobile) {scope}.clickable-icon:not(:disabled, .is-disabled, [aria-disabled=\"true\"])"
        );
        writeln!(
            css,
            "{button} {{ --icon-color: var(--text-muted); --icon-color-hover: var(--text-normal); --icon-color-active: {accent}; --icon-color-focused: {focus}; }}\n\
             {button}.mod-bookmarked {{ --icon-color: {accent}; --icon-color-hover: {accent}; }}\n\
             {button}:focus-visible {{ color: var(--icon-color-focused); --background-modifier-border-focus: {focus}; }}"
        )
        .unwrap();
        if settings.size_px.is_some() || settings.stroke_width.is_some() {
            write!(css, "{button} > svg.svg-icon {{").unwrap();
            if let Some(size) = settings.size_px {
                write!(css, " --icon-size: {size}px;").unwrap();
            }
            if let Some(stroke) = settings.stroke_width {
                write!(css, " --icon-stroke: {stroke};").unwrap();
            }
            css.push_str(" }\n");
        }
    }
}

fn mermaid_styles(css: &mut String, settings: &ObsidianMermaid) {
    let diagram = ":is(.markdown-reading-view .markdown-rendered, .markdown-source-view.is-live-preview .cm-embed-block) .mermaid";
    if settings.use_theme_palette {
        writeln!(
            css,
            "{diagram} {{\n  --mermaid-text-color: var(--text-normal);\n  --mermaid-surface: var(--background-secondary);\n  --mermaid-line-color: var(--text-muted);\n  --mermaid-border-color: var(--text-muted);\n}}"
        )
        .unwrap();
    }
    if settings.scroll_wide_diagrams {
        writeln!(
            css,
            "@media screen {{\n  {diagram} {{ max-inline-size: 100%; overflow-x: auto; }}\n}}"
        )
        .unwrap();
    }
}

fn image_styles(css: &mut String, images: &DocumentImages) {
    let scope = ".markdown-reading-view .markdown-rendered";
    let paragraph = ".el-p > p";
    let embed = ".image-embed:only-child";
    let figure = "figure:has(> img, > :is(a, picture, .image-embed) > img)";
    let margins = images.alignment.map_or("auto", |value| value.margins());
    writeln!(
        css,
        "{scope} {paragraph} > {embed},\n\
         {scope} {figure} > .image-embed {{ display: block; max-inline-size: 100%; }}\n\
         {scope} {paragraph} > img:only-child,\n\
         {scope} {paragraph} > {embed} > img,\n\
         {scope} {paragraph} > :is(a, picture):only-child > img:only-child,\n\
         {scope} {figure} > img,\n\
         {scope} {figure} > :is(a, picture, .image-embed) > img {{ display: block; max-inline-size: 100%; block-size: auto; margin-inline: {margins}; }}\n\
         {scope} {figure} > figcaption {{ margin-block-start: {}px; font-size: {}%; color: var(--text-muted); text-align: center; }}",
        images.caption_gap_px, images.caption_size_percent
    )
    .unwrap();
}

fn list_styles(css: &mut String, lists: &DocumentLists) {
    let scope = ".markdown-reading-view .markdown-rendered :is(.el-ul, .el-ol)";
    let list = ":is(ol, ul)";
    let item = "li:not(:where(.task-list-item, .task-list-item *, .footnotes *))";
    writeln!(
        css,
        "{scope} {list} > {item} {{ margin-inline-start: {}px; }}\n\
         {scope} {list} > {item} + {item} {{ margin-block-start: {}px; }}",
        lists.indent_px, lists.item_gap_px
    )
    .unwrap();
    if let Some(markers) = &lists.markers {
        for (ancestors, marker) in ["", ":is(ol, ul) ", ":is(ol, ul) :is(ol, ul) "]
            .into_iter()
            .zip(markers.shapes())
        {
            let (radius, border, fill) = match marker {
                ListMarker::Disc => ("50%", "none", "var(--list-marker-color)"),
                ListMarker::Circle => ("50%", "1px solid currentColor", "transparent"),
                ListMarker::Square => ("0", "none", "var(--list-marker-color)"),
            };
            let bullet = format!("{scope} {ancestors}ul > {item}");
            writeln!(
                css,
                "{bullet} > .list-bullet {{ --list-bullet-radius: {radius}; --list-bullet-border: {border}; }}\n\
                 {bullet}:not(:where(.is-collapsed, .is-collapsed *)) > .list-bullet::after {{ background-color: {fill}; }}"
            )
            .unwrap();
        }
    }
}

fn footnote_and_rule_styles(
    css: &mut String,
    class: &str,
    theme: &ResolvedTheme,
    mode: &ResolvedMode,
) {
    let scope = format!(".theme-{class} .markdown-reading-view .markdown-rendered");
    if let Some(footnotes) = &theme.layers.obsidian_footnotes {
        let reference = theme.editor_color(
            mode,
            "obsidian",
            FOOTNOTE_REFERENCE_COLOR,
            &mode.roles["function"],
        );
        writeln!(
            css,
            "{scope} a.footnote-ref {{ color: {reference}; }}\n\
             {scope} .footnotes {{ --footnote-size: {}%; color: var(--text-muted); }}",
            footnotes.body_size_percent
        )
        .unwrap();
    }
    if let Some(rule) = &theme.layers.obsidian_horizontal_rule {
        writeln!(
            css,
            "{scope} hr:not(.footnotes *) {{ --hr-color: var(--text-muted); --hr-thickness: {}px; margin-block: {}px; }}",
            rule.thickness_px, rule.margin_block_px
        )
        .unwrap();
    }
}

fn emphasis_styles(css: &mut String, class: &str, colors: &crate::layers::DocumentEmphasisColors) {
    let live = format!(".theme-{class} .markdown-source-view.is-live-preview");
    let rendered = format!(
        ".theme-{class} :is(.markdown-reading-view, .markdown-source-view.is-live-preview) .markdown-rendered"
    );
    let outside_code = ":not(:where(code *, pre *))";
    let prose = ":not(:where(.cm-formatting, [class*=\"cm-formatting-\"], .cm-inline-code, .cm-math, .HyperMD-codeblock *))";
    writeln!(
        css,
        "{rendered} mark:not([data-highlight]){outside_code} {{ background-color: {}; color: inherit; }}\n\
         {live} .cm-highlight:not([class*=\"cm-highlight-\"]){prose} {{ background-color: {}; }}",
        colors.highlight_background, colors.highlight_background
    )
    .unwrap();
    // On a flattened editor token, strike takes priority over italic and strong.
    for (html, token, color) in [
        (":is(strong, b)", "cm-strong", &colors.strong),
        (":is(em, i)", "cm-em", &colors.italic),
        (":is(del, s)", "cm-strikethrough", &colors.strikethrough),
    ] {
        writeln!(
            css,
            "{rendered} {html}{outside_code},\n{live} span.{token}{prose} {{ color: {color}; }}"
        )
        .unwrap();
    }
}

fn blockquote_styles(css: &mut String, class: &str, theme: &ResolvedTheme, mode: &ResolvedMode) {
    let Some(quote) = &theme.layers.obsidian_blockquote else {
        return;
    };
    let mut declarations = String::new();
    if let Some(role) = &quote.text_role {
        writeln!(declarations, "  color: {};", mode.roles[role]).unwrap();
    }
    if let Some(role) = &quote.border_role {
        writeln!(
            declarations,
            "  border-inline-start-color: {};",
            mode.roles[role]
        )
        .unwrap();
    }
    if let Some(width) = quote.border_width_px {
        writeln!(declarations, "  border-inline-start-width: {width}px;").unwrap();
    }
    if let Some(padding) = quote.padding_start_em {
        writeln!(declarations, "  padding-inline-start: {padding}em;").unwrap();
    }
    if declarations.is_empty() {
        return;
    }
    let selector = format!(
        ".theme-{class} .markdown-reading-view .markdown-rendered blockquote:not(.callout *)"
    );
    writeln!(css, "{selector} {{\n{declarations}}}").unwrap();
    if quote.text_role.is_some() {
        // A callout inside a quote would otherwise inherit the quote's text color.
        writeln!(
            css,
            ":where({selector}) .callout:not(:where(.callout *)) {{ color: var(--text-normal); }}"
        )
        .unwrap();
    }
}

fn link_styles(css: &mut String, class: &str, theme: &ResolvedTheme, mode: &ResolvedMode) {
    let links = theme.layers.obsidian_links.as_ref();
    if links.is_some_and(|links| links.keyboard_focus_outline) {
        writeln!(
            css,
            ".theme-{class} .markdown-reading-view .markdown-rendered a:is(.internal-link, .external-link):focus-visible {{\n  outline: 2px solid {};\n  outline-offset: 2px;\n}}",
            mode.roles["focus"]
        )
        .unwrap();
    }
    for (selector, color_keys, decoration_keys, underline) in [
        (
            concat!(
                ".markdown-reading-view .markdown-rendered a.internal-link:not(.is-unresolved), ",
                ".markdown-source-view.is-live-preview .cm-hmd-internal-link:not(.is-unresolved, .is-unresolved *)"
            ),
            &["--link-color", "--link-color-hover"][..],
            ["--link-decoration", "--link-decoration-hover"],
            [
                links.and_then(|v| v.internal_underline),
                links.and_then(|v| v.internal_hover_underline),
            ],
        ),
        (
            concat!(
                ".markdown-reading-view .markdown-rendered a.external-link, ",
                ".markdown-source-view.is-live-preview :is(.cm-link, .cm-url):not(.is-unresolved, .is-unresolved *)"
            ),
            &["--link-external-color-hover"][..],
            [
                "--link-external-decoration",
                "--link-external-decoration-hover",
            ],
            [
                links.and_then(|v| v.external_underline),
                links.and_then(|v| v.external_hover_underline),
            ],
        ),
    ] {
        let mut declarations = String::new();
        for key in color_keys {
            if let Some(role) = theme.editor_role("obsidian", key) {
                writeln!(declarations, "  {key}: {};", mode.roles[role]).unwrap();
            }
        }
        for (key, enabled) in decoration_keys.into_iter().zip(underline) {
            if let Some(enabled) = enabled {
                let value = if enabled { "underline" } else { "none" };
                writeln!(declarations, "  {key}: {value};").unwrap();
            }
        }
        if !declarations.is_empty() {
            writeln!(css, ".theme-{class} :is({selector}) {{\n{declarations}}}").unwrap();
        }
    }
}

fn rgb_to_hsl(rgb: [u8; 3]) -> [f64; 3] {
    let [r, g, b] = rgb.map(|channel| f64::from(channel) / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (max + min) / 2.0;
    let delta = max - min;
    if delta == 0.0 {
        return [0.0, 0.0, lightness * 100.0];
    }
    let sector = if max == r {
        ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());
    [sector * 60.0, saturation * 100.0, lightness * 100.0]
}
