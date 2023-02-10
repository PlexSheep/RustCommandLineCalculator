mod expression_parser;
mod linear_algebra;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arg {
    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show debug output
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,

    /// An expression that should be used to calculate something
    expressionVector: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Assert if two expressions are equal to each other
    Equal {
        expressionVector: Vec<String>,
    }
}

fn main() {
    let args = Arg::parse();
    dbg!(args.expressionVector);
}

