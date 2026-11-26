use crate::{
    helix, iterm2, lbrx_studio, macos_terminal,
    model::Theme,
    obsidian,
    resolve::{self, ResolvedTheme},
    typora, vim, vscode, windows_terminal,
};
use std::{
    fs::{self, File, Metadata},
    io::{self, Write},
    path::{Path, PathBuf},
};

pub const OUTPUT: &str = "lbrx";
pub const GUIDE: &str = "README.md";
const STAGING: &str = ".lbrx-build";
const BACKUP: &str = ".lbrx-previous";
const LOCK: &str = ".lbrx-build.lock";
const TOKENS: &str = "resolved.toml";
type AdapterOutput = (App, Vec<(&'static str, Vec<u8>)>);

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum App {
    WindowsTerminal,
    MacosTerminal,
    Typora,
    Obsidian,
    #[value(name = "iterm2")]
    Iterm2,
    Vim,
    #[value(name = "vscode")]
    VsCode,
    Helix,
    LbrxStudio,
}

impl App {
    pub const ALL: [Self; 9] = [
        Self::WindowsTerminal,
        Self::MacosTerminal,
        Self::Typora,
        Self::Obsidian,
        Self::Iterm2,
        Self::Vim,
        Self::VsCode,
        Self::Helix,
        Self::LbrxStudio,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::WindowsTerminal => "Windows Terminal",
            Self::MacosTerminal => "macOS Terminal.app",
            Self::Typora => "Typora",
            Self::Obsidian => "Obsidian",
            Self::Iterm2 => "iTerm2",
            Self::Vim => "Vim",
            Self::VsCode => "VS Code",
            Self::Helix => "Helix",
            Self::LbrxStudio => "LBRX Studio",
        }
    }

    pub fn directory(self) -> &'static str {
        self.output().0
    }

    pub fn files(self) -> &'static [&'static str] {
        self.output().1
    }

    pub fn output_path(self, parent: &Path) -> PathBuf {
        Path::new(self.directory())
            .components()
            .fold(parent.join(OUTPUT), |path, component| path.join(component))
    }

    fn output(self) -> (&'static str, &'static [&'static str]) {
        match self {
            Self::WindowsTerminal => (windows_terminal::DIRECTORY, &windows_terminal::FILES),
            Self::MacosTerminal => (macos_terminal::DIRECTORY, &macos_terminal::FILES),
            Self::Typora => (typora::DIRECTORY, typora::FILES),
            Self::Obsidian => (obsidian::DIRECTORY, obsidian::FILES),
            Self::Iterm2 => (iterm2::DIRECTORY, &iterm2::FILES),
            Self::Vim => (vim::DIRECTORY, vim::FILES),
            Self::VsCode => (vscode::DIRECTORY, vscode::FILES),
            Self::Helix => (helix::DIRECTORY, helix::FILES),
            Self::LbrxStudio => (lbrx_studio::DIRECTORY, lbrx_studio::FILES),
        }
    }

    fn render(self, theme: &ResolvedTheme) -> Result<AdapterOutput, String> {
        let files: Vec<(&str, String)> = match self {
            Self::WindowsTerminal => windows_terminal::render(theme)?.into(),
            Self::MacosTerminal => macos_terminal::render(theme)?.into(),
            Self::Typora => typora::render(theme).into(),
            Self::Obsidian => obsidian::render(theme).into(),
            Self::Iterm2 => iterm2::render(theme)?.into(),
            Self::Vim => vim::render(theme).into(),
            Self::VsCode => {
                let mut files = vscode::render(theme)?;
                if let Some((_, text)) = files.iter_mut().find(|(name, _)| *name == GUIDE) {
                    text.extend_from_slice(self.language_guide(theme).as_bytes());
                }
                return Ok((self, files.into()));
            }
            Self::Helix => helix::render(theme)?.into(),
            Self::LbrxStudio => lbrx_studio::render(theme).into(),
        };
        Ok((
            self,
            files
                .into_iter()
                .map(|(name, mut text)| {
                    if name == GUIDE {
                        text.push_str(&self.language_guide(theme));
                    }
                    (name, text.into_bytes())
                })
                .collect(),
        ))
    }

    fn language_guide(self, theme: &ResolvedTheme) -> String {
        if matches!(
            self,
            Self::WindowsTerminal | Self::MacosTerminal | Self::Iterm2
        ) || theme.layers.languages.is_empty()
        {
            return String::new();
        }
        let mut text = String::from(
            "\n\n## 구문 강조 설정 목록\n\n이 목록은 빌드에 입력된 언어 TOML과 적용 경로를 나타냅니다. 테마는 문법을 설치하지 않으며, 실제 분류는 앱과 설치된 문법에 따릅니다.\n\n| 설정 ID | 적용 경로 |\n|---|---|\n",
        );
        for id in theme.language_ids() {
            let route = if id == "text" {
                "일반 전경색, 별도 구문 없음"
            } else {
                match self {
                    Self::VsCode if !vscode::grammar_roots(id).is_empty() => "언어별 TextMate 규칙",
                    Self::VsCode if id == "bash" => "`shell` 공유 문법",
                    Self::VsCode => "공통 scope만 적용 가능; 문법 확장 필요 시 별도 설치",
                    Self::Vim if vim::has_language_groups(id, theme) => "언어별 Vim 문법 그룹",
                    Self::Vim => "공통 Vim 문법 그룹",
                    Self::Helix => "공통 Tree-sitter 캡처; 언어별 색 재정의 미적용",
                    Self::LbrxStudio => "HTML 토큰 클래스; Studio 렌더러의 토큰 분류 필요",
                    Self::Typora => "코드 펜스 CodeMirror 클래스가 있는 경우만",
                    Self::Obsidian => "읽기 뷰 Prism 클래스가 있는 경우만",
                    Self::WindowsTerminal | Self::MacosTerminal | Self::Iterm2 => unreachable!(),
                }
            };
            text.push_str(&format!("| `{id}` | {route} |\n"));
        }
        text
    }
}

