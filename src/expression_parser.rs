use std::fmt;
use regex::Regex;

// In an expression like `sqrt(25)` the Task would correspond to `sqrt`. This is the enum to 
// configure possible Tasks.
// None means, the Expression doesn't send it's Value to a Task Handler
#[derive(Debug)]    // automatically generate Debug Formatter
enum Task {
    None,
    Sqrt,
    Power,
    Log,
}

// How to clone a Task, i was supprised I had to do it myself.
impl Clone for Task {
    fn clone(&self) -> Self {
        // This can probably be done cleaner than with a verbose match. FIXME
        match self {
            Task::None => Task::None,
            Task::Sqrt => Task::Sqrt,
            Task::Power => Task::Power,
            Task::Log => Task::Log,
        }
    }
}

// An Expression is something that can be calculated. 20+5 is an expression. Expressions can 
// contain other
// Expressions and have tasks: 20+sqrt(20+5)
// Expressions are marked down with braces and a task before those braces:
// task(Expression)
// once the Value of the Expression got calculated, the calculated value should be sent to the 
// TaskHandler, if the Task of the Expression is not Task::None
pub struct Expression {
    text: String,
    task: Task,
    complex: bool,
    inner_value: f64,
    outer_value: f64,
    children: Vec<Expression>,
}

// Debug Formatter for Expression
impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Expression")
            .field("text", &self.text)
            .field("task", &self.task)
            .field("is complex?", &self.complex)
            .field("inner value", &self.inner_value)
            .field("outer value", &self.outer_value)
            .field("children", &self.children)
            .finish()
    }
}

// implement clone by ourselves, as it's not automatically done for us.
impl Clone for Expression{
    fn clone(&self) -> Self {
        Expression { 
            text: self.text.clone(), 
            task: self.task.clone(),
            complex: self.complex.clone(),  // TODO add support for complex numbers
            inner_value: self.inner_value.clone(),
            outer_value: self.outer_value.clone(),
            children: self.children.clone(),
        }
    }
}

/*
 * Main logic for the Expression struct
 */
impl Expression {
    /*
     * Main function for making text into Expression
     */
    pub fn new(expression_text: String) -> Expression {

        // find children
        let re_sub_expression = Regex::new(r"\w+\(.+\)").unwrap();
        if re_sub_expression.is_match(&expression_text) {
            for sub_expression_text in re_sub_expression.captures_iter(&expression_text) {
                println!("{}", &sub_expression_text[0]);
            }
        }

        let expression = Expression {
            text: expression_text,
            // TODO generate these from the text!
            task: Task::None,
            complex: false,
            inner_value: 0.0,
            outer_value: 0.0,
            children: Vec::new(),
        };
        expression
    }
}
