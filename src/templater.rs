use std::collections::HashMap;
use std::process::exit;

struct ParserState {
    escape_next: bool,
    stack: Vec<char>,
    expression: String,
}

struct ExprState {
    variables: HashMap<String, String>
}

pub fn format_template(file: String) {
    let mut state = ParserState {
        escape_next: false,
        stack: Vec::new(),
        expression: String::new(),
    };
    let mut formatted = String::new();
    let mut expr_state = ExprState {
        variables: HashMap::new()
    };

    for (lc, line) in file.lines().enumerate() {
        for (cc, chr) in line.chars().enumerate() {
            match chr {
                '\\' if state.escape_next => {
                    formatted.push('\\');
                    state.escape_next = false;
                },
                '\\' => state.escape_next = true,
                _ if state.escape_next => {
                    formatted.push(chr);
                    state.escape_next = false;
                },
                '{' => state.stack.push('{'),
                '}' if state.stack.last() == Some(&'{') => {
                    formatted.push_str(
                        eval_expression(state.expression.trim(), &mut expr_state, (lc, cc))
                            .as_str()
                    );
                    state.expression = String::new();
                    state.stack.pop();
                },
                _ if state.stack.last() == Some(&'{') => state.expression.push(chr),
                '}' => {
                    println!("Error ({}:{}): Unexpected '}}'", lc + 1, cc + 1);
                    exit(1);
                },
                _ => formatted.push(chr),
            }
        }

        // Add a newline if the line isn't an expression and it isn't the last line
        if !(line.starts_with('{') && line.ends_with('}')) && lc + 1 != file.lines().count() {
            formatted.push('\n');
        }

        if state.escape_next {
            println!("Error ({}:EOL): Unexpected '\\'", lc + 1);
            exit(1);
        }

        if let Some(c) = state.stack.last() {
            println!("Error ({}:EOL): Unexpected '{}'", lc + 1, c);
            exit(1);
        }
    }

    println!("{}", formatted);
}

fn eval_expression(expr: &str, state: &mut ExprState, (lc, cc): (usize, usize)) -> String {
    if expr.starts_with('#') && expr.ends_with('#') {
        return String::new();
    }

    let args = expr.split(' ').collect::<Vec<&str>>();

    match args[..] {
        ["define", name, ..] => {
            state.variables.insert(name.to_string(), args[2..].join(" "));
            String::new()
        },
        [var] => match state.variables.get(var) {
            Some(val) => val.clone(),
            None => {
                println!("Error ({}:{}): Undefined variable '{}'", lc + 1, cc + 1, var);
                exit(1);
            }
        },
        ["if", var, "is", val, ..] => {
            if state.variables.get(var) == Some(&val.to_string()) {
                args[4..].join(" ").to_string()
            } else {
                String::new()
            }
        },
        _ => {
            println!("Error ({}:{}): Undefined expression '{}'", lc + 1, cc + 1, expr);
            exit(1);
        }
    }
}
