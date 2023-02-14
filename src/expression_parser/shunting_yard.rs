
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

pub fn form_reverse_polish_notation(regular_math: &str) -> Result<String, String> {
    let mut output_queue: Vec<char> = Vec::new();
    let mut input_queue: Vec<char> = regular_math.chars().rev().collect();
    let mut operator_stack: Vec<char> = Vec::new();

    // while there are tokens to br read:
    while !(input_queue.is_empty()) {
        // read a token
        let token: char = input_queue.pop().unwrap();
        dbg!(&token);
            
        // if the token is:
        // a number:
        if token.is_numeric() {
            // put it into the output_queue
            output_queue.push(token);
        }
        // a function
        // handled by the expression parser

        // a operator o1
        else if Operator::is_operator(token) {
            // (get the constant Operator (which is a struct) that fits to that token.)
            let o1 = match Operator::get_operator(token) {
                Some(valid_op) => valid_op,
                None => {panic!("Operator '{token}' not found.");},
            };
            
            // while there is an operator o2 at the top of the stack 
            if !operator_stack.is_empty() {
                dbg!(&operator_stack);
                dbg!(&operator_stack);
                let o2 = match Operator::get_operator(*(operator_stack.clone().last().clone().unwrap())) {
                    Some(valid_op) => valid_op,
                    None => {panic!("Operator '{token}' not found.");},
                };
                // and
                // (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1
                // is left associative))
                while ((operator_stack.last().is_some()) & ((o2.precedence > o1.precedence) | ((o1.precedence == o2.precedence) & (o1.associativity == Associativity::Left)))) {
                    dbg!(&operator_stack);
                    println!("REACHED THE MAGIC WHILE");
                    // pop o2 from the operator stack into the output queue.
                    // after this debug statement, the operator_stack is empty for no reason!!!!
                    // FIXME
                    let my_c = match operator_stack.pop() {
                        Some(c) => c,
                        None => {panic!("weirdly gone!")},
                        };
                    output_queue.push(my_c);
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
            return Err("Unrecognized token: '{token}'".to_string());
        }
    }
    dbg!(&output_queue);

    // afterwards, process any operators still on the operator_stack
    while !(operator_stack.is_empty()) {
        output_queue.push(operator_stack.pop().unwrap());
    }

    dbg!(&output_queue);
    let rpn: String = output_queue.iter().cloned().collect::<String>();
    Ok(rpn)
}

// after we have the rpn, we may want to calculate the values with it.
pub fn calc_reverse_polish_notation(rpn: &str) -> Result<f64, String> {
    Ok(0.0)
}
