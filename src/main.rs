use clap::Parser;

mod expression_parser;
mod linear_algebra;

use expression_parser::Expression;
use expression_parser::Task;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arg {
    ///Syntax: '1 + task_param(inner) + 1 '{n}
    ///{n}
    ///Specify an expression, any expression may contain child expressions, which can be denoted{n}
    ///with parenthesis '(child)'. Expressions may have a task applied to them, such as a
    ///logarithm{n} or drawing a root. To apply a task to a expression, simply write the name of{n}
    ///the task before denoting an expression: 'myTask(myExpression)'. You can apply a parameter{n}
    ///to some tasks by using an underscore '_': 'myTask_myParameter(myExpression)'.{n}
    ///{n}
    ///List of Tasks:{n}
    ///{n}
    ///"none"                           explicitly set no task for expression{n}
    ///                                 parameter: none 
    ///{n}
    ///"root" or "sqrt"                 draw the root of the expression{n}
    ///                                 parameter: draw n'th root of expression, default is 2.0{n}
    ///{n}
    ///"power" or "pow" or "sq"         apply an exponent to the expression{n}
    ///                                 parameter: specify exponent n, default is 2.0{n}
    ///{n}
    ///"log" or "ln"                    apply a logarithm to the expression{n}
    ///                                 parameter: specify base n, default is 10{n}
    expressions: Vec<String>,
}

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
