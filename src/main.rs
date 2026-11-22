mod build;
mod css;
mod model;
mod obsidian;
mod resolve;
mod typora;
mod windows_terminal;

use clap::{Parser, Subcommand};
use std::{
    fs,
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
    /// 검증된 토큰과 Windows Terminal·Typora·Obsidian 테마를 dist\lbrx에 생성합니다.
    Build {
        #[arg(default_value = "themes/lbrx.toml")]
        path: PathBuf,
    },
}

fn main() -> ExitCode {
    let command = Cli::parse().command;
    let path = match &command {
        Command::Check { path } | Command::Build { path } => path,
    };
    let result = fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|input| toml::from_str::<model::Theme>(&input).map_err(|e| e.to_string()))
        .and_then(|theme| match &command {
            Command::Check { .. } => resolve::check(&theme),
            Command::Build { .. } => build::build(&theme, Path::new("dist")),
        });
    match result {
        Ok(()) => {
            match command {
                Command::Check { .. } => println!("{}: 테마 검사 통과", path.display()),
                Command::Build { .. } => {
                    println!(
                        "dist\\lbrx: 검증된 토큰 및 Windows Terminal·Typora·Obsidian 테마 생성 완료"
                    )
                }
            }
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("{}: {reason}", path.display());
            ExitCode::FAILURE
        }
    }
}