pub fn build(theme: &Theme, parent: &Path, apps: &[App]) -> Result<(), String> {
    if apps.is_empty() {
        return Err("생성할 앱을 하나 이상 선택하세요".into());
    }
    let resolved = resolve::resolve(theme)?;
    let contents = toml::to_string_pretty(&resolved).map_err(|e| e.to_string())?;
    let adapters = App::ALL
        .into_iter()
        .filter(|app| apps.contains(app))
        .map(|app| app.render(&resolved))
        .collect::<Result<Vec<_>, _>>()?;
    if !exists(parent)? {
        fs::create_dir(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    require_directory(parent)?;
    let lock_path = parent.join(LOCK);
    let lock = File::create_new(&lock_path).map_err(|e| {
        format!(
            "{}: 빌드 잠금 실패 (다른 빌드·이전 중단 흔적 확인): {e}",
            lock_path.display()
        )
    })?;
    let result = build_locked(parent, &resolved, &contents, &adapters);
    drop(lock);
    match fs::remove_file(&lock_path) {
        Ok(()) => result,
        Err(error) => Err(format!(
            "{}{}: 빌드 잠금 정리 실패: {error}",
            result
                .err()
                .map(|reason| format!("{reason}; "))
                .unwrap_or_default(),
            lock_path.display()
        )),
    }
}

fn build_locked(
    parent: &Path,
    resolved: &ResolvedTheme,
    contents: &str,
    adapters: &[AdapterOutput],
) -> Result<(), String> {
    let staging = parent.join(STAGING);
    fs::create_dir(&staging).map_err(|e| {
        format!(
            "{}: 임시 출력 예약 실패 (다른 빌드·이전 중단 흔적 확인): {e}",
            staging.display()
        )
    })?;
    let result = stage_and_publish(parent, resolved, contents, adapters);
    let cleanup = remove_generated_directory(&staging);
    match (result, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(reason), Ok(())) => Err(reason),
        (result, Err(error)) => Err(format!(
            "{}{}: 임시 출력 정리 실패: {error}",
            result
                .err()
                .map(|reason| format!("{reason}; "))
                .unwrap_or_default(),
            staging.display()
        )),
    }
}

fn stage_and_publish(
    parent: &Path,
    resolved: &ResolvedTheme,
    contents: &str,
    adapters: &[AdapterOutput],
) -> Result<(), String> {
    let staging = parent.join(STAGING);
    let path = staging.join(TOKENS);
    let written = write_staged(&path, contents.as_bytes())?;
    let text = std::str::from_utf8(&written).map_err(|e| format!("{}: {e}", path.display()))?;
    let parsed: ResolvedTheme =
        toml::from_str(text).map_err(|e| format!("{}: 출력 재파싱 실패: {e}", path.display()))?;
    if parsed != *resolved {
        return Err(format!("{}: 해석 결과와 출력이 다름", path.display()));
    }
    for (app, files) in adapters {
        let directory = staging.join(app.directory());
        fs::create_dir_all(&directory).map_err(|e| format!("{}: {e}", directory.display()))?;
        for (name, contents) in files {
            let path = directory.join(name);
            let written = write_staged(&path, contents)?;
            if name.ends_with(".json") {
                serde_json::from_slice::<serde_json::Value>(&written)
                    .map_err(|e| format!("{}: 출력 재파싱 실패: {e}", path.display()))?;
            } else if name.ends_with(".itermcolors") {
                plist::Value::from_reader_xml(written.as_slice())
                    .map_err(|e| format!("{}: 출력 재파싱 실패: {e}", path.display()))?;
            } else if name.ends_with(".vsix") {
                vscode::validate(&written)
                    .map_err(|e| format!("{}: VSIX 검증 실패: {e}", path.display()))?;
            } else if name.ends_with(".toml") {
                let text = std::str::from_utf8(&written)
                    .map_err(|e| format!("{}: {e}", path.display()))?;
                toml::from_str::<toml::Table>(text)
                    .map_err(|e| format!("{}: 출력 재파싱 실패: {e}", path.display()))?;
            }
        }
    }
    if adapters.len() < App::ALL.len() {
        preserve_unselected(parent, &staging, adapters)?;
    }
    publish(parent)
}

fn preserve_unselected(
    parent: &Path,
    staging: &Path,
    adapters: &[AdapterOutput],
) -> Result<(), String> {
    let output = parent.join(OUTPUT);
    if !exists(&output)? {
        return Ok(());
    }
    require_generated_directory(&output)?;
    for app in App::ALL {
        if adapters.iter().any(|(selected, _)| *selected == app) {
            continue;
        }
        let (directory, files) = app.output();
        for name in files {
            let source = output.join(directory).join(name);
            if !exists(&source)? {
                continue;
            }
            let destination = staging.join(directory).join(name);
            fs::create_dir_all(staging.join(directory))
                .and_then(|()| fs::copy(&source, &destination).map(|_| ()))
                .map_err(|e| format!("{}: 미선택 앱 출력 보존 실패: {e}", source.display()))?;
        }
    }
    Ok(())
}

fn write_staged(path: &Path, contents: &[u8]) -> Result<Vec<u8>, String> {
    let write_and_read = || -> io::Result<Vec<u8>> {
        let mut file = File::create_new(path)?;
        file.write_all(contents)?;
        file.sync_all()?;
        drop(file);
        fs::read(path)
    };
    let written =
        write_and_read().map_err(|e| format!("{}: 임시 출력 실패: {e}", path.display()))?;
    if written != contents {
        return Err(format!("{}: 읽은 출력이 생성 내용과 다름", path.display()));
    }
    Ok(written)
}

fn publish(parent: &Path) -> Result<(), String> {
    let output = parent.join(OUTPUT);
    let staging = parent.join(STAGING);
    let backup = parent.join(BACKUP);
    if exists(&backup)? {
        return Err(format!(
            "{}: 이전 백업이 남아 있어 교체 중단; 수동 복구 필요",
            backup.display()
        ));
    }
    let replacing = exists(&output)?;
    if replacing {
        require_generated_directory(&output)?;
        fs::rename(&output, &backup)
            .map_err(|e| format!("{}: 기존 출력 백업 실패: {e}", output.display()))?;
    }
    if let Err(error) = fs::rename(&staging, &output) {
        if replacing {
            fs::rename(&backup, &output).map_err(|rollback| {
                format!(
                    "{}: 출력 교체 실패: {error}; 복구 실패: {rollback}; 이전 출력은 {}에 보존됨",
                    output.display(),
                    backup.display()
                )
            })?;
        }
        return Err(format!("{}: 출력 교체 실패: {error}", output.display()));
    }
    if replacing {
        remove_generated_directory(&backup).map_err(|e| {
            format!(
                "{}: 새 출력은 게시됐지만 이전 백업 정리 실패: {e}",
                backup.display()
            )
        })?;
    }
    Ok(())
}

fn exists(path: &Path) -> Result<bool, String> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

fn require_directory(path: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if !plain_directory(&metadata) {
        return Err(format!("{}: 링크가 아닌 디렉터리가 필요", path.display()));
    }
    Ok(())
}

fn require_generated_directory(path: &Path) -> Result<(), String> {
    require_directory(path)?;
    check_tree(path).map_err(|e| format!("{}: 교체 거부: {e}", path.display()))?;
    let file = path.join(TOKENS);
    let contents = fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
    toml::from_str::<ResolvedTheme>(&contents)
        .map_err(|e| format!("{}: 기존 생성물 형식 확인 실패: {e}", file.display()))?;
    Ok(())
}

fn remove_generated_directory(path: &Path) -> io::Result<()> {
    check_tree(path)?;
    for (directory, files) in App::ALL.map(App::output) {
        for name in files {
            ignore_missing(fs::remove_file(path.join(directory).join(name)))?;
        }
        for dir in Path::new(directory)
            .ancestors()
            .take_while(|p| !p.as_os_str().is_empty())
        {
            ignore_missing(fs::remove_dir(path.join(dir)))?;
        }
    }
    ignore_missing(fs::remove_file(path.join(TOKENS)))?;
    ignore_missing(fs::remove_dir(path))
}

fn check_tree(path: &Path) -> io::Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if !plain_directory(&metadata) {
        return Err(unexpected(path));
    }
    let allowed: Vec<_> = std::iter::once(path.join(TOKENS))
        .chain(
            App::ALL
                .map(App::output)
                .into_iter()
                .flat_map(|(dir, files)| files.iter().map(move |file| path.join(dir).join(file))),
        )
        .collect();
    check_entries(path, &allowed)
}

