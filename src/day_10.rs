pub fn print_answers() {
    println!("\n--- Day 10: Syntax Scoring ---");
    let input = std::fs::read_to_string("assets\\day_10_input.txt").unwrap();
    println!(
        "What is the total syntax error score for those errors? {}",
        SyntaxChecker::get_syntax_error_score(&input)
    );
    println!(
        "What is the middle score? {}",
        SyntaxChecker::get_autocomplete_score(&input)
    )
}

struct SyntaxChecker {}

#[derive(Debug)]
struct SyntaxError {
    token: char,
    message: String,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl SyntaxChecker {
    fn get_syntax_error_score(input: &str) -> u64 {
        let mut illegal_tokens = Vec::new();
        for chunk in input.lines() {
            if let Err(e) = Self::parse_chunk(chunk) {
                illegal_tokens.push(e.token);
            }
        }
        illegal_tokens
            .iter()
            .map(|&c| match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            })
            .sum()
    }

    fn get_autocomplete_score(input: &str) -> u64 {
        let mut scores = Vec::new();
        for chunk in input.lines() {
            if let Ok(completion) = Self::parse_chunk(chunk) {
                scores.push(completion.chars().fold(0, |acc, c| {
                    acc * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                }));
            }
        }
        scores.sort();
        scores[(scores.len() - 1) / 2]
    }

    fn parse_chunk(chunk: &str) -> Result<String, SyntaxError> {
        let mut stack = Vec::new();
        for token in chunk.chars() {
            match token {
                '(' | '[' | '{' | '<' => {
                    stack.push(token);
                }
                ')' | ']' | '}' | '>' => {
                    let open_token = stack[stack.len() - 1];
                    let close_token = Self::get_closing_token(open_token);
                    if token == close_token {
                        let _ = stack.pop();
                    } else {
                        return Err(SyntaxError {
                            token,
                            message: format!(
                                "expected {}, but found {} instead",
                                close_token, token
                            ),
                        });
                    }
                }
                _ => {
                    return Err(SyntaxError {
                        token,
                        message: format!("invalid token: {}", token),
                    })
                }
            }
        }
        Ok(stack
            .iter()
            .rev()
            .map(|&c| Self::get_closing_token(c))
            .collect::<String>())
    }

    fn get_closing_token(token: char) -> char {
        match token {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("invalid token: {}", token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_syntax_error_score() {
        assert_eq!(SyntaxChecker::get_syntax_error_score(INPUT), 26397);
    }

    #[test]
    fn test_autocomplete_score() {
        assert_eq!(SyntaxChecker::get_autocomplete_score(INPUT), 288957);
    }
}
