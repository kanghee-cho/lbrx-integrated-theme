use crate::{
    build::App,
    read_theme,
    resolve::{self, ColorSource, ResolvedMode, ResolvedTheme},
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, IsTerminal, Write},
    path::Path,
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn run(source: &Path, selected: &[App]) -> Result<(), String> {
    let mut preview = Preview::open(source, selected)?;
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("preview는 Windows Terminal 또는 VS Code 통합 터미널에서 실행하세요".into());
    }
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(terminal_error)?;
    let result = (|| {
        execute!(stdout, EnterAlternateScreen, Hide)?;
        loop {
            preview.draw(&mut stdout, terminal::size()?)?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && preview.key(key.code) {
                    return Ok(());
                }
            }
        }
    })();
    let restore_screen = execute!(stdout, ResetColor, Show, LeaveAlternateScreen);
    let restore_input = terminal::disable_raw_mode();
    result
        .and(restore_screen)
        .and(restore_input)
        .map_err(terminal_error)
}

fn terminal_error(error: io::Error) -> String {
    format!("미리보기: {error}")
}

struct Language {
    id: &'static str,
    label: &'static str,
    file: &'static str,
    comment: &'static str,
    code: &'static [(&'static str, &'static str)],
}

const LANGUAGES: &[Language] = &[
    Language {
        id: "python",
        label: "Python",
        file: "example.py",
        comment: "# 주석 · Comment",
        code: &[
            ("keyword", "def "),
            ("function", "greet"),
            ("text", "(): "),
            ("keyword", "return "),
            ("string", "\"안녕\""),
            ("text", " * "),
            ("number", "2"),
        ],
    },
    Language {
        id: "rust",
        label: "Rust",
        file: "example.rs",
        comment: "// 인사말",
        code: &[
            ("keyword", "fn "),
            ("function", "greet"),
            ("text", "() { "),
            ("keyword", "let "),
            ("text", "x = ("),
            ("string", "\"안녕\""),
            ("text", ", "),
            ("number", "2"),
            ("text", "); }"),
        ],
    },
    Language {
        id: "powershell",
        label: "PowerShell",
        file: "example.ps1",
        comment: "# 인사말",
        code: &[
            ("keyword", "if "),
            ("text", "($ok) { "),
            ("function", "Write-Output"),
            ("text", " ("),
            ("string", "\"안녕\""),
            ("text", " * "),
            ("number", "2"),
            ("text", ") }"),
        ],
    },
    Language {
        id: "toml",
        label: "TOML",
        file: "example.toml",
        comment: "# 인사말 설정",
        code: &[
            ("text", "message = ["),
            ("string", "\"안녕\""),
            ("text", ", "),
            ("number", "2"),
            ("text", "]"),
        ],
    },
    Language {
        id: "typescript",
        label: "TypeScript",
        file: "example.ts",
        comment: "// 인사말",
        code: &[
            ("keyword", "function "),
            ("function", "greet"),
            ("text", "() {"),
            ("keyword", "return "),
            ("string", "\"안녕\""),
            ("text", ".repeat("),
            ("number", "2"),
            ("text", ");}"),
        ],
    },
];

impl Language {
    fn tokens(&self) -> impl Iterator<Item = &'static str> + '_ {
        crate::layers::TOKENS
            .iter()
            .copied()
            .filter(|token| *token == "comment" || self.code.iter().any(|(kind, _)| kind == token))
    }
}

fn has_language(app: App) -> bool {
    !matches!(app, App::WindowsTerminal | App::Iterm2)
}

struct Preview<'a> {
    source: &'a Path,
    theme: ResolvedTheme,
    apps: Vec<App>,
    index: usize,
    language: usize,
    info: bool,
    dark: bool,
    status: String,
}

impl<'a> Preview<'a> {
    fn open(source: &'a Path, selected: &[App]) -> Result<Self, String> {
        Ok(Self {
            source,
            theme: load(source)?,
            apps: App::ALL
                .into_iter()
                .filter(|app| selected.is_empty() || selected.contains(app))
                .collect(),
            index: 0,
            language: 0,
            info: false,
            dark: false,
            status: String::new(),
        })
    }

