mod build;
mod css;
mod helix;
mod iterm2;
mod layers;
mod model;
mod obsidian;
mod resolve;
mod tui;
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
        #[arg(default_value = "themes/lbrx.toml")]
        path: PathBuf,
    },
    /// 검증된 토큰과 선택한 앱의 테마를 dist\lbrx에 생성합니다.
    Build {
        #[arg(default_value = "themes/lbrx.toml")]
        path: PathBuf,
        /// 생성할 앱. 여러 번 지정할 수 있으며 생략하면 지원 앱 전체를 생성합니다.
        #[arg(long, value_enum)]
        app: Vec<App>,
    },
    /// 원본을 변경하지 않고 앱별 라이트·다크 견본을 표시합니다.
    Preview {
        #[arg(default_value = "themes/lbrx.toml")]
        path: PathBuf,
        /// 표시할 앱. 여러 번 지정할 수 있으며 생략하면 지원 앱 전체를 표시합니다.
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
    let output = Path::new("dist");
    match command {
        Command::Preview { path, app } => tui::run(&path, &app),
        Command::Check { path } => {
            resolve::check(&read_theme(&path)?).map_err(|e| format!("{}: {e}", path.display()))?;
            println!("{}: 테마 검사 통과", path.display());
            Ok(())
        }
        Command::Build { path, app } => {
            let apps = if app.is_empty() { &App::ALL[..] } else { &app };
            build::build(&read_theme(&path)?, output, apps)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            for app in App::ALL.into_iter().filter(|app| apps.contains(app)) {
                println!("{}: {}", app.label(), app.output_path(output).display());
            }
            Ok(())
        }
    }
}

fn read_theme(path: &Path) -> Result<model::Theme, String> {
    layers::load(path)
}
