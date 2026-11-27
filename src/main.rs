mod agy_cli;
mod antigravity;
mod build;
mod claude_code;
mod css;
mod helix;
mod iterm2;
mod layers;
mod lbrx_studio;
mod macos_terminal;
mod model;
mod obsidian;
mod resolve;
mod typora;
mod vim;
mod vscode;
mod windows_terminal;

use build::App;
use clap::{Parser, Subcommand};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Parser)]
#[command(version, about = "LBRX 테마 도구")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 테마 원본의 스키마, 색상·ANSI 슬롯, 서체, 대비를 검사합니다.
    Check {
        #[arg(default_value = "themes")]
        path: PathBuf,
    },
    /// 스타일별 토큰, 앱 테마와 설치 안내를 dist\<style>\lbrx에 생성합니다.
    Build {
        #[arg(default_value = "themes")]
        path: PathBuf,
        /// 생성할 앱. 여러 번 지정할 수 있으며 생략하면 지원 앱 전체를 생성합니다.
        #[arg(long, value_enum)]
        app: Vec<App>,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse().command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(reason) => {
            eprintln!("{reason}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result<(), String> {
    run_at(command, Path::new("dist"))
}

fn run_at(command: Command, output: &Path) -> Result<(), String> {
    match command {
        Command::Check { path } => {
            for (_, source) in sources(&path)? {
                resolve::check(&read_theme(&source)?)
                    .map_err(|e| format!("{}: {e}", source.display()))?;
                println!("{}: 테마 검사 통과", source.display());
            }
            Ok(())
        }
        Command::Build { path, app } => {
            let apps = if app.is_empty() { &App::ALL[..] } else { &app };
            let themes = sources(&path)?
                .into_iter()
                .map(|(style, source)| {
                    let theme = read_theme(&source)?;
                    resolve::check(&theme).map_err(|e| format!("{}: {e}", source.display()))?;
                    Ok((style, source, theme))
                })
                .collect::<Result<Vec<_>, String>>()?;
            for (style, source, theme) in themes {
                let parent = style.map_or_else(|| output.to_owned(), |style| output.join(style));
                std::fs::create_dir_all(&parent)
                    .map_err(|e| format!("{}: {e}", parent.display()))?;
                build::build(&theme, &parent, apps)
                    .map_err(|e| format!("{}: {e}", source.display()))?;
                println!("{}", source.display());
                for app in App::ALL.into_iter().filter(|app| apps.contains(app)) {
                    let directory = app.output_path(&parent);
                    println!("{}", app.label());
                    for &file in app.files() {
                        let label = if file == build::GUIDE {
                            "설치 안내"
                        } else {
                            "결과 파일"
                        };
                        println!("  {label}: {}", directory.join(file).display());
                    }
                }
            }
            Ok(())
        }
    }
}

fn sources(path: &Path) -> Result<Vec<(Option<String>, PathBuf)>, String> {
    let metadata =
        std::fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    reject_link(path, &metadata)?;
    if metadata.is_file() {
        let style = path
            .parent()
            .filter(|_| path.file_name().is_some_and(|name| name == "lbrx.toml"))
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .filter(|name| *name != "themes")
            .map(str::to_owned);
        return Ok(vec![(style, path.to_owned())]);
    }
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(format!(
            "{}: 테마 파일 또는 실제 디렉터리가 필요합니다",
            path.display()
        ));
    }
    if path.join("lbrx.toml").is_file() {
        reject_link(
            &path.join("lbrx.toml"),
            &std::fs::symlink_metadata(path.join("lbrx.toml"))
                .map_err(|e| format!("{}: {e}", path.display()))?,
        )?;
        let style = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{}: 스타일 이름이 필요합니다", path.display()))?;
        return Ok(vec![(Some(style.into()), path.join("lbrx.toml"))]);
    }
    let mut result = Vec::new();
    for entry in std::fs::read_dir(path).map_err(|e| format!("{}: {e}", path.display()))? {
        let entry = entry.map_err(|e| format!("{}: {e}", path.display()))?;
        let directory = entry.path();
        reject_link(
            &directory,
            &std::fs::symlink_metadata(&directory)
                .map_err(|e| format!("{}: {e}", directory.display()))?,
        )?;
        if !entry
            .file_type()
            .map_err(|e| format!("{}: {e}", directory.display()))?
            .is_dir()
        {
            continue;
        }
        let source = directory.join("lbrx.toml");
        if !source.is_file() {
            return Err(format!(
                "{}: 스타일 원본 lbrx.toml이 없습니다",
                directory.display()
            ));
        }
        reject_link(
            &source,
            &std::fs::symlink_metadata(&source)
                .map_err(|e| format!("{}: {e}", source.display()))?,
        )?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| format!("{}: UTF-8 스타일 이름이 필요합니다", directory.display()))?;
        result.push((Some(name), source));
    }
    result.sort_by(|a, b| a.0.cmp(&b.0));
    if result.is_empty() {
        return Err(format!("{}: 빌드할 스타일이 없습니다", path.display()));
    }
    Ok(result)
}

