use crate::{
    model::Theme,
    resolve::{self, ResolvedTheme},
    typora, windows_terminal,
};
use std::{
    fs::{self, File, Metadata},
    io::{self, Write},
    path::Path,
};

const OUTPUT: &str = "lbrx";
const STAGING: &str = ".lbrx-build";
const BACKUP: &str = ".lbrx-previous";
const LOCK: &str = ".lbrx-build.lock";
const TOKENS: &str = "resolved.toml";
const ADAPTERS: &[(&str, &[&str])] = &[
    (windows_terminal::DIRECTORY, &windows_terminal::FILES),
    (typora::DIRECTORY, typora::FILES),
];
type AdapterOutput<'a> = (&'static str, &'a [(&'static str, String)]);

pub fn build(theme: &Theme, parent: &Path) -> Result<(), String> {
    let resolved = resolve::resolve(theme)?;
    let contents = toml::to_string_pretty(&resolved).map_err(|e| e.to_string())?;
    let terminal_files = windows_terminal::render(&resolved)?;
    let typora_files = typora::render(&resolved);
    let adapters = [
        (windows_terminal::DIRECTORY, terminal_files.as_slice()),
        (typora::DIRECTORY, typora_files.as_slice()),
    ];
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
    adapters: &[AdapterOutput<'_>],
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
    adapters: &[AdapterOutput<'_>],
) -> Result<(), String> {
    let staging = parent.join(STAGING);
    let path = staging.join(TOKENS);
    let written = write_staged(&path, contents)?;
    let parsed: ResolvedTheme = toml::from_str(&written)
        .map_err(|e| format!("{}: 출력 재파싱 실패: {e}", path.display()))?;
    if parsed != *resolved {
        return Err(format!("{}: 해석 결과와 출력이 다름", path.display()));
    }
    for (directory, files) in adapters {
        let directory = staging.join(directory);
        fs::create_dir(&directory).map_err(|e| format!("{}: {e}", directory.display()))?;
        for (name, contents) in *files {
            let path = directory.join(name);
            let written = write_staged(&path, contents)?;
            if name.ends_with(".json") {
                serde_json::from_str::<serde_json::Value>(&written)
                    .map_err(|e| format!("{}: 출력 재파싱 실패: {e}", path.display()))?;
            }
        }
    }
    publish(parent)
}

fn write_staged(path: &Path, contents: &str) -> Result<String, String> {
    let write_and_read = || -> io::Result<String> {
        let mut file = File::create_new(path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
        drop(file);
        fs::read_to_string(path)
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
    for (directory, files) in ADAPTERS {
        let directory = path.join(directory);
        for name in *files {
            ignore_missing(fs::remove_file(directory.join(name)))?;
        }
        ignore_missing(fs::remove_dir(&directory))?;
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
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_file() && entry.file_name() == TOKENS {
            continue;
        }
        let Some((_, allowed)) = ADAPTERS.iter().find(|(name, _)| entry.file_name() == *name)
        else {
            return Err(unexpected(&entry.path()));
        };
        if !plain_directory(&entry.metadata()?) {
            return Err(unexpected(&entry.path()));
        }
        for file in fs::read_dir(entry.path())? {
            let file = file?;
            if !file.file_type()?.is_file() || !allowed.iter().any(|name| file.file_name() == *name)
            {
                return Err(unexpected(&file.path()));
            }
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
            "/themes/lbrx.toml"
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
        assert_eq!(tokens.light.roles["link"], theme.light.palette["pine"]);
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
        assert_eq!(fs::read_dir(workspace.0.join(OUTPUT)).unwrap().count(), 3);
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
            &format!("--lbrx-link: {};", theme.dark.palette["pine"]),
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
        expected["schemes"][1]["cursorColor"] = "#A2E3C6".into();
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
