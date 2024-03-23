use itertools::Itertools;
use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::fmt::Debug;
// Lexical Analysis Errors
const PROCEDURE_SYNTAX_ERROR: &str = "Attempted to locate procedure: Unsucessful";
const PARENTHESIS_ASSIGN_ERROR: &str = "Could not find correspondence for every parenthesis";
//
#[derive(Debug)]
#[repr(i32)]
enum Tokens<'a> {
    ParenthesisLeft,
    ParenthesisRight,
    Function,
    Exponent,
    Mul,
    Div,
    Add,
    Sub,
    Procedure(i32, Box<(HashMap<&'a str, String>, Tokens<'a>)>),
    Variable(i32),
    Number(i32),
}
enum FilterType {
    WhiteList,
    BlackList,
}
struct FilterList<'a> {
    list_type: FilterType,
    list: Vec<&'a str>,
}
#[derive(Debug)]
struct TracebackWrapper<'a, T> {
    err: Result<T, &'a str>,
    traceback: String,
}
enum TraceExists<'a, T> {
    Wrapper(TracebackWrapper<'a, T>),
    Success(T),
}
macro_rules! backtrace_wrapper {
    ($backtrace_wrapper:ident, $name:ty) => {
        fn $backtrace_wrapper<'a>(err: Result<$name, &'a str>) -> TraceExists<'a, $name> {
            return TraceExists::Wrapper(TracebackWrapper {
                err: err,
                traceback: Backtrace::force_capture().to_string(),
            })
        }
    };
}
type LexicalTracerType<'a> = HashMap<&'a str, Vec<Tokens<'a>>>;
backtrace_wrapper!(lexical_tracer, LexicalTracerType<'a>);
pub mod lexical_analyzer {
    use super::*;
    pub fn clean<'a>(function_map: &HashMap<&'a str, String>) -> HashMap<&'a str, String> {
        function_map
            .iter()
            .map(|(&name, function)| (name, function.replace(" ", "")))
            .collect::<HashMap<&str, String>>()
    }
    pub fn analyze<'a>(
        function_map: &HashMap<&'a str, String>,
        filter_list: Option<FilterList>,
    ) -> TraceExists<'a, LexicalTracerType<'a>> {
        let mut tokenized_map = HashMap::new();
        for (function, expression) in function_map.iter() {
            let mut vector = Vec::new();
            let mut expression_iterator = expression.chars().enumerate();
            let mut parenthesis_check = 0;
            let analyze = |position: usize, char: char| {};
            while let Some((position, char)) = expression_iterator.next() {
                if (char == '(') {
                    parenthesis_check += 1;
                } else if (char == ')') {
                    parenthesis_check -= 1;
                }
                let mut token = None;
                if (char.is_numeric()) {
                    let mut consecutive_digit = 0;
                    let mut encountered_alphabet = false;
                    expression
                        .chars()
                        .skip(position)
                        .take_while(|x| {
                            consecutive_digit += 1;
                            expression_iterator.next();
                            let is_alphabetic = x.is_alphabetic();
                            if (is_alphabetic) {
                                encountered_alphabet = true;
                            }
                            (x.is_numeric() || (*x == '.'))
                                || (encountered_alphabet && !(is_alphabetic))
                        })
                        .collect::<String>();
                    token = Some(Tokens::Number(consecutive_digit));
                } else if (char.is_alphabetic()) {
                    let mut name_length = 0;
                    let mut invalid_flag = false;
                    let mut variables_iteration = 0;
                    let mut variables_iter = expression.chars().multipeek();
                    while let Some(x) = variables_iter.next() {
                        variables_iteration += 1;
                        if (variables_iteration < position) {
                            continue;
                        }
                        name_length += 1;
                        let is_alphabetic = x.is_alphabetic();
                        if (!is_alphabetic) {
                            if let Some(i) = self::match_operation(x) {
                                token = Some(Tokens::Variable(name_length - 1));
                            } else if (x == ')') {
                                break;
                            } else {
                                let f1 = (x == '(');
                                let f2 = {
                                    let mut temp_parenthesis_check = 0;
                                    let mut is_first_iteration = true;
                                    let mut break_flag = false;
                                    let mut match_success = false;
                                    let mut n_peek = variables_iter.peek();
                                    let nested_functions: Vec<&str> = Vec::new();
                                    while (!is_first_iteration && !break_flag) {
                                        if (is_first_iteration) {
                                            is_first_iteration = false;
                                        } else if (temp_parenthesis_check <= 0) {
                                            match_success = true;
                                            break;
                                        }
                                        match n_peek {
                                            Some(v) => {
                                                if (*v == '(') {
                                                    temp_parenthesis_check += 1;
                                                    let mut rev_find_variable = expression
                                                        .chars()
                                                        .rev()
                                                        .skip(expression.len() - position);
                                                    let mut rev_iteration = 0;
                                                    let mut flag = false;
                                                    let mut name =
                                                        String::with_capacity(expression.len());
                                                    while let Some(w) = rev_find_variable.next() {
                                                        rev_iteration += 1;
                                                        if (!w.is_alphabetic()) {
                                                            if (self::match_operation(w).is_some())
                                                            {
                                                                if (rev_iteration > 1) {
                                                                    flag = true;
                                                                }
                                                            }
                                                            break;
                                                        } else {
                                                            name.push(w);
                                                        }
                                                    }
                                                    if (flag) {}
                                                } else if (*v == ')') {
                                                    temp_parenthesis_check -= 1;
                                                }
                                            }
                                            None => {
                                                break_flag = true;
                                            }
                                        }
                                        n_peek = variables_iter.peek();
                                    }
                                    match break_flag {
                                        true => None,
                                        false => {
                                            Some(lexical_tracer(Err(PARENTHESIS_ASSIGN_ERROR)))
                                        }
                                    }
                                };
                                if !(f1 && f2.is_some()) {
                                    invalid_flag = true;
                                    break;
                                }
                                // token = Some(Tokens::Procedure(name_length));
                                break;
                            };
                        }
                        expression_iterator.next();
                        if (x.is_alphabetic()) {
                            continue;
                        }
                        break;
                    }
                    if (invalid_flag) {
                        return lexical_tracer(Err(PROCEDURE_SYNTAX_ERROR));
                    }
                } else {
                    token = match_operation(char);
                }
                match token {
                    Some(x) => {
                        vector.push(x);
                    }
                    None => println!("Well that was a dud!"),
                }
            }
            if (parenthesis_check != 0) {
                return lexical_tracer(Err(PARENTHESIS_ASSIGN_ERROR));
            }
            tokenized_map.insert(*function, vector);
        }
        return TraceExists::Success(tokenized_map);
    }
    pub fn get_terms<'a>(tokenized_map: HashMap<&'a str, Vec<Tokens>>) -> Vec<&'a str> {
        // After lexical analysis has been successful, retrieve terms
        let vector = Vec::new();
        vector
    }
    fn match_operation<'a>(operation: char) -> Option<Tokens<'a>> {
        match operation {
            '(' => Some(Tokens::ParenthesisLeft),
            ')' => Some(Tokens::ParenthesisRight),
            '^' => Some(Tokens::Exponent),
            '*' => Some(Tokens::Mul),
            '/' => Some(Tokens::Div),
            '+' => Some(Tokens::Add),
            '-' => Some(Tokens::Sub),
            _ => None,
        }
    }
    fn match_token_to_priority(operation: Tokens) -> f32 {
        match operation {
            Tokens::ParenthesisLeft => 61 as f32,
            Tokens::ParenthesisRight => 60 as f32,
            Tokens::Function => 50 as f32,
            Tokens::Exponent => 40 as f32,
            Tokens::Mul => 31 as f32,
            Tokens::Div => 30 as f32,
            Tokens::Add => 20 as f32,
            Tokens::Sub => 10 as f32,
            Tokens::Procedure(x, y) => 3 as f32,
            Tokens::Variable(x) => 2 as f32,
            Tokens::Number(x) => 1 as f32,
        }
    }
    fn token_is_less_than(a: Tokens, b: Tokens) -> bool {
        (match_token_to_priority(a) / 10.0).floor() < (match_token_to_priority(b) / 10.0).floor()
    }
    fn token_is_greater_than(a: Tokens, b: Tokens) -> bool {
        (match_token_to_priority(a) / 10.0).floor() > (match_token_to_priority(b) / 10.0).floor()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lexical_analysis() {}
}
// AST = Abstract Syntax Tree
pub fn generate_all_ast(data: &HashMap<&str, String>) {
    let mut function_map = (*data).clone();
    function_map = lexical_analyzer::clean(&function_map);
    let tokenized_map = lexical_analyzer::analyze(&function_map, None);
    match tokenized_map {
        TraceExists::Success(x) => {}
        TraceExists::Wrapper(x) => {}
    }
}
//
