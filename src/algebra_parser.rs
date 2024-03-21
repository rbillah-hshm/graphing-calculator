use itertools::Itertools::*;
use std::collections::HashMap;
// Lexical Analysis Errors
const PROCEDURE_SYNTAX_ERROR: &str = "Attempted to locate procedure: Unsucessful";
const PARENTHESIS_ASSIGN_ERROR: &str = "Could not find correspondence for every parenthesis";
//
#[derive(Debug)]
#[repr(i32)]
enum Tokens<'a> {
    ParenthesisLeft = 62,
    ParenthesisRight = 61,
    Function = 60,
    Exponent = 50,
    Mul = 41,
    Div = 40,
    Add = 30,
    Sub = 20,
    Procedure(
        i32,
        Box<Result<(HashMap<&'a str, String>, Tokens<'a>), &'a str>>,
    ) = 12,
    Variable(i32) = 11,
    Number(i32) = 10,
}
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
    ) -> Result<HashMap<&'a str, Vec<Tokens<'a>>>, &'a str> {
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
                        variables_iteration += 1
                        if (variables_iteration < 0) {
                            
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
                                    let temp_parenthesis_check = 0;
                                    let is_first_iteration = true;
                                    let mut n_peek = variables_iter.peek();
                                    while (!(is_first_iteration) && temp_parenthesis_check > 0) {
                                        if (is_first_iteration) {
                                            is_first_iteration = false;
                                        }
                                        if (n_peek == '(') {
                                            temp_parenthesis_check += 1;
                                        } else if (n_peek == ')') {
                                            temp_parenthesis_check -= 1;
                                        }
                                        n_peek =
                                    }
                                };
                                let f3 = (*variables_iter.peek().unwrap() == ')');
                                if !(f1 && f2) {
                                    invalid_flag = true;
                                    break;
                                }
                                token = Some(Tokens::Procedure(name_length));
                                break;
                            }
                        }
                        expression_iterator.next();
                        if (x.is_alphabetic()) {
                            continue;
                        }
                        break;
                    }
                    if (invalid_flag) {
                        return Err(PROCEDURE_SYNTAX_ERROR);
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
                return Err(PARENTHESIS_ASSIGN_ERROR);
            }
            tokenized_map.insert(*function, vector);
        }
        return Ok(tokenized_map);
    }
    pub fn get_terms(tokenized_map: HashMap<&str, Vec<Tokens>>) -> Vec<&str> {
        // After lexical analysis has been successful, retrieve terms
    }
    fn match_operation(operation: char) -> Option<Tokens> {
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
    let tokenized_map = lexical_analyzer::analyze(&function_map);
    println!("{:?}", tokenized_map.ok().unwrap());
}
//
