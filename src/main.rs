#![allow(dead_code)]
#![allow(unused_variables)]

extern crate comrak;
extern crate glob;
extern crate regex;

extern crate clap;
use clap::{Parser, Subcommand, ValueEnum};

mod contents;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Export {
        #[arg(short = 'f', long = "source", value_enum)]
        source: Format,

        #[arg(short = 't', long = "target", value_enum)]
        target: Format,

        #[arg(long = "source_dir", default_value = ".")]
        source_dirpath: String,

        #[arg(long = "target_dir", default_value = ".")]
        target_dirpath: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    HugoRobust,
    Zenn,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Export {
            source,
            target,
            source_dirpath,
            target_dirpath,
        } => contents::export_contents(source, target, source_dirpath, target_dirpath),
    }
}
