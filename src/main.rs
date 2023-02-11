use clap::Parser;

mod expression_parser;
mod linear_algebra;

use expression_parser::Expression;
use expression_parser::Task;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arg {
//    /// Optional subcommand
//    #[command(subcommand)]
//    command: Option<Commands>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// An expression that should be used to calculate something
    expressions: Vec<String>,
}

//#[derive(Subcommand)]
//enum Commands {
//    /// Assert if two expressions are equal to each other
//    Equal {
//    }
//}

fn main() {
    let args = Arg::parse();
    let mut expression_vec: Vec<Expression> = Vec::new();

    // join expression_texts to a big expression text, split at '%'. Remove unnessecary whitespace
    // at the start ot end.
    // TODO implement splitting of expressions, currently they are made only into a single big
    // expression text
    let mut expression_texts_concat: Vec<String> = Vec::new();
    expression_texts_concat.push(args.expressions.join(" ").trim().to_string());

    for expression_text in expression_texts_concat {
        expression_vec.push(Expression::new(expression_text, Task::None));
    }

    for expression in expression_vec {
        expression.process();
    }

}
