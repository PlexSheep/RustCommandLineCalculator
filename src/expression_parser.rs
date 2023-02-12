use std::fmt;
use std::collections::HashMap;
use std::hash::Hash;
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

fn find_brace_groups(haystack: String) -> Vec<Vec<(usize, usize)>> {

        // TODO add support for diffrent braces
        // TODO add error if not all braces are closed
        let mut parenthesis_group: Vec<(usize, usize)> = Vec::new();
        let mut parenthesis_open: usize = 0;
        let mut parenthesis_open_processed: usize = 0;
        let mut parenthesis_closed_processed: usize = 0;
        let mut parenthesis_last_opened: Vec<usize> = Vec::new();
        //let mut brackets_group: Vec<(usize, usize)> = Vec::new();
        //let mut brackets_open: usize = 0;
        //let mut square_braces_group: Vec<(usize, usize)> = Vec::new();
        //let mut square_braces_open: usize = 0;
        // first open stuff
        for (index, char) in haystack.chars().enumerate() {
            match char {
                '(' => { 
                    #[cfg(debug_assertions)]
                    {
                    dbg!(char);
                    dbg!(index);
                    }
                    parenthesis_group.push((index, 0)); 
                    parenthesis_open = parenthesis_open + 1;
                    parenthesis_last_opened.push(parenthesis_open_processed);
                    parenthesis_open_processed = parenthesis_open_processed + 1;
                },
                ')' => { 
                    let len = parenthesis_group.len(); 
                    #[cfg(debug_assertions)]
                    {
                    dbg!(char);
                    dbg!(index);
                    dbg!(parenthesis_last_opened.len());
                    dbg!(parenthesis_last_opened[parenthesis_last_opened.len() - 1]);
                    }
                    parenthesis_group[parenthesis_last_opened[parenthesis_last_opened.len() - 1]].1 = index;
                    parenthesis_open = parenthesis_open - 1;
                    parenthesis_closed_processed = parenthesis_closed_processed + 1;
                    parenthesis_last_opened.pop();
                    // TODO add error if no parenthesis is open yet.
                },
                _ => (),
            }
        }
        // now iterate backwards and search for closing things

        let brace_groups = vec![parenthesis_group/*, square_braces_group, brackets_group*/];
        #[cfg(debug_assertions)]
        dbg!(&brace_groups);
        return brace_groups;
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

        let re_contains_sub_expression= Regex::new(r"(\(.*\))|(\[.*\])|(\{.*\})").unwrap();
        if re_contains_sub_expression.is_match(expression_text.as_str()) {
            let brace_groups: Vec<Vec<(usize, usize)>> = find_brace_groups(expression_text.clone());

            let mut brace_groups_texts: Vec<String> = Vec::new();
            let mut children: Vec<Expression> = Vec::new();

            // 1 brace group per possible combination, by default, this is only (), so 1 iteration.
            // This is still O(n¹) 
            for brace_group in brace_groups {
                for pair in brace_group {
                    let text = &expression_text[pair.0..pair.1 + 1];
                    let text = &text[1..text.len() - 1];
                    #[cfg(debug_assertions)]
                    dbg!(text);
                    brace_groups_texts.push(text.to_string());
                    // we have the expression_text, now we just need to get the task until we can
                    // pass these parameters into Expression::new(). This is the recursive part.
                    let possible_task = &expression_text[..pair.0].chars().rev().collect::<String>();
                    dbg!(possible_task);
                }
            }
        } 


        let expression = Expression {
            text: expression_text,
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