    fn key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Tab | KeyCode::Right => self.index = (self.index + 1) % self.apps.len(),
            KeyCode::BackTab | KeyCode::Left => {
                self.index = (self.index + self.apps.len() - 1) % self.apps.len();
            }
            KeyCode::Char('m' | 'M') => self.dark = !self.dark,
            KeyCode::Char('l' | 'L') if has_language(self.apps[self.index]) => {
                self.language = (self.language + 1) % LANGUAGES.len();
            }
            KeyCode::Char('i' | 'I') => self.info = !self.info,
            KeyCode::Char('r' | 'R') => match load(self.source) {
                Ok(theme) => {
                    self.theme = theme;
                    self.status = "원본을 다시 읽었습니다".into();
                }
                Err(error) => {
                    self.status = format!("다시 읽기 실패 (마지막 정상 원본 표시): {error}")
                }
            },
            KeyCode::Esc | KeyCode::Char('q' | 'Q') => return true,
            _ => {}
        }
        false
    }

    fn draw(&self, out: &mut impl Write, (width, height): (u16, u16)) -> io::Result<()> {
        queue!(out, ResetColor, MoveTo(0, 0), Clear(ClearType::All))?;
        let mut page = Page {
            out,
            width: width.saturating_sub(1),
            row: 0,
        };
        if width < 50 || height < 19 {
            if height > 1 {
                page.line("미리보기: 50열 × 19행 이상 필요")?;
            }
            if height > 0 {
                page.row = height - 1;
                page.line("[Esc/Q] 종료")?;
            }
            return page.out.flush();
        }

        let tab_style = &self.theme.light;
        let labels_width: usize = self.apps.iter().map(|app| app.label().width()).sum();
        let width = usize::from(page.width);
        let full_width = labels_width + 2 * self.apps.len();
        let compact_terminal = full_width > width && self.apps.contains(&App::WindowsTerminal);
        let compact_width = if compact_terminal {
            full_width - (App::WindowsTerminal.label().width() - "WT".width())
        } else {
            full_width
        };
        let gap = if compact_width > width { " " } else { "  " };
        queue!(page.out, MoveTo(0, page.row))?;
        for (index, app) in self.apps.iter().enumerate() {
            let label = if *app == App::WindowsTerminal && compact_terminal {
                "WT"
            } else {
                app.label()
            };
            if index > 0 {
                queue!(page.out, Print(gap))?;
            }
            if index == self.index {
                queue!(
                    page.out,
                    SetBackgroundColor(parse_color(&tab_style.roles["background"])),
                    SetForegroundColor(parse_color(&tab_style.roles["text"])),
                    Print(format!("[{label}]")),
                    ResetColor
                )?;
            } else {
                queue!(page.out, Print(label))?;
            }
        }
        page.row += 1;
        page.rule()?;
        page.line(&format!("원본: {}", self.source.display()))?;
        page.row += 1;

        let app = self.apps[self.index];
        let base = if self.dark {
            &self.theme.dark
        } else {
            &self.theme.light
        };
        let language = &LANGUAGES[self.language];
        let sample = sample_mode(&self.theme, base, app, language);
        let mode = &sample;
        let (bg, fg) = match app {
            App::WindowsTerminal | App::Iterm2 => ("terminal_background", "terminal_foreground"),
            _ => ("background", "text"),
        };
        let background = parse_color(&mode.roles[bg]);
        let foreground = parse_color(&mode.roles[fg]);
        let sample_end = height - 5;
        queue!(
            page.out,
            SetBackgroundColor(background),
            SetForegroundColor(foreground)
        )?;
        for row in page.row..sample_end {
            queue!(page.out, MoveTo(0, row), Clear(ClearType::UntilNewLine))?;
        }
        queue!(
            page.out,
            SetBackgroundColor(parse_color(&mode.roles["surface"])),
            MoveTo(0, page.row),
            Clear(ClearType::UntilNewLine)
        )?;
        let view = if self.info {
            "적용 근거"
        } else {
            "색상 견본"
        };
        let mut title = format!(
            "  {} · {} · {view}",
            app.label(),
            if self.dark { "다크" } else { "라이트" }
        );
        if has_language(app) {
            title.push_str(" · ");
            title.push_str(language.label);
        }
        page.line(&title)?;
        queue!(page.out, SetBackgroundColor(background))?;
        page.rule()?;
        if self.info {
            self.draw_info(&mut page, base, app, language)?;
        } else {
            self.draw_sample(&mut page, mode, app, language, (bg, fg))?;
        }

        queue!(page.out, ResetColor)?;
        page.row = sample_end + 1;
        let fonts = &self.theme.typography;
        match app {
            App::WindowsTerminal => page.line(&format!(
                "프로필 글꼴: {} · {}pt",
                fonts.mono.families[0],
                fonts.mono.size_px * 0.75
            ))?,
            App::Iterm2 => page.line("글꼴: iTerm2 프로파일에서 직접 설정")?,
            App::Vim => page.line("글꼴: 호스트 터미널 설정 · RGB 색상 견본")?,
            App::Helix => page.line("글꼴: 호스트 터미널 설정 · True Color")?,
            App::VsCode => page.line(&format!(
                "편집기 글꼴 (별도 설정): {} · {}px",
                fonts.mono.families[0], fonts.mono.size_px
            ))?,
            App::Typora => page.line(&format!(
                "본문: {} ({}px) · 코드: {} ({}px)",
                fonts.prose.families[0],
                fonts.prose.size_px,
                fonts.mono.families[0],
                fonts.mono.size_px
            ))?,
            App::Obsidian => page.line(&format!(
                "본문: {} · 코드: {} (크기는 앱 설정)",
                fonts.prose.families[0], fonts.mono.families[0]
            ))?,
        }
        page.line(&self.status)?;
        page.rule()?;
        page.line(if has_language(app) {
            "[Tab]앱 [L]언어 [M]모드 [I]근거 [R]갱신 [Q]종료"
        } else {
            "[Tab]앱 [M]모드 [I]근거 [R]갱신 [Esc/Q]종료"
        })?;
        page.out.flush()
    }

    fn draw_sample(
        &self,
        page: &mut Page<'_, impl Write>,
        mode: &ResolvedMode,
        app: App,
        language: &Language,
        (bg, fg): (&str, &str),
    ) -> io::Result<()> {
        let background = parse_color(&mode.roles[bg]);
        let foreground = parse_color(&mode.roles[fg]);
        match app {
            App::WindowsTerminal | App::Iterm2 => {
                page.line(if app == App::Iterm2 {
                    "  % ls"
                } else {
                    "  PS> Get-ChildItem"
                })?;
                page.line("  themes      lbrx.toml      README.md")?;
                page.styled(
                    mode,
                    "terminal_foreground",
                    Some("terminal_cursor"),
                    " ",
                    "  ",
                )?;
                page.line("  ANSI 16")?;
                for colors in mode.ansi.chunks_exact(8) {
                    queue!(page.out, MoveTo(2, page.row))?;
                    for hex in colors {
                        queue!(
                            page.out,
                            SetForegroundColor(parse_color(hex)),
                            Print("███ ")
                        )?;
                    }
                    page.row += 1;
                }
            }
            App::Typora => {
                page.styled(mode, "heading", None, "프로젝트 노트", "  ")?;
                page.styled(mode, "link", None, "문서 링크", "  ")?;
                page.styled(mode, "quote", None, "인용문 · 읽기 흐름 확인", "  ")?;
                page.code_block(mode, language)?;
            }
            App::Obsidian => {
                page.styled(
                    mode,
                    "muted",
                    Some("surface"),
                    "보관함 / 프로젝트 노트",
                    "  ",
                )?;
                page.styled(mode, "link", None, "[[관련 노트]]", "  ")?;
                page.styled(mode, "external_link", None, "외부 링크", "  ")?;
                page.code_block(mode, language)?;
            }
            App::Vim => {
                page.styled(mode, "comment", None, language.comment, "  ")?;
                page.code(mode, language)?;
                page.styled(mode, "line_number", None, "1 · LineNr", "  ")?;
                page.styled(mode, "background", Some("warning"), " 검색 · Search ", "  ")?;
                page.styled(
                    mode,
                    "text",
                    Some("surface"),
                    &format!(" {}  ·  StatusLine ", language.file),
                    "  ",
                )?;
            }
            App::VsCode => {
                page.styled(
                    mode,
                    "text",
                    Some("sidebar"),
                    &format!(" Explorer   •   {} ", language.file),
                    "  ",
                )?;
                page.styled(mode, "comment", None, language.comment, "  ")?;
                page.code(mode, language)?;
                page.styled(mode, "editor_cursor", None, "│ 편집기 커서", "  ")?;
            }
            App::Helix => {
                page.styled(mode, "line_number", None, "1  줄 번호 · ui.linenr", "  ")?;
                page.styled(
                    mode,
                    "selected_line_number",
                    None,
                    "2  선택된 줄 번호",
                    "  ",
                )?;
                page.styled(mode, "comment", None, language.comment, "  ")?;
                page.code(mode, language)?;
                page.styled(
                    mode,
                    "text",
                    Some("surface"),
                    &format!(" NOR  {} ", language.file),
                    "  ",
                )?;
            }
        }
        if has_language(app) {
            queue!(page.out, SetForegroundColor(foreground))?;
            page.line(&format!(
                "  {}",
                language_summary(&self.theme, app, language)
            ))?;
        }
        queue!(
            page.out,
            SetBackgroundColor(background),
            SetForegroundColor(foreground)
        )?;
        let selection_fg = if app == App::WindowsTerminal {
            fg
        } else {
            "selection_text"
        };
        if matches!(app, App::Typora | App::Obsidian) {
            queue!(
                page.out,
                MoveTo(2, page.row),
                SetForegroundColor(parse_color(&mode.roles[selection_fg])),
                SetBackgroundColor(parse_color(mode.prose_selection_background())),
                Print(" 본문 선택 "),
                SetBackgroundColor(background),
                Print("  "),
                SetBackgroundColor(parse_color(&mode.roles["selection_background"])),
                Print(" 코드 선택 "),
            )?;
            page.row += 1;
        } else {
            page.styled(
                mode,
                selection_fg,
                Some("selection_background"),
                " 선택 영역 ",
                "  ",
            )?;
        }
        queue!(
            page.out,
            SetBackgroundColor(background),
            SetForegroundColor(foreground)
        )?;
        page.line(&format!(
            "  글자/배경 대비 {:.2}:1",
            resolve::contrast_ratio(
                resolve::parse_rgb(&mode.roles[fg]).expect("resolved color"),
                resolve::parse_rgb(&mode.roles[bg]).expect("resolved color"),
            )
        ))?;

        Ok(())
    }

    fn draw_info(
        &self,
        page: &mut Page<'_, impl Write>,
        mode: &ResolvedMode,
        app: App,
        language: &Language,
    ) -> io::Result<()> {
        let (id, options) = app_options(app);
        let base = self
            .source
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("원본");
        let language_file = format!("{}.toml", language.id);
        let editor_file = format!("{id}.toml");
        let mut show = |label: &str, choice: Choice<'_>, split: bool| {
            let file = match choice.source {
                ColorSource::Language => &language_file,
                ColorSource::Common => "common.toml",
                ColorSource::Editor => &editor_file,
                ColorSource::Base => base,
            };
            let hex = &mode.roles[choice.role];
            if split {
                page.line(&format!("  {label} · {hex} · {}", choice.role))?;
                page.line(&format!("    ↳ {file}"))
            } else {
                page.line(&format!("  {label} · {hex} · {file} · {}", choice.role))
            }
        };
        if has_language(app) {
            for token in language.tokens() {
                show(
                    token,
                    sample_token(&self.theme, app, language, token),
                    false,
                )?;
            }
        } else {
            for (label, role) in [
                ("배경", "terminal_background"),
                ("전경", "terminal_foreground"),
            ] {
                show(
                    label,
                    Choice {
                        role,
                        source: ColorSource::Base,
                    },
                    false,
                )?;
            }
        }
        for option in options {
            show(
                option.label,
                option.choice(&self.theme, id),
                !has_language(app),
            )?;
        }
        if matches!(app, App::Typora | App::Obsidian) {
            page.line(&format!(
                "  코드 배경 · {} · {base}",
                mode.code_block_background()
            ))
        } else if has_language(app) {
            page.line(&format!(
                "  {}",
                language_summary(&self.theme, app, language)
            ))
        } else {
            page.line(&format!("  ANSI 16 · {base}"))
        }
    }
}

