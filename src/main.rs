// fuck it
#![feature(str_split_remainder)]
#![feature(const_mut_refs)]
#![feature(iter_advance_by)]
#![feature(step_trait)]

use std::{env::current_dir, process::exit};

use clap::{Parser, Subcommand};

mod compiler;
mod error;
mod project;

use compiler::build;
use error::Error;

#[derive(Parser)]
pub struct Fxg {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Build {
        file: String,

        #[arg(short, long)]
        template: String,

        #[arg(short, long)]
        output: String,
    },

    #[cfg(debug_assertions)]
    VomitDebug {
        file: String,
        #[arg(short, long)]
        output: String,
    },

    New {
        folder: String,
    },
}

fn do_cli(args: Subcommands) -> Result<(), Error> {
    use Subcommands::*;
    match &args {
        Build {
            file,
            template,
            output,
        } => build(file, template, output),

        New { folder } => {
            let mut path = current_dir()?;
            path.push(folder);
            project::new(path)
        }

        #[cfg(debug_assertions)]
        VomitDebug { file, output } => compiler::vomit_debug(file, output),
    }
}

fn main() {
    let args = Fxg::parse().subcommand;
    if let Err(e) = do_cli(args) {
        println!("{}", e);
        exit(-1);
    }
}
