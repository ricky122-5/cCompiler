use regex::Regex;
use regex::RegexSet;
fn main() {
    lex(&String::from("int main() {
    return 2;
}"));
}

fn lex(input: &String) -> Vec<String> {

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
        if let Some(match_result) = open_bracket.find_at(input, c.0) {
            if match_result.start() == c.0 {
                // Match at current val, push
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = closed_bracket.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = open_paranthesis.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = closed_paranthesis.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = semicolon.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = int_keyword.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = return_keyword.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = identifier.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }

        if let Some(match_result) = integer_literal.find_at(input, c.0) {
            if match_result.start() == c.0 {
                tokens.push(match_result.as_str().to_string());
                c += match_result.len();
            }
        }
    }

    return tokens;

}