struct Choice<'a> {
    role: &'a str,
    source: ColorSource,
}

struct AppOption {
    sample: &'static str,
    key: &'static str,
    fallback: &'static str,
    label: &'static str,
}

impl AppOption {
    const fn new(
        sample: &'static str,
        key: &'static str,
        fallback: &'static str,
        label: &'static str,
    ) -> Self {
        Self {
            sample,
            key,
            fallback,
            label,
        }
    }

    fn choice<'a>(&self, theme: &'a ResolvedTheme, id: &str) -> Choice<'a> {
        if let Some(role) = theme.editor_role(id, self.key) {
            Choice {
                role,
                source: ColorSource::Editor,
            }
        } else {
            let (role, source) = theme.token_role("", self.fallback);
            Choice { role, source }
        }
    }
}

fn app_options(app: App) -> (&'static str, [AppOption; 2]) {
    use AppOption as O;
    match app {
        App::WindowsTerminal => (
            "windows-terminal",
            [
                O::new("terminal_cursor", "cursorColor", "terminal_cursor", "커서"),
                O::new(
                    "selection_background",
                    "selectionBackground",
                    "selection_background",
                    "선택 배경",
                ),
            ],
        ),
        App::Iterm2 => (
            "iterm2",
            [
                O::new("terminal_cursor", "Cursor Color", "terminal_cursor", "커서"),
                O::new(
                    "selection_text",
                    "Selected Text Color",
                    "selection_text",
                    "선택 전경",
                ),
            ],
        ),
        App::VsCode => (
            "vscode",
            [
                O::new("sidebar", "sideBar.background", "surface", "사이드바"),
                O::new("editor_cursor", "editorCursor.foreground", "focus", "커서"),
            ],
        ),
        App::Typora => (
            "typora",
            [
                O::new("quote", "--lbrx-quote", "quote", "인용"),
                O::new("link", "--lbrx-link", "link", "링크"),
            ],
        ),
        App::Obsidian => (
            "obsidian",
            [
                O::new(
                    "external_link",
                    "--link-external-color",
                    "link",
                    "외부 링크",
                ),
                O::new("muted", "--text-muted", "muted", "보조 글자"),
            ],
        ),
        App::Helix => (
            "helix",
            [
                O::new("line_number", "ui.linenr", "muted", "줄 번호"),
                O::new(
                    "selected_line_number",
                    "ui.linenr.selected",
                    "text",
                    "선택 줄 번호",
                ),
            ],
        ),
        App::Vim => (
            "vim",
            [
                O::new("line_number", "LineNr", "muted", "줄 번호"),
                O::new("comment", "Comment", "comment", "기본 주석"),
            ],
        ),
    }
}

