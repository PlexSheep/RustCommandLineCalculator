use clap::{Parser, Subcommand};

mod expression_parser;
mod linear_algebra;

use expression_parser::Expression;


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
    expression_texts: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Assert if two expressions are equal to each other
    Equal {
    }
}

fn main() {
    let args = Arg::parse();
    let mut expression_vec: Vec<Expression> = Vec::new();

    // join expression_texts to a big expression text, split at '%'. Remove unnessecary whitespace
    // at the start ot end.
    // TODO implement splitting of expressions, currently they are made only into a single big
    // expression text
    let mut expression_texts_concat: Vec<String> = Vec::new();
    expression_texts_concat.push(args.expression_texts.join(" ").trim().to_string());

    for expression_text in expression_texts_concat {
        expression_vec.push(Expression::new(expression_text));
    }
    dbg!(expression_vec);

}
