
/*
 * Custom made implementation of the shunting yard algorithm.
 * Makes a regular mathmatical expression into reverse polish notation,
 * a + b -> a b +
 * a * b + c -> a b * c +
 * and so on. 
 * these can be easily interpreted by an algorithm to calculate the value of any given term.
 *
 * note: this version of shunting yard does not implement functions. They are handled by the
 * expression parser.
 */

#[derive(PartialEq)]
enum Associativity {
    Right,
    Left
}

#[derive(PartialEq)]
pub struct Operator {
    character: char,
    precedence: u8,
    associativity: Associativity
}

impl Operator {
    pub fn is_operator(c: char) -> bool {
        for op in OPERATORS {
            if c == op.character { return true; }
        }
        return false;
    }

    pub fn get_operator(c: char) -> Option<Operator> {
        match c {
            '+' => Some(ADDITION),
            '-' => Some(SUBTRACTION),
            '*' => Some(MULTIPLICATION),
            '/' => Some(DIVISION),
            '^' => Some(EXPONENTIATION),
            _ => None
        }
    }
}

const ADDITION: Operator = Operator {
    character: '+',
    precedence: 2,
    associativity: Associativity::Left
};

const SUBTRACTION: Operator = Operator {
    character: '-',
    precedence: 2,
    associativity: Associativity::Left
};

const MULTIPLICATION: Operator = Operator {
    character: '*',
    precedence: 2,
    associativity: Associativity::Left
};

const DIVISION: Operator = Operator {
    character: '/',
    precedence: 2,
    associativity: Associativity::Left
};

const EXPONENTIATION: Operator = Operator {
    character: '*',
    precedence: 2,
    associativity: Associativity::Right
};

const OPERATORS: [Operator; 5] = [ADDITION, SUBTRACTION, MULTIPLICATION, DIVISION, EXPONENTIATION];

pub fn form_reverse_polish_notation(regular_math: &str) -> Result<Vec<String>, String> {
    let mut output_queue: Vec<Vec<char>> = Vec::new();
    let mut input_queue: Vec<char> = regular_math.chars().rev().collect();
    let mut operator_stack: Vec<char> = Vec::new();
    let mut currently_processing_numeric_group = false;
    let mut current_numeric_group: Vec<char> = Vec::new();
    let mut current_numeric_group_has_point = false;

    // while there are tokens to br read:
    while !(input_queue.is_empty()) {
        // read a token
        let token: char = input_queue.pop().unwrap();
        dbg!(&token);
            
        // if the token is:
        // a number:
        if token.is_numeric() | (token == '.') {
            // put it into the output_queue
            current_numeric_group.push(token);
            currently_processing_numeric_group = true;
            if (token == '.') & (!current_numeric_group_has_point) {
                current_numeric_group_has_point = true;
            }
            else if (token == '.') & (current_numeric_group_has_point) {
                return Err("Numeric group contains too many '.' Only one is allowed.".to_string());
            }
        }
        // a function
        // handled by the expression parser

        // a operator o1
        else if Operator::is_operator(token) {

            // numeric group is done, push it.
            if currently_processing_numeric_group {
                output_queue.push(current_numeric_group);
                current_numeric_group = Vec::new();
                currently_processing_numeric_group = false;
                current_numeric_group_has_point = false;
            }

            // (get the constant Operator (which is a struct) that fits to that token.)
            let o1 = match Operator::get_operator(token) {
                Some(valid_op) => valid_op,
                None => {panic!("Operator '{}' not found.", token);},
            };
            
            // while there is an operator o2 at the top of the stack 
            if !operator_stack.is_empty() {
                dbg!(&operator_stack);
                let o2 = match Operator::get_operator(*(operator_stack.clone().last().clone().unwrap())) {
                    Some(valid_op) => valid_op,
                None => {panic!("Operator '{}' not found.", token);},
                };
                // and
                // (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1
                // is left associative))
                while ((operator_stack.last().is_some()) & ((o2.precedence > o1.precedence) | ((o1.precedence == o2.precedence) & (o1.associativity == Associativity::Left)))) {
                    // pop o2 from the operator stack into the output queue.
                    // after this debug statement, the operator_stack is empty for no reason!!!!
                    // FIXME
                    let my_c = match operator_stack.pop() {
                        Some(c) => c,
                        None => {panic!("weirdly gone!")},
                        };
                    output_queue.push(vec![my_c]);
                }
            }
            operator_stack.push(o1.character);
        }
        /*
        // Unnessecary, will be processed by the expression parser
        else if '(' == token {
            println!("(");
        }
        else if ')' == token {
            println!(")");
        }
        */
        else {
            return Err(("Unrecognized token: '".to_string() + token.to_string().as_str() + "'").to_string());
        }
    }
    // numeric group is done, push it.
    if currently_processing_numeric_group {
        output_queue.push(current_numeric_group);
    }
    dbg!(&output_queue);

    // afterwards, process any operators still on the operator_stack
    while !(operator_stack.is_empty()) {
        output_queue.push(vec![operator_stack.pop().unwrap()]);
    }

    dbg!(&output_queue);
    let mut rpn: Vec<String> = Vec::new();
    for group in output_queue {
        rpn.push(group.iter().cloned().collect::<String>());
    }
    Ok(rpn)
}

// after we have the rpn, we may want to calculate the values with it.
pub fn calc_reverse_polish_notation(rpn: Vec<String>) -> Result<f64, String> {
    Ok(0.0)
}