fn check_entries(path: &Path, allowed: &[std::path::PathBuf]) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        let metadata = fs::symlink_metadata(&child)?;
        if metadata.is_file() && allowed.contains(&child) {
            continue;
        }
        if plain_directory(&metadata)
            && allowed
                .iter()
                .any(|file| file != &child && file.starts_with(&child))
        {
            check_entries(&child, allowed)?;
        } else {
            return Err(unexpected(&child));
        }
    }
    Ok(())
}

fn plain_directory(metadata: &Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return false;
        }
    }
    metadata.is_dir()
}

fn unexpected(path: &Path) -> io::Error {
    io::Error::other(format!("{}: 알 수 없는 파일·디렉터리·링크", path.display()))
}

fn ignore_missing(result: io::Result<()>) -> io::Result<()> {
    match result {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    fn build(theme: &Theme, parent: &Path) -> Result<(), String> {
        super::build(theme, parent, &App::ALL)
    }

    #[test]
    fn selected_build_only_generates_requested_apps_and_preserves_other_outputs() {
        let workspace = Workspace::new();
        let mut theme = sample();
        let output = workspace.0.join(OUTPUT);
        assert!(super::build(&theme, &workspace.0, &[]).is_err());
        assert!(!output.exists());

        super::build(&theme, &workspace.0, &[App::Obsidian]).unwrap();
        assert!(workspace.output().is_file());
        assert!(output.join(App::Obsidian.directory()).is_dir());
        for app in [App::WindowsTerminal, App::Typora] {
            assert!(!output.join(app.directory()).exists());
        }

        build(&theme, &workspace.0).unwrap();
        let snapshot = |app: App| {
            app.output()
                .1
                .iter()
                .map(|file| fs::read(output.join(app.directory()).join(file)).unwrap())
                .collect::<Vec<_>>()
        };
        let terminal = snapshot(App::WindowsTerminal);
        let typora = snapshot(App::Typora);
        let obsidian = snapshot(App::Obsidian);
        theme.dark.palette.insert("orange".into(), "#FFD1A3".into());
        super::build(&theme, &workspace.0, &[App::Obsidian]).unwrap();
        assert_eq!(snapshot(App::WindowsTerminal), terminal);
        assert_eq!(snapshot(App::Typora), typora);
        assert_ne!(snapshot(App::Obsidian), obsidian);
        assert!(
            fs::read_to_string(workspace.output())
                .unwrap()
                .contains("#FFD1A3")
        );

        super::build(&theme, &workspace.0, &[App::WindowsTerminal, App::Typora]).unwrap();
        assert_ne!(snapshot(App::WindowsTerminal), terminal);
        assert_ne!(snapshot(App::Typora), typora);
    }

    struct Workspace(std::path::PathBuf);

    impl Workspace {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "lbrx-build-test-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }

        fn output(&self) -> std::path::PathBuf {
            self.0.join(OUTPUT).join(TOKENS)
        }
    }

    impl Drop for Workspace {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn sample() -> Theme {
        toml::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/lbrx.toml"
        )))
        .unwrap()
    }

    #[test]
    fn output_is_resolved_deterministic_and_has_only_expected_files() {
        let workspace = Workspace::new();
        let theme = sample();
        build(&theme, &workspace.0).unwrap();
        let first = fs::read(workspace.output()).unwrap();
        let terminal = workspace.0.join(OUTPUT).join(windows_terminal::DIRECTORY);
        let adapter_contents =
            windows_terminal::FILES.map(|file| fs::read(terminal.join(file)).unwrap());
        let tokens: ResolvedTheme = toml::from_str(std::str::from_utf8(&first).unwrap()).unwrap();
        assert_eq!(tokens.light.roles["link"], theme.light.palette["orange"]);
        for (index, color) in tokens.dark.ansi.iter().enumerate() {
            assert_eq!(
                color,
                &theme.dark.palette[&theme.dark.ansi[&index.to_string()]]
            );
        }
        assert_eq!(tokens.typography, theme.typography);
        build(&theme, &workspace.0).unwrap();
        assert_eq!(fs::read(workspace.output()).unwrap(), first);
        assert_eq!(
            windows_terminal::FILES.map(|file| fs::read(terminal.join(file)).unwrap()),
            adapter_contents
        );
        assert_eq!(fs::read_dir(&workspace.0).unwrap().count(), 1);
        assert_eq!(
            fs::read_dir(workspace.0.join(OUTPUT)).unwrap().count(),
            App::ALL.len() + 1
        );
    }

    #[test]
    fn studio_exports_ui_fonts_and_language_tokens_without_changing_other_apps() {
        let workspace = Workspace::new();
        let mut theme = sample();
        theme
            .layers
            .common
            .insert("function".into(), "keyword".into());
        theme
            .layers
            .languages
            .insert("python".into(), [("function".into(), "text".into())].into());
        theme.layers.language_styles.insert(
            "python".into(),
            [("function".into(), crate::layers::TokenStyle::Italic)].into(),
        );
        theme.typography.prose.size_px = 18.0;
        theme.typography.mono.line_height = 1.6;
        super::build(&theme, &workspace.0, &[App::VsCode, App::LbrxStudio]).unwrap();
        let vscode_dir = App::VsCode.output_path(&workspace.0);
        let vscode_before = vscode::FILES
            .iter()
            .map(|name| fs::read(vscode_dir.join(name)).unwrap())
            .collect::<Vec<_>>();
        let studio_dir = App::LbrxStudio.output_path(&workspace.0);
        let read = || fs::read_to_string(studio_dir.join("theme.css")).unwrap();
        let baseline = read();
        assert!(baseline.contains("--border: #6D5A4B40;"));
        assert!(baseline.contains("--border: #C0AC9B40;"));
        assert!(baseline.contains("--font-prose-size: 18px;"));
        assert!(baseline.contains("--font-mono-line-height: 1.6;"));
        assert!(baseline.contains("--font-mono: \"D2Coding ligature\", monospace;"));
        assert!(baseline.contains("--lbrx-code-function: #344F85;"));
        assert!(baseline.contains(
            ":root .lbrx-code.language-python { --lbrx-code-function: #38291F; --lbrx-code-function-weight: normal; --lbrx-code-function-style: italic; }"
        ));
        assert!(baseline.contains(
            ":root[data-theme=\"dark\"] .lbrx-code.language-python { --lbrx-code-function: #F4E6D8;"
        ));
        assert!(baseline.contains(
            "@media (prefers-color-scheme: dark) {\n:root:not([data-theme=\"light\"]):not([data-theme=\"dark\"])"
        ));
        theme.layers.editors.insert(
            "lbrx-studio".into(),
            [("--accent".into(), "function".into())].into(),
        );
        super::build(&theme, &workspace.0, &[App::LbrxStudio]).unwrap();
        let customized = read();
        assert!(customized.contains("--accent: #245E8A;"));
        assert!(customized.contains("--accent: #B4D9F5;"));
        assert_eq!(
            vscode_before,
            vscode::FILES
                .iter()
                .map(|name| fs::read(vscode_dir.join(name)).unwrap())
                .collect::<Vec<_>>()
        );
        super::build(&theme, &workspace.0, &[App::LbrxStudio]).unwrap();
        assert_eq!(read(), customized);
        theme.dark.roles.insert("accent".into(), "ink".into());
        super::build(&theme, &workspace.0, &[App::LbrxStudio]).unwrap();
        assert_eq!(read(), customized);
    }

    #[test]
    fn helix_build_exports_separate_modes_and_preserves_other_apps() {
        let workspace = Workspace::new();
        let mut theme = sample();
        let dir = App::Helix.output_path(&workspace.0);
        let vim_dir = App::Vim.output_path(&workspace.0);
        super::build(&theme, &workspace.0, &[App::Vim]).unwrap();
        let vim = fs::read(vim_dir.join(vim::FILES[1])).unwrap();
        assert!(!dir.exists());
        super::build(&theme, &workspace.0, &[App::Helix]).unwrap();
        let light_path = dir.join(helix::FILES[0]);
        let dark_path = dir.join(helix::FILES[1]);
        let light = fs::read_to_string(&light_path).unwrap();
        let resolved = resolve::resolve(&theme).unwrap();
        for (path, mode) in [(&light_path, &resolved.light), (&dark_path, &resolved.dark)] {
            let table: toml::Table = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
            assert_eq!(
                table["ui.background"]["bg"].as_str(),
                Some(mode.roles["background"].as_str())
            );
            assert_eq!(
                table["ui.selection.primary"]["fg"].as_str(),
                Some(mode.roles["selection_text"].as_str())
            );
            assert_eq!(
                table["diagnostic.warning"]["underline"]["color"].as_str(),
                Some(mode.roles["warning"].as_str())
            );
            assert!(!table.contains_key("comment.block"));
        }
        theme.dark.roles.insert("function".into(), "ink".into());
        super::build(&theme, &workspace.0, &[App::Helix]).unwrap();
        assert_eq!(fs::read_to_string(light_path).unwrap(), light);
        let updated: toml::Table = toml::from_str(&fs::read_to_string(dark_path).unwrap()).unwrap();
        assert_eq!(
            updated["function"].as_str(),
            Some(resolved.dark.roles["text"].as_str())
        );
        assert_eq!(fs::read(vim_dir.join(vim::FILES[1])).unwrap(), vim);
    }

    #[test]
    fn helix_exports_native_settings_without_changing_either_theme() {
        let workspace = Workspace::new();
        let input = tempfile::tempdir().unwrap();
        let source = input.path().join("lbrx.toml");
        let editors = input.path().join("editors");
        fs::create_dir(&editors).unwrap();
        fs::write(&source, include_str!("../tests/fixtures/lbrx.toml")).unwrap();
        let settings = editors.join("helix.toml");
        let stock = r#"
[colors]
"ui.linenr" = "muted"
"ui.linenr.selected" = "text"
"#;
        fs::write(&settings, stock).unwrap();
        let dir = App::Helix.output_path(&workspace.0);
        let read = |name| fs::read_to_string(dir.join(name)).unwrap();
        let parse = |name| toml::from_str::<toml::Table>(&read(name)).unwrap();
        super::build(
            &crate::layers::load(&source).unwrap(),
            &workspace.0,
            &[App::Helix],
        )
        .unwrap();
        let baseline = [read("lbrx_light.toml"), read("lbrx_dark.toml")];
        assert!(parse("config.toml").is_empty());
        assert!(parse("languages.toml").is_empty());

        let languages = input.path().join("languages");
        fs::create_dir(&languages).unwrap();
        for id in ["rust", "csharp", "bash", "shell", "text"] {
            fs::write(languages.join(format!("{id}.toml")), "[tokens]\n").unwrap();
        }
        let native = r#"
[config]
theme = "lbrx_dark"
[config.editor]
line-number = "relative"
[config.keys.normal]
C-s = ":write"
[[languages.language]]
name = "rust"
auto-format = false
indent = { tab-width = 8 }
[[languages.language]]
name = "toml"
auto-format = true
[languages.language-server.rust-analyzer.config.check]
command = "clippy"
"#;
        fs::write(&settings, format!("{stock}\n{native}")).unwrap();
        super::build(
            &crate::layers::load(&source).unwrap(),
            &workspace.0,
            &[App::Helix],
        )
        .unwrap();
        let expected: toml::Table = toml::from_str(native).unwrap();
        assert_eq!(
            parse("config.toml"),
            *expected["config"].as_table().unwrap()
        );
        let output = parse("languages.toml");
        assert_eq!(
            output["language-server"],
            expected["languages"]["language-server"]
        );
        let entries = output["language"].as_array().unwrap();
        let names: Vec<_> = entries
            .iter()
            .map(|entry| entry["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["rust", "toml", "bash", "c-sharp"]);
        let rust = &entries[0];
        assert_eq!(rust["indent"]["tab-width"].as_integer(), Some(8));
        assert_eq!(rust["indent"]["unit"].as_str(), Some("    "));
        assert_eq!(rust["auto-format"].as_bool(), Some(false));
        assert_eq!(
            rust["comment-tokens"].as_array().unwrap()[2].as_str(),
            Some("//!")
        );
        assert_eq!(entries[3]["comment-token"].as_str(), Some("//"));
        assert_eq!([read("lbrx_light.toml"), read("lbrx_dark.toml")], baseline);
    }

    #[test]
    fn vim_build_keeps_light_bytes_when_a_dark_code_role_changes() {
        let workspace = Workspace::new();
        let mut theme = sample();
        theme
            .layers
            .common
            .insert("function.macro".into(), "syntax_macro".into());
        let dir = App::Vim.output_path(&workspace.0);
        super::build(&theme, &workspace.0, &[App::Vim]).unwrap();
        let light = fs::read(dir.join(vim::FILES[0])).unwrap();
        let dark = fs::read_to_string(dir.join(vim::FILES[1])).unwrap();
        assert_eq!(fs::read_dir(&dir).unwrap().count(), vim::FILES.len());
        assert!(dark.contains("highlight! link Float Number\n"));
        theme.dark.roles.insert("function".into(), "paper".into());
        assert!(super::build(&theme, &workspace.0, &[App::Vim]).is_err());
        theme.dark.roles.insert("function".into(), "ink".into());
        super::build(&theme, &workspace.0, &[App::Vim]).unwrap();
        assert_eq!(fs::read(dir.join(vim::FILES[0])).unwrap(), light);
        let after = fs::read_to_string(dir.join(vim::FILES[1])).unwrap();
        let changed: Vec<_> = dark
            .lines()
            .zip(after.lines())
            .filter(|(a, b)| a != b)
            .collect();
        assert_eq!(changed.len(), 1);
        assert!(
            changed[0]
                .1
                .starts_with("highlight Function guifg=#F4E6D8 guibg=NONE ctermfg=224 ")
        );
    }

    #[test]
    fn vim_share_syntax_and_ui_roles_without_overriding_ignore() {
        let theme = resolve::resolve(&sample()).unwrap();
        let output = vim::render(&theme);
        for ((_, script), mode) in output.iter().zip([&theme.light, &theme.dark]) {
            let group = |name| {
                let prefix = format!("highlight {name} ");
                script
                    .lines()
                    .find_map(|line| line.strip_prefix(&prefix))
                    .unwrap()
            };
            let macro_color = theme.token(mode, "", "function.macro");
            assert!(group("Macro").starts_with(&format!("guifg={macro_color} ctermfg=")));
            let menu = group("PmenuKindSel");
            assert!(menu.starts_with(&format!(
                "guifg={} guibg={} ctermfg=",
                mode.roles["selection_text"], mode.roles["selection_background"]
            )));
            assert!(menu.ends_with(" gui=bold cterm=bold"));
            assert!(group("DiffAdd").starts_with("guifg=NONE "));
            assert!(group("DiffChange").starts_with("guifg=NONE "));
            assert_ne!(group("DiffAdd"), group("DiffChange"));
            assert!(group("Underlined").ends_with(" gui=underline cterm=underline"));
            assert!(script.contains("highlight! link Debug Function\n"));
            assert!(
                !script
                    .lines()
                    .any(|line| line.split_whitespace().any(|word| word == "Ignore"))
            );
        }
    }

    #[test]
    fn iterm2_presets_map_srgb_and_all_ansi_slots_with_dark_changes_isolated() {
        let workspace = Workspace::new();
        let mut theme = sample();
        let directory = App::Iterm2.output_path(&workspace.0);
        let read = |index: usize| fs::read(directory.join(iterm2::FILES[index])).unwrap();
        super::build(&theme, &workspace.0, &[App::Iterm2]).unwrap();
        assert_eq!(
            fs::read_dir(&directory).unwrap().count(),
            iterm2::FILES.len()
        );
        assert!(!App::WindowsTerminal.output_path(&workspace.0).exists());
        let light = read(0);
        let dark = read(1);
        let resolved = resolve::resolve(&theme).unwrap();
        let parse = |data: &[u8]| {
            plist::Value::from_reader_xml(data)
                .unwrap()
                .into_dictionary()
                .unwrap()
        };
        for (data, mode) in [(&light, &resolved.light), (&dark, &resolved.dark)] {
            let colors = parse(data);
            assert_eq!(colors.len(), 21);
            let check = |name: &str, hex: &str| {
                let color = colors[name].as_dictionary().unwrap();
                assert_eq!(color["Color Space"].as_string(), Some("sRGB"));
                assert_eq!(color["Alpha Component"].as_real(), Some(1.0));
                for (component, byte) in ["Red", "Green", "Blue"]
                    .into_iter()
                    .zip(resolve::parse_rgb(hex).unwrap())
                {
                    let actual = color[&format!("{component} Component")].as_real().unwrap();
                    assert!(
                        (actual - f64::from(byte) / 255.0).abs() < 1e-12,
                        "{name}: {component}"
                    );
                }
            };
            check("Foreground Color", &mode.roles["terminal_foreground"]);
            check("Background Color", &mode.roles["terminal_background"]);
            check("Cursor Color", &mode.roles["terminal_cursor"]);
            check("Selection Color", &mode.roles["selection_background"]);
            check("Selected Text Color", &mode.roles["selection_text"]);
            for (slot, hex) in mode.ansi.iter().enumerate() {
                check(&format!("Ansi {slot} Color"), hex);
            }
        }
        theme
            .dark
            .ansi
            .insert("2".into(), "ansi_bright_green".into());
        super::build(&theme, &workspace.0, &[App::Iterm2]).unwrap();
        assert_eq!(read(0), light);
        let mut expected = parse(&dark);
        expected.insert("Ansi 2 Color".into(), expected["Ansi 10 Color"].clone());
        assert_eq!(parse(&read(1)), expected);
    }

    #[test]
    fn build_bundles_install_guides_and_keeps_unselected_guides() {
        let workspace = Workspace::new();
        let theme = sample();
        build(&theme, &workspace.0).unwrap();
        let guides: Vec<_> = App::ALL
            .iter()
            .map(|app| {
                let path = app.output_path(&workspace.0).join(GUIDE);
                let text = fs::read_to_string(&path).unwrap();
                assert!(text.contains(app.label()), "{}", path.display());
                (path, text)
            })
            .collect();
        super::build(&theme, &workspace.0, &[App::Typora]).unwrap();
        for (path, text) in guides {
            assert_eq!(fs::read_to_string(path).unwrap(), text);
        }
    }

    #[test]
    fn document_selection_keeps_code_blue_and_checks_prose_contrast() {
        let mut theme = sample();
        let resolved = resolve::resolve(&theme).unwrap();
        let typora = typora::render(&resolved);
        let [_, (_, obsidian), ..] = obsidian::render(&resolved);
        for ((_, css), expected) in typora.iter().zip(["#FFE0BE", "#62452F"]) {
            let css = css.replace("\r\n", "\n");
            let prose = format!("--lbrx-prose-selection-background: {expected};");
            assert!(css.contains(&prose));
            assert!(obsidian.contains(&prose));
            assert!(css.contains(
                "#write *::selection {\n  background-color: var(--lbrx-prose-selection-background);"
            ));
            assert!(css.contains("#write :is(code, .md-fences) *::selection {\n  background-color: var(--lbrx-selection-background);"));
        }
        assert!(obsidian.contains("--text-selection: #E0EDFC;"));
        assert!(obsidian.contains("--text-selection: #3C4A61;"));
        assert!(obsidian.contains(".markdown-rendered :is(pre, code) *::selection,"));
        theme
            .dark
            .roles
            .insert("prose_selection_background".into(), "ink".into());
        assert!(
            resolve::check(&theme)
                .unwrap_err()
                .contains("prose_selection_background")
        );
    }

    #[test]
    fn document_adapters_scope_the_code_block_background_and_validate_contrast() {
        let mut theme = sample();
        let resolved = resolve::resolve(&theme).unwrap();
        for ((_, css), hex) in typora::render(&resolved)
            .iter()
            .take(2)
            .zip(["#FFEAD6", "#38313A"])
        {
            assert!(css.contains(&format!("--lbrx-code-block-background: {hex};")));
            assert!(css.replace("\r\n", "\n").contains(".md-fences .CodeMirror-gutters) {\n  background-color: var(--lbrx-code-block-background);"));
        }
        let [_, (_, css), ..] = obsidian::render(&resolved);
        assert!(css.contains("--code-background: #FFF8F0;"));
        assert!(css.contains("--code-background: #28242A;"));
        assert!(css.contains(".markdown-rendered pre,\n.markdown-source-view.is-live-preview .HyperMD-codeblock {\n  --code-background: var(--lbrx-code-block-background);"));

        theme
            .light
            .roles
            .insert("code_block_background".into(), "ink".into());
        assert!(
            resolve::check(&theme)
                .unwrap_err()
                .contains("code_block_background")
        );
        theme.light.roles.remove("code_block_background");
        theme.dark.roles.remove("code_block_background");
        assert_eq!(
            resolve::resolve(&theme)
                .unwrap()
                .light
                .code_block_background(),
            "#FFF8F0"
        );
    }

    #[test]
    fn obsidian_package_maps_roles_and_keeps_dark_changes_isolated() {
        let workspace = Workspace::new();
        let mut theme = sample();
        let dir = workspace.0.join(OUTPUT).join(obsidian::DIRECTORY);
        build(&theme, &workspace.0).unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(dir.join("manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["name"], dir.file_name().unwrap().to_str().unwrap());
        assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));
        let css = || fs::read_to_string(dir.join("theme.css")).unwrap();
        let before = css();
        let (light, dark) = before.split_once(".theme-dark {").unwrap();
        for (block, mode) in [(light, &theme.light), (dark, &theme.dark)] {
            for (var, role) in [
                ("background-primary", "background"),
                ("text-selection", "selection_background"),
                ("code-function", "function"),
                ("link-color", "link"),
            ] {
                assert!(block.contains(&format!("--{var}: {};", mode.palette[&mode.roles[role]])));
            }
        }
        theme.dark.roles.insert("function".into(), "ink".into());
        build(&theme, &workspace.0).unwrap();
        assert_eq!(
            css(),
            format!(
                "{light}.theme-dark {{{}",
                dark.replace(
                    &format!("--code-function: {};", theme.dark.palette["slate"]),
                    &format!("--code-function: {};", theme.dark.palette["ink"])
                )
            )
        );
        fs::write(dir.join("note.md"), "keep").unwrap();
        assert!(
            build(&theme, &workspace.0)
                .unwrap_err()
                .contains("교체 거부")
        );
        assert_eq!(fs::read_to_string(dir.join("note.md")).unwrap(), "keep");
    }

    #[test]
    fn obsidian_accent_follows_each_modes_accent_role_independently_of_links() {
        let mut theme = sample();
        let render = |theme: &Theme| {
            obsidian::render(&resolve::resolve(theme).unwrap())[1]
                .1
                .clone()
        };
        let before = render(&theme);
        let (light, dark) = before.split_once(".theme-dark {").unwrap();
        assert!(light.contains("--accent-h: 21.78082192;"));
        assert!(light.contains("--accent-s: 76.84210526%;"));
        assert!(light.contains("--accent-l: 37.25490196%;"));
        assert!(dark.contains("--accent-h: 27.73584906;"));
        assert!(dark.contains("--accent-s: 100.00000000%;"));
        assert!(dark.contains("--accent-l: 79.21568627%;"));

        theme
            .dark
            .palette
            .insert("test_accent".into(), "#808080".into());
        theme
            .light
            .palette
            .insert("test_accent".into(), "#808080".into());
        theme
            .dark
            .roles
            .insert("accent".into(), "test_accent".into());
        let after = render(&theme);
        let expected = dark
            .replace("--accent-h: 27.73584906;", "--accent-h: 0.00000000;")
            .replace("--accent-s: 100.00000000%;", "--accent-s: 0.00000000%;")
            .replace("--accent-l: 79.21568627%;", "--accent-l: 50.19607843%;");
        assert_eq!(after, format!("{light}.theme-dark {{{expected}"));
    }

    #[test]
    fn typora_preserves_font_priority_and_only_updates_changed_mode() {
        let workspace = Workspace::new();
        let mut theme = sample();
        theme.typography.prose.families =
            ["English Font", "Korean Font", "Japanese Font", "sans-serif"]
                .map(String::from)
                .into();
        theme.typography.prose.line_height = 1.7;
        theme.typography.mono.families = ["Code Font", "monospace"].map(String::from).into();
        theme.typography.mono.size_px = 15.0;
        let output = workspace.0.join(OUTPUT);
        let typora = output.join(typora::DIRECTORY);
        let terminal = output.join(windows_terminal::DIRECTORY);
        build(&theme, &workspace.0).unwrap();
        let light = fs::read_to_string(typora.join(typora::LIGHT)).unwrap();
        let dark = fs::read_to_string(typora.join(typora::DARK)).unwrap();
        for css in [&light, &dark] {
            assert!(css.contains(r#"--lbrx-prose-family: "English Font", "Korean Font", "Japanese Font", sans-serif;"#));
            assert!(css.contains("--lbrx-prose-leading: 1.7;"));
            assert!(css.contains(r#"--lbrx-mono-family: "Code Font", monospace;"#));
            assert!(css.contains("--lbrx-mono-size: 15px;"));
        }
        let windows = fs::read(terminal.join(windows_terminal::SCHEMES)).unwrap();
        theme.dark.roles.insert("link".into(), "ink".into());
        let expected = dark.replace(
            &format!("--lbrx-link: {};", theme.dark.palette["orange"]),
            &format!("--lbrx-link: {};", theme.dark.palette["ink"]),
        );
        assert_ne!(dark, expected);
        build(&theme, &workspace.0).unwrap();
        assert_eq!(
            fs::read_to_string(typora.join(typora::LIGHT)).unwrap(),
            light
        );
        assert_eq!(
            fs::read_to_string(typora.join(typora::DARK)).unwrap(),
            expected
        );
        assert_eq!(
            fs::read(terminal.join(windows_terminal::SCHEMES)).unwrap(),
            windows
        );
    }

    #[test]
    fn typora_preferences_styles_target_the_affected_pane_in_each_mode() {
        let theme = sample();
        let resolved = resolve::resolve(&theme).unwrap();
        let [(_, light), (_, dark), _] = typora::render(&resolved);
        let content = ".ty-preferences .pane-group > .pane:not(.sidebar) {";
        assert!(!dark.contains(content));
        assert!(!light.contains(".ty-preferences .sidebar"));
        assert!(dark.contains(&format!(
            "--lbrx-surface: {};",
            resolved.dark.roles["surface"]
        )));
        for (css, selector, background, text) in [
            (
                &light,
                content,
                "var(--lbrx-background)",
                "var(--lbrx-text)",
            ),
            (
                &dark,
                ".ty-preferences .sidebar {",
                "var(--lbrx-surface)",
                "var(--lbrx-text)",
            ),
            (
                &dark,
                ".ty-preferences .sidebar .nav-group-item:is(:hover, :active, .active) {",
                "var(--lbrx-selection-background)",
                "var(--lbrx-selection-text)",
            ),
        ] {
            let (_, rest) = css.split_once(selector).unwrap();
            let (declarations, _) = rest.split_once('}').unwrap();
            assert!(
                declarations
                    .lines()
                    .any(|line| line.trim() == format!("background-color: {background};"))
            );
            assert!(
                declarations
                    .lines()
                    .any(|line| line.trim() == format!("color: {text};"))
            );
        }
    }

    #[test]
    fn terminal_scheme_mapping_and_dark_color_isolation() {
        let workspace = Workspace::new();
        let mut theme = sample();
        theme.typography.mono.families = vec!["Test Mono".into(), "monospace".into()];
        theme.typography.mono.size_px = 14.0;
        let terminal = workspace.0.join(OUTPUT).join(windows_terminal::DIRECTORY);
        let read_json = |name| -> serde_json::Value {
            serde_json::from_slice(&fs::read(terminal.join(name)).unwrap()).unwrap()
        };
        build(&theme, &workspace.0).unwrap();
        let before = read_json(windows_terminal::SCHEMES);
        let font = read_json(windows_terminal::FONT);
        assert_eq!(
            font,
            serde_json::json!({"font": {"face": "Test Mono", "size": 10.5}})
        );
        assert_eq!(before["schemes"].as_array().unwrap().len(), 2);
        for (index, (name, mode)) in [("LBRX Light", &theme.light), ("LBRX Dark", &theme.dark)]
            .into_iter()
            .enumerate()
        {
            let scheme = &before["schemes"][index];
            assert_eq!(scheme.as_object().unwrap().len(), 21);
            assert_eq!(scheme["name"], name);
            for (field, role) in [
                ("background", "terminal_background"),
                ("foreground", "terminal_foreground"),
                ("cursorColor", "terminal_cursor"),
                ("selectionBackground", "selection_background"),
            ] {
                assert_eq!(scheme[field], mode.palette[&mode.roles[role]]);
            }
            for (index, field) in [
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
            ]
            .into_iter()
            .enumerate()
            {
                assert_eq!(scheme[field], mode.palette[&mode.ansi[&index.to_string()]]);
            }
        }
        theme.dark.palette.insert("pine".into(), "#a2e3c6".into());
        build(&theme, &workspace.0).unwrap();
        let after = read_json(windows_terminal::SCHEMES);
        let mut expected = before;
        expected["schemes"][1]["green"] = "#A2E3C6".into();
        assert_eq!(after, expected);
        assert_eq!(read_json(windows_terminal::FONT), font);

        theme.typography.mono.size_px = 1.0;
        assert!(
            build(&theme, &workspace.0)
                .unwrap_err()
                .contains("typography.mono.size_px")
        );
        assert_eq!(read_json(windows_terminal::SCHEMES), after);
        assert_eq!(read_json(windows_terminal::FONT), font);
    }

    #[test]
    fn dark_palette_changes_only_referencing_dark_tokens() {
        let mut theme = sample();
        let before = resolve::resolve(&theme).unwrap();
        theme.dark.palette.insert("pine".into(), "#a2e3c6".into());
        let after = resolve::resolve(&theme).unwrap();
        assert_eq!(before.light, after.light);
        assert_eq!(before.typography, after.typography);
        for (key, value) in &before.dark.roles {
            let expected = if theme.dark.roles[key] == "pine" {
                "#A2E3C6"
            } else {
                value
            };
            assert_eq!(after.dark.roles[key], expected);
        }
        for (index, value) in before.dark.ansi.iter().enumerate() {
            let expected = if theme.dark.ansi[&index.to_string()] == "pine" {
                "#A2E3C6"
            } else {
                value
            };
            assert_eq!(after.dark.ansi[index], expected);
        }
    }

    #[test]
    fn invalid_source_preserves_previous_output_and_creates_nothing_on_first_build() {
        let workspace = Workspace::new();
        let mut theme = sample();
        theme.schema_version = 2;
        assert!(build(&theme, &workspace.0).is_err());
        assert_eq!(fs::read_dir(&workspace.0).unwrap().count(), 0);
        theme.schema_version = 1;
        build(&theme, &workspace.0).unwrap();
        let before = fs::read(workspace.output()).unwrap();
        theme.dark.ansi.remove("15");
        assert!(build(&theme, &workspace.0).is_err());
        assert_eq!(fs::read(workspace.output()).unwrap(), before);
    }

    #[test]
    fn unknown_output_files_are_preserved() {
        let workspace = Workspace::new();
        build(&sample(), &workspace.0).unwrap();
        let before = fs::read(workspace.output()).unwrap();
        let user_file = workspace
            .0
            .join(OUTPUT)
            .join(windows_terminal::DIRECTORY)
            .join("user.txt");
        fs::write(&user_file, "keep").unwrap();
        assert!(
            build(&sample(), &workspace.0)
                .unwrap_err()
                .contains("교체 거부")
        );
        assert_eq!(fs::read(workspace.output()).unwrap(), before);
        assert_eq!(fs::read_to_string(user_file).unwrap(), "keep");
        assert!(!workspace.0.join(STAGING).exists());
    }

    #[test]
    fn occupied_staging_or_backup_never_overwrites_previous_output() {
        let workspace = Workspace::new();
        build(&sample(), &workspace.0).unwrap();
        let before = fs::read(workspace.output()).unwrap();
        for reserved in [STAGING, BACKUP] {
            let path = workspace.0.join(reserved);
            fs::create_dir(&path).unwrap();
            fs::write(path.join("keep.txt"), "keep").unwrap();
            assert!(build(&sample(), &workspace.0).is_err());
            assert_eq!(fs::read(workspace.output()).unwrap(), before);
            assert_eq!(fs::read_to_string(path.join("keep.txt")).unwrap(), "keep");
            fs::remove_file(path.join("keep.txt")).unwrap();
            fs::remove_dir(path).unwrap();
        }
    }

    #[test]
    fn occupied_lock_is_not_removed() {
        let workspace = Workspace::new();
        let lock = workspace.0.join(LOCK);
        fs::write(&lock, "another build").unwrap();
        assert!(
            build(&sample(), &workspace.0)
                .unwrap_err()
                .contains("빌드 잠금 실패")
        );
        assert_eq!(fs::read_to_string(lock).unwrap(), "another build");
        assert!(!workspace.0.join(STAGING).exists());
    }

    #[test]
    fn failed_publish_restores_previous_output() {
        let workspace = Workspace::new();
        build(&sample(), &workspace.0).unwrap();
        let before = fs::read(workspace.output()).unwrap();
        // Missing staging simulates a rename failure after the old output was backed up.
        assert!(
            publish(&workspace.0)
                .unwrap_err()
                .contains("출력 교체 실패")
        );
        assert_eq!(fs::read(workspace.output()).unwrap(), before);
        assert!(!workspace.0.join(BACKUP).exists());
    }
}
