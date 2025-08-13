use regex::Regex;
fn main() {
    let res = lex(&String::from("int main() {
    return 2;
}"));
    println!("{:?}", res);
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
        // Check if we have whitespace
        if input.chars().nth(c).unwrap().is_whitespace() {
            c += 1;
            continue;
        }

        let mut best_match: Option<&str> = None;
        let mut best_length = 0;


        if let Some(match_result) = open_bracket.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = closed_bracket.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = open_paranthesis.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = closed_paranthesis.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = semicolon.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = int_keyword.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = return_keyword.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = identifier.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        if let Some(match_result) = integer_literal.find_at(input, c) {
            if match_result.start() == c && match_result.len() > best_length {
                // Match at current val, push
                best_match = Some(match_result.as_str());
                best_length = match_result.len();
            }
        }

        // Push final value, check if invalid
        if let Some(token) = best_match {
            tokens.push(token.to_string());
            c += best_length;
        } else {
            // invalid char
            // we still want to parse it
            tokens.push(input.chars().nth(c).unwrap().to_string());
            c += 1;
        }
    }

    return tokens;

}

