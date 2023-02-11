use std::fmt;
use regex::Regex;

// In an expression like `sqrt(25)` the Task would correspond to `sqrt`. This is the enum to 
// configure possible Tasks.
// None means, the Expression doesn't send it's Value to a Task Handler
#[derive(Debug)]    // automatically generate Debug Formatter
pub enum Task {
    None,
    Sqrt,
    Power,
    Log(u64),
}

// How to clone a Task, i was supprised I had to do it myself.
impl Clone for Task {
    fn clone(&self) -> Self {
        // This can probably be done cleaner than with a verbose match. FIXME
        match self {
            Task::None => Task::None,
            Task::Sqrt => Task::Sqrt,
            Task::Power => Task::Power,
            Task::Log(base) => Task::Log(*base), // TODO add base for log
        }
    }
}

impl Task {
    pub fn new(task_text: &str, task_param: &str) -> Task {
        let task_text = task_text.to_lowercase();
        match task_text.as_str() {
            "none" => Task::None,
            "sqrt" => Task::Sqrt,
            "power"|"pow" => Task::Power,
            "log"|"ln" => Task::Log(10),    // TODO add base
            // what to do if a bad task was given:
            &_ => {eprintln!("Bad Task: {}", task_text); std::process::exit(1); },
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
     * example: "12 + log_10(10 + 15) + 3"
     * has a sub expression log_10(10 + 5), which has Task::Log with base 10
     */
    pub fn new(expression_text: String, task: Task) -> Expression {

        // find children
        // TODO add error for unused task parameters
        // TODO add supprot for truly recursie expressions, currently only one expression can be in
        // a root expression.
        let re_sub_expression = Regex::new(r"\w+\(.+?\)").unwrap(); // FIXME doesnt support nested
                                                                    // expressions!!!
        if re_sub_expression.is_match(&expression_text) {
            let mut children: Vec<Expression> = Vec::new();
            for sub_expression_text in re_sub_expression.captures_iter(&expression_text) {
                // if any task parameters are set ( syntax: task_para(expression) )
                if sub_expression_text[0].contains('_') {
                    let task_and_expr: Vec<&str> = sub_expression_text[0].split(['_', '(']).collect();
                    #[cfg(debug_assertions)]
                    dbg!(&task_and_expr);
                    let task = Task::new(task_and_expr[0], task_and_expr[1]);                
                    let mut expression_inner = task_and_expr[2].clone().to_string();
                    #[cfg(debug_assertions)]
                    dbg!(&expression_inner);
                    expression_inner.pop();
                    #[cfg(debug_assertions)]
                    dbg!(&expression_inner);
                    children.push(Expression::new(expression_inner, task));
                }
                // if there are no parameters we need to do diffrent splitting and assume defaults
                else {
                    let task_and_expr: Vec<&str> = sub_expression_text[0].split(['(']).collect();
                    #[cfg(debug_assertions)]
                    dbg!(&task_and_expr);
                    let task_text = task_and_expr[0].clone().to_lowercase();
                    let task = Task::new(&task_text, "");                
                    let mut expression_inner = task_and_expr[1].clone().to_string();
                    expression_inner.pop();
                    #[cfg(debug_assertions)]
                    dbg!(&expression_inner);
                    children.push(Expression::new(expression_inner, task));
                }
            }
        }

        let expression = Expression {
            text: expression_text,
            // TODO generate these from the text!
            task: task,
            complex: false,
            inner_value: 0.0,
            outer_value: 0.0,
            children: Vec::new(),
        };
        #[cfg(debug_assertions)]
        dbg!(&expression);
        expression
    }

    pub fn process(&self) {
        println!("{}", self.text);
    }
}

