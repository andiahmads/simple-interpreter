#[derive(Debug)]
enum TokenType {
    NumberLiteral, // 1,2,3,4,5
    Identifier,    // a,b,c
    Equal,
    Plus,
    Minus,
    Slash,
    Star,
    LeftParent,
    RightParent,
    NewLine,
}

use TokenType::*;

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String, // 123 -> lexeme "123"
}

fn tokenizer(source_code: &str) -> Vec<Token> {
    let mut position = 0;
    let mut result = Vec::new();
    while position < source_code.len() {
        let current_char = source_code.chars().nth(position).unwrap();
        match current_char {
            '=' => result.push(Token {
                token_type: Equal,
                lexeme: "=".to_string(),
            }),
            '*' => result.push(Token {
                token_type: Star,
                lexeme: "*".to_string(),
            }),
            '/' => result.push(Token {
                token_type: Slash,
                lexeme: "/".to_string(),
            }),

            '+' => result.push(Token {
                token_type: Plus,
                lexeme: "+".to_string(),
            }),
            '-' => result.push(Token {
                token_type: Minus,
                lexeme: "-".to_string(),
            }),
            '(' => result.push(Token {
                token_type: RightParent,
                lexeme: "(".to_string(),
            }),
            ')' => result.push(Token {
                token_type: LeftParent,
                lexeme: ")".to_string(),
            }),
            '\n' => result.push(Token {
                token_type: NewLine,
                lexeme: "\n".to_string(),
            }),

            x if x.is_digit(10) => {
                let mut number_lexeme = x.to_string();
                position += 1;
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if next_char == ' ' || next_char == ')' || next_char == '\n' {
                        break;
                    }

                    if next_char.is_digit(10) {
                        number_lexeme.push(next_char);
                    } else {
                        panic!("invalid character: '{}'", next_char);
                    }
                    position += 1;
                }

                result.push(Token {
                    token_type: NumberLiteral,
                    lexeme: number_lexeme,
                });
                continue; // we don't to consumer the next character /after/ the number
            }
            ' ' => {}
            c => {
                // ex: print()
                // identifier if you don't have any keywords
                let mut lexeme = c.to_string();
                position += 1;
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if !next_char.is_numeric() {
                        break;
                    }
                    lexeme.push(next_char);
                    position += 1;
                }
                result.push(Token {
                    token_type: Identifier,
                    lexeme,
                });
                continue;
            } // _ => todo!(),
        }
        position += 1;
    }
    return result;
}

fn main() {
    let src = "a = 123 
            print(a)";

    let tokens = tokenizer(src);
    for token in tokens {
        println!("{:?}", token)
    }
}