fn reject_link(path: &Path, metadata: &std::fs::Metadata) -> Result<(), String> {
    let linked = metadata.file_type().is_symlink();
    #[cfg(windows)]
    let linked = {
        use std::os::windows::fs::MetadataExt;
        linked || metadata.file_attributes() & 0x400 != 0
    };
    if linked {
        return Err(format!(
            "{}: 링크된 테마 경로는 지원하지 않습니다",
            path.display()
        ));
    }
    Ok(())
}

fn read_theme(path: &Path) -> Result<model::Theme, String> {
    layers::load(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn style(root: &Path, name: &str, source: &str) -> PathBuf {
        let directory = root.join(name);
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("lbrx.toml"), source).unwrap();
        directory
    }

    #[test]
    fn discovers_sorted_styles_and_consistent_file_paths() {
        let tmp = tempfile::tempdir().unwrap();
        let orange = style(tmp.path(), "orange-type", "");
        style(tmp.path(), "green-type", "");
        fs::write(tmp.path().join("backup.zip"), "").unwrap();
        let all = sources(tmp.path()).unwrap();
        assert_eq!(all[0].0.as_deref(), Some("green-type"));
        assert_eq!(all[1].0.as_deref(), Some("orange-type"));
        assert_eq!(
            sources(&orange).unwrap(),
            sources(&orange.join("lbrx.toml")).unwrap()
        );
        fs::create_dir(tmp.path().join("missing")).unwrap();
        assert!(sources(tmp.path()).unwrap_err().contains("missing"));
    }

    #[test]
    fn builds_independent_styles_and_validates_all_before_writes() {
        let tmp = tempfile::tempdir().unwrap();
        let inputs = tmp.path().join("themes");
        fs::create_dir(&inputs).unwrap();
        style(
            &inputs,
            "green-type",
            include_str!("../themes/green-type/lbrx.toml"),
        );
        let orange = style(
            &inputs,
            "orange-type",
            include_str!("../tests/fixtures/lbrx.toml"),
        );
        fs::create_dir(orange.join("editors")).unwrap();
        fs::write(
            orange.join("editors").join("windows-terminal.toml"),
            "[colors]\ncursorColor = 'text'\n",
        )
        .unwrap();
        let output = tmp.path().join("dist");
        let build = || Command::Build {
            path: inputs.clone(),
            app: vec![App::WindowsTerminal],
        };
        run_at(build(), &output).unwrap();
        let green_file = output
            .join("green-type")
            .join("lbrx")
            .join("windows-terminal")
            .join("schemes.json");
        let orange_file = output
            .join("orange-type")
            .join("lbrx")
            .join("windows-terminal")
            .join("schemes.json");
        let green = fs::read(&green_file).unwrap();
        assert_ne!(green, fs::read(&orange_file).unwrap());
        let orange_json: serde_json::Value =
            serde_json::from_slice(&fs::read(&orange_file).unwrap()).unwrap();
        assert_eq!(orange_json["schemes"][0]["cursorColor"], "#38291F");
        run_at(
            Command::Build {
                path: orange.clone(),
                app: vec![App::Typora],
            },
            &output,
        )
        .unwrap();
        assert_eq!(green, fs::read(&green_file).unwrap());
        let previous = fs::read(&orange_file).unwrap();
        fs::write(orange.join("lbrx.toml"), "invalid = true").unwrap();
        assert!(run_at(build(), &output).is_err());
        assert_eq!(previous, fs::read(&orange_file).unwrap());
        let fresh = tmp.path().join("fresh");
        assert!(run_at(build(), &fresh).is_err());
        assert!(!fresh.exists());
    }
}
