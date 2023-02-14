
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

enum Associativity {
    Right,
    Left
}

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
    associativity: Associativity::Left
};

const OPERATORS: [Operator; 5] = [ADDITION, SUBTRACTION, MULTIPLICATION, DIVISION, EXPONENTIATION];

pub fn form_reverse_polish_notation(regular_math: &str) -> Result<String, String> {
    let mut output_queue: Vec<char> = Vec::new();
    let mut input_queue: Vec<char> = regular_math.chars().rev().collect();
    let mut operator_stack: Vec<char> = Vec::new();

    // process all tokens first.
    while !(input_queue.is_empty()) {
        let token: char = *(input_queue.last().unwrap());
        input_queue.pop();
        dbg!(&token);
            
        if token.is_numeric() {
            println!("number");
        }
        else if Operator::is_operator(token) {
            println!("operator");
        }
        // Unnessecary, will be processed by the expression parser
        //else if '(' == token {
        //    println!("(");
        //}
        //else if ')' == token {
        //    println!(")");
        //}
        else {
            eprintln!("Unrecognized token:  '{token}'");
            std::process::exit(1);
        }

    }

    // afterwards, process any operators still on the operator_stack
    while !(operator_stack.is_empty()) {
        todo!();
    }

    Ok("TODO".to_string())
}

// after we have the rpn, we may want to calculate the values with it.
pub fn calc_reverse_polish_notation(rpn: &str) -> Result<f64, String> {
    Ok(0.0)
}
