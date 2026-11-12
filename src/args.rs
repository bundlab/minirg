use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "minirg",
    author = "Developer",
    version = "0.1.0",
    about = "A fast line-oriented search tool inspired by ripgrep"
)]
pub struct Args {
    #[arg(help = "The regular expression pattern to search for")]
    pub pattern: String,

    #[arg(help = "The file or directory to search in", default_value = ".")]
    pub path: PathBuf,
}