fn supports_override(app: App, language: &Language, token: &str) -> bool {
    match app {
        App::Vim => crate::vim::language_groups(language.id, token).is_some(),
        App::VsCode | App::Typora | App::Obsidian => true,
        _ => false,
    }
}

fn sample_token<'a>(
    theme: &'a ResolvedTheme,
    app: App,
    language: &Language,
    token: &'a str,
) -> Choice<'a> {
    let lang = if supports_override(app, language, token) {
        language.id
    } else {
        ""
    };
    let (role, source) = theme.token_role(lang, token);
    if source != ColorSource::Language && app == App::Vim && token == "comment" {
        return app_options(app).1[1].choice(theme, "vim");
    }
    if source == ColorSource::Base && app == App::Obsidian && token == "number" {
        return Choice {
            role: "text",
            source,
        };
    }
    Choice { role, source }
}

fn language_summary(theme: &ResolvedTheme, app: App, language: &Language) -> String {
    if app == App::Helix {
        return "공통 구문색만 적용".into();
    }
    let mut applied = 0;
    let mut unsupported = 0;
    for token in language.tokens() {
        if theme.token_role(language.id, token).1 == ColorSource::Language {
            if supports_override(app, language, token) {
                applied += 1;
            } else {
                unsupported += 1;
            }
        }
    }
    let mut result = if applied == 0 {
        "공통 구문색 사용".into()
    } else {
        format!("견본 언어 예외: {applied}개")
    };
    if unsupported > 0 {
        result.push_str(&format!(" · 미반영 {unsupported}개"));
    }
    if app == App::Obsidian {
        result.push_str(" · 읽기 뷰");
    }
    result
}

