use regex::Regex;
fn main() {
    let res = lex(&String::from("int main() {
    return 2;
}"));
    println!("{:?}", res);
    let AST = parse(&res);
    println!("{:?}", AST);
}
#[derive(Debug, PartialEq)]

enum TokenType {
    OpenBracket, 
    ClosedBracket,
    OpenParanthesis,
    ClosedParanthesis,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier,
    IntegerLiteral,
    Unknown
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String
}

fn lex(input: &String) -> Vec<Token> {

    // Regex for open bracket
    let open_bracket = Regex::new(r"\{").unwrap();

    // Regex for closed bracket
    let closed_bracket = Regex::new(r"\}").unwrap();

    // Regex for open paranthesis
    let open_paranthesis = Regex::new(r"\(").unwrap();

    // Regex for closed paranthesis
    let closed_paranthesis = Regex::new(r"\)").unwrap();

    // Regex for semicolon
    let semicolon = Regex::new(r";").unwrap();

    // Regex for int var type
    let int_keyword = Regex::new(r"int").unwrap();

    // Regex for return keyword
    let return_keyword = Regex::new(r"return").unwrap();

    // Regex for identifier
    let identifier = Regex::new(r"[a-zA-Z]\w*").unwrap();

    // Regex for int literal
    let integer_literal = Regex::new(r"[0-9]+").unwrap();

    let mut tokens = Vec::new();
    let mut c = 0;
    while c < input.len() {
        // Check if we have whitespace
        if input.chars().nth(c).unwrap().is_whitespace() {
            c += 1;
            continue;
        }

        let mut best_match: Option<Token> = None;
        let mut best_length = 0;


        if let Some(match_result) = open_bracket.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Current char is open bracket
                best_match = Some(Token {
                    token_type: TokenType::OpenBracket,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = closed_bracket.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::ClosedBracket,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = open_paranthesis.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::OpenParanthesis,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = closed_paranthesis.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::ClosedParanthesis,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = semicolon.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::Semicolon,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = int_keyword.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::IntKeyword,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = return_keyword.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::ReturnKeyword,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = identifier.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::Identifier,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = integer_literal.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(Token {
                    token_type: TokenType::IntegerLiteral,
                    value: match_result.as_str().to_string()
                });
                best_length = match_result.len();
            }
        }

        // Push final value, check if invalid
        if let Some(token) = best_match {
            tokens.push(token);
            c += best_length;
        } else {
            // invalid char
            // we still want to lex it

            tokens.push(Token { token_type: TokenType::Unknown, value: input.chars().nth(c).unwrap().to_string()});
            c += 1;
        }
    }

    return tokens;

}

#[derive(Debug)]
struct Program {
    function_declaration: Function
}

#[derive(Debug)]
struct Function {
    function_name: String,
    statement: Return
}

#[derive(Debug)]
struct Return {
    expression: Constant
}

#[derive(Debug)]
struct Constant {
    value: i32
}



fn parse(tokens: &Vec<Token>) -> Program {
    let mut pos = 0;
    let func_name: String;
    
    if tokens[pos].token_type != TokenType::IntKeyword {
        fail();
    }
    pos += 1;
    if tokens[pos].token_type != TokenType::Identifier {
        fail();
    }
    func_name = tokens[pos].value.to_string();
    pos += 1;
    if tokens[pos].token_type != TokenType::OpenParanthesis {
        fail();
    }
    pos += 1;
    if tokens[pos].token_type != TokenType::ClosedParanthesis {
        fail();
    }
    pos += 1;
    if tokens[pos].token_type != TokenType::OpenBracket {
        fail();
    }
    pos += 1;
    let stmt = parse_statement(tokens, &mut pos);

    if tokens[pos].token_type != TokenType::ClosedBracket {
        fail();
    }

    return Program { function_declaration: Function { function_name: func_name, statement: stmt } }
    
}

fn fail() {
    panic!("Invalid token.")
}

fn parse_statement(tokens: &Vec<Token>, pos: &mut usize) -> Return {
    if tokens[*pos].token_type != TokenType::ReturnKeyword {
        fail();
    }
    *pos += 1;
    if tokens[*pos].token_type != TokenType::IntegerLiteral {
        fail();
    }
    let constant = Constant {value: Result::expect(tokens[*pos].value.parse(), r"Constant was not a i32 value!")};
    *pos += 1;
    if tokens[*pos].token_type != TokenType::Semicolon {
        fail();
    }
    *pos += 1;
    return Return { expression: constant }
}