fn sample_mode(
    theme: &ResolvedTheme,
    base: &ResolvedMode,
    app: App,
    language: &Language,
) -> ResolvedMode {
    let mut sample = base.clone();
    let (id, options) = app_options(app);
    for option in options {
        sample.roles.insert(
            option.sample.into(),
            base.roles[option.choice(theme, id).role].clone(),
        );
    }
    if has_language(app) {
        for token in language.tokens() {
            sample.roles.insert(
                token.into(),
                base.roles[sample_token(theme, app, language, token).role].clone(),
            );
        }
    }
    sample
}

fn load(source: &Path) -> Result<ResolvedTheme, String> {
    resolve::resolve(&read_theme(source)?).map_err(|error| format!("{}: {error}", source.display()))
}

fn parse_color(hex: &str) -> Color {
    let [r, g, b] = resolve::parse_rgb(hex).expect("resolved color");
    Color::Rgb { r, g, b }
}

struct Page<'a, W> {
    out: &'a mut W,
    width: u16,
    row: u16,
}

impl<W: Write> Page<'_, W> {
    fn rule(&mut self) -> io::Result<()> {
        self.line(&"─".repeat(usize::from(self.width)))
    }

    fn line(&mut self, text: &str) -> io::Result<()> {
        let mut remaining = usize::from(self.width);
        let clipped: String = text
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .take_while(|c| {
                let width = c.width().unwrap_or(0);
                if width > remaining {
                    return false;
                }
                remaining -= width;
                true
            })
            .collect();
        queue!(self.out, MoveTo(0, self.row), Print(clipped))?;
        self.row += 1;
        Ok(())
    }

    fn styled(
        &mut self,
        mode: &ResolvedMode,
        fg: &str,
        bg: Option<&str>,
        text: &str,
        indent: &str,
    ) -> io::Result<()> {
        if let Some(bg) = bg {
            queue!(self.out, SetBackgroundColor(parse_color(&mode.roles[bg])))?;
        }
        queue!(self.out, SetForegroundColor(parse_color(&mode.roles[fg])))?;
        self.line(&format!("{indent}{text}"))?;
        if bg.is_some() {
            queue!(
                self.out,
                SetBackgroundColor(parse_color(
                    &mode.roles[if fg == "terminal_foreground" {
                        "terminal_background"
                    } else {
                        "background"
                    }]
                ))
            )?;
        }
        Ok(())
    }

    fn code_block(&mut self, mode: &ResolvedMode, language: &Language) -> io::Result<()> {
        let fill = " ".repeat(usize::from(self.width.saturating_sub(2)));
        queue!(
            self.out,
            SetBackgroundColor(parse_color(mode.code_block_background())),
            MoveTo(2, self.row),
            Print(&fill),
            MoveTo(2, self.row + 1),
            Print(&fill),
        )?;
        queue!(
            self.out,
            SetForegroundColor(parse_color(&mode.roles["comment"])),
            MoveTo(2, self.row),
            Print(language.comment),
        )?;
        self.row += 1;
        self.code(mode, language)?;
        queue!(
            self.out,
            SetBackgroundColor(parse_color(&mode.roles["background"]))
        )
    }

    fn code(&mut self, mode: &ResolvedMode, language: &Language) -> io::Result<()> {
        queue!(self.out, MoveTo(2, self.row))?;
        let mut remaining = usize::from(self.width.saturating_sub(2));
        for &(role, text) in language.code {
            let width = text.width();
            if width > remaining {
                break;
            }
            remaining -= width;
            queue!(
                self.out,
                SetForegroundColor(parse_color(&mode.roles[role])),
                Print(text)
            )?;
        }
        self.row += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[cfg(windows)]
    const SOURCE: &str = include_str!("..\\themes\\lbrx.toml");
    #[cfg(not(windows))]
    const SOURCE: &str = include_str!("../themes/lbrx.toml");

    fn render(preview: &Preview<'_>) -> String {
        let mut screen = Vec::new();
        preview.draw(&mut screen, (80, 19)).unwrap();
        String::from_utf8(screen).unwrap()
    }

    #[test]
    fn selected_apps_show_distinct_mode_samples() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        let text = SOURCE.replacen(
            "terminal_background = \"paper\"",
            "terminal_background = \"selection\"",
            1,
        );
        fs::write(&source, &text).unwrap();
        let mut preview = Preview::open(&source, &[App::WindowsTerminal, App::Obsidian]).unwrap();
        let terminal = render(&preview);
        assert!(terminal.contains(
            "\x1b[48;2;243;244;236m\x1b[38;2;36;54;47m[Windows Terminal]\x1b[0m  Obsidian"
        ));
        assert!(terminal.contains("\x1b[3;1H원본: "));
        assert!(terminal.contains(concat!(
            "\x1b[48;2;232;237;228m\x1b[5;1H\x1b[K",
            "\x1b[5;1H  Windows Terminal · 라이트 · 색상 견본",
            "\x1b[48;2;216;230;218m"
        )));
        assert!(terminal.contains("\x1b[14;1H  글자/배경 대비"));
        assert!(terminal.contains("\x1b[16;1H프로필 글꼴: "));
        assert!(terminal.ends_with("\x1b[19;1H[Tab]앱 [M]모드 [I]근거 [R]갱신 [Esc/Q]종료"));
        let rule = "─".repeat(79);
        for row in [2, 6, 18] {
            assert!(terminal.contains(&format!("\x1b[{row};1H{rule}")));
        }
        for row in [4, 15] {
            assert!(!terminal.contains(&format!("\x1b[{row};1H{rule}")));
        }
        assert_eq!(terminal.matches("[Tab]앱").count(), 1);
        assert!(terminal.contains("ANSI 16"));
        preview.key(KeyCode::Tab);
        preview.key(KeyCode::Char('m'));
        let obsidian = render(&preview);
        assert!(obsidian.contains(
            "Windows Terminal  \x1b[48;2;243;244;236m\x1b[38;2;36;54;47m[Obsidian]\x1b[0m"
        ));
        assert!(obsidian.contains(concat!(
            "\x1b[48;2;37;51;44m\x1b[5;1H\x1b[K",
            "\x1b[5;1H  Obsidian · 다크 · 색상 견본 · Python",
            "\x1b[48;2;25;35;31m"
        )));
        assert!(obsidian.contains("[[관련 노트]]"));
        assert!(!obsidian.contains("ANSI 16"));
        assert!(!obsidian.contains("Typora"));
        assert_eq!(fs::read_to_string(source).unwrap(), text);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[test]
    fn document_code_sample_uses_its_own_background() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let mut preview = Preview::open(&source, &[App::Typora, App::Obsidian, App::Vim]).unwrap();
        for (fill, hex) in [
            ("\x1b[48;2;232;237;228m", "#E8EDE4"),
            ("\x1b[48;2;37;51;44m", "#25332C"),
        ] {
            for _ in 0..2 {
                assert!(render(&preview).contains(&format!("{fill}\x1b[10;3H ")));
                preview.key(KeyCode::Char('i'));
                assert!(render(&preview).contains(&format!("코드 배경 · {hex} · theme.toml")));
                preview.key(KeyCode::Char('i'));
                preview.key(KeyCode::Tab);
            }
            assert!(!render(&preview).contains(&format!("{fill}\x1b[10;3H ")));
            preview.key(KeyCode::Tab);
            preview.key(KeyCode::Char('m'));
        }
    }

    #[test]
    fn vim_preview_displays_editor_samples_and_host_font_notice() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let mut preview = Preview::open(&source, &[App::Vim]).unwrap();
        preview.key(KeyCode::Char('m'));
        let screen = render(&preview);
        assert!(screen.contains("Vim · 다크"));
        assert!(screen.contains("# 주석 · Comment"));
        assert!(screen.contains("StatusLine"));
        assert!(screen.contains("글꼴: 호스트 터미널 설정"));
        assert!(!screen.contains("ANSI 16"));
    }

    #[test]
    fn all_tabs_fit_at_minimum_width_with_helix_selected() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let mut preview = Preview::open(&source, &[]).unwrap();
        for _ in 1..App::ALL.len() {
            preview.key(KeyCode::Tab);
        }
        preview.key(KeyCode::Char('m'));
        let mut screen = Vec::new();
        preview.draw(&mut screen, (50, 19)).unwrap();
        let screen = String::from_utf8(screen).unwrap();
        let tabs = "WT Typora Obsidian iTerm2 Vim VS Code ";
        assert!(screen.contains(tabs));
        assert!(screen.contains("[Helix]\x1b[0m\x1b[2;1H"));
        assert!(tabs.width() + "[Helix]".width() < 50);
        assert!(screen.contains("Helix · 다크"));
        assert!(screen.contains("ui.linenr"));
        assert!(screen.contains("NOR  example.py"));
    }

    #[test]
    fn iterm2_preview_shows_terminal_roles_and_manual_font_setting() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let mut preview = Preview::open(&source, &[App::Iterm2]).unwrap();
        preview.key(KeyCode::Char('m'));
        let screen = render(&preview);
        assert!(screen.contains("iTerm2 · 다크"));
        assert!(screen.contains("  % ls"));
        assert!(screen.contains("ANSI 16"));
        assert!(screen.contains("글꼴: iTerm2 프로파일에서 직접 설정"));
        assert!(!screen.contains("PS>"));
    }

    #[test]
    fn language_details_follow_generated_priority_and_remember_the_language() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let langs = dir.path().join("languages");
        fs::create_dir(&langs).unwrap();
        fs::write(
            langs.join("common.toml"),
            "[tokens]\nfunction='string'\nstring='string'",
        )
        .unwrap();
        fs::write(
            langs.join("python.toml"),
            "[tokens]\nfunction='accent'\nkeyword='text'",
        )
        .unwrap();
        let editors = dir.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(
            editors.join("vscode.toml"),
            "[colors]\n'sideBar.background'='selection_background'",
        )
        .unwrap();

        let mut preview = Preview::open(&source, &[App::Vim, App::VsCode, App::Helix]).unwrap();
        assert!(render(&preview).contains("견본 언어 예외: 1개 · 미반영 1개"));
        preview.key(KeyCode::Tab);
        preview.key(KeyCode::Char('m'));
        let screen = render(&preview);
        assert!(screen.contains("\x1b[48;2;37;51;44m\x1b[5;1H\x1b[K"));
        assert!(screen.contains("\x1b[48;2;48;65;56m\x1b[38;2;220;230;219m\x1b[7;1H   Explorer"));
        assert!(screen.contains("\x1b[38;2;132;199;172mgreet"));
        preview.key(KeyCode::Char('i'));
        let screen = render(&preview);
        assert!(screen.contains("function · #84C7AC · python.toml · accent"));
        assert!(screen.contains("string · #84C7AC · common.toml · string"));
        assert!(screen.contains("사이드바 · #304138 · vscode.toml"));
        preview.key(KeyCode::Tab);
        let screen = render(&preview);
        assert!(screen.contains("Helix · 다크 · 적용 근거 · Python"));
        assert!(screen.contains("function · #84C7AC · common.toml · string"));
        assert!(screen.contains("공통 구문색만 적용"));
        preview.key(KeyCode::Char('l'));
        preview.key(KeyCode::Left);
        let rust = render(&preview);
        assert!(rust.contains("VS Code · 다크 · 적용 근거 · Rust"));
        assert!(!rust.contains("python.toml"));
        preview.key(KeyCode::Char('l'));
        preview.key(KeyCode::Char('l'));
        let toml = render(&preview);
        assert!(toml.contains("적용 근거 · TOML"));
        assert!(!toml.contains("function ·"));
        preview.key(KeyCode::Char('i'));
        assert!(render(&preview).contains("message = ["));
    }

    #[test]
    fn terminal_details_hide_the_language_control_and_fit_at_minimum_width() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let editors = dir.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(
            editors.join("windows-terminal.toml"),
            "[colors]\nselectionBackground='surface'",
        )
        .unwrap();
        let mut preview = Preview::open(&source, &[App::WindowsTerminal, App::VsCode]).unwrap();
        preview.key(KeyCode::Char('l'));
        preview.key(KeyCode::Char('i'));
        let mut screen = Vec::new();
        preview.draw(&mut screen, (50, 19)).unwrap();
        let screen = String::from_utf8(screen).unwrap();
        assert!(screen.contains("선택 배경 · #E8EDE4 · surface"));
        assert!(screen.contains("↳ windows-terminal.toml"));
        assert!(!screen.contains("[L]"));
        assert!(!screen.contains("Python"));
        preview.key(KeyCode::Tab);
        let mut screen = Vec::new();
        preview.draw(&mut screen, (50, 19)).unwrap();
        let screen = String::from_utf8(screen).unwrap();
        assert!(screen.contains("적용 근거 · Python"));
        assert!(screen.ends_with("[Tab]앱 [L]언어 [M]모드 [I]근거 [R]갱신 [Q]종료"));
    }

    #[test]
    fn reload_keeps_last_good_samples_until_saved_source_is_valid() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("theme.toml");
        fs::write(&source, SOURCE).unwrap();
        let mut preview = Preview::open(&source, &[App::Typora]).unwrap();
        fs::write(&source, "not valid TOML").unwrap();
        preview.key(KeyCode::Char('r'));
        assert!(preview.status.contains("다시 읽기 실패"));
        assert!(render(&preview).contains("프로젝트 노트"));
        let changed = SOURCE.replace("#246958", "#205C4F");
        fs::write(&source, &changed).unwrap();
        preview.key(KeyCode::Char('r'));
        assert_eq!(preview.theme.light.roles["link"], "#205C4F");
        assert_eq!(fs::read_to_string(source).unwrap(), changed);
    }
}
