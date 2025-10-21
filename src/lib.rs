use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
enum Token {
    Index(usize),
    Not,
    And,
    Xor,
    Or,
    LPar,
    RPar,
}

#[derive(PartialEq, Debug)]
pub enum RPNItem {
    Index(usize),
    Not,
    And,
    Xor,
    Or,
}

pub trait RPNTrait {
    fn to_string(&self) -> String;
    fn print(&self);
    fn exec(&self, args: &[bool]) -> bool;
}

impl RPNTrait for VecDeque<RPNItem> {
    fn to_string(&self) -> String {
        self.iter().map(|item| match item {
            RPNItem::Index(n) => n.to_string(),
            RPNItem::Not => "!".to_string(),
            RPNItem::And => "&".to_string(),
            RPNItem::Xor => "^".to_string(),
            RPNItem::Or => "|".to_string(),
        }).collect::<Vec<String>>().join(" ")
    }
    fn print(&self) {
        println!("{}", self.to_string());
    }
    fn exec(&self, args: &[bool]) -> bool {
        let mut stack = Vec::<bool>::new();
        for item in self {
            match item {
                RPNItem::Index(n) => stack.push(args[*n]),
                RPNItem::Not => {
                    let v = stack.pop().unwrap();
                    stack.push(!v);
                },
                RPNItem::And => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a & b);
                },
                RPNItem::Xor => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a ^ b);
                },
                RPNItem::Or => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a | b);
                },
            }
        }
        stack.pop().unwrap()
    }
}

pub fn parse(expr: &str) -> VecDeque<RPNItem> {
    let tokens = tokenize(expr);
    rpn(&tokens)
}
pub fn eval(expr: &str, args: &[bool]) -> bool {
    let rpn_expr = parse(expr);
    rpn_expr.exec(args)
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                let mut num = chars[i] as usize - '0' as usize;
                while i + 1 < chars.len() && chars[i+1].is_digit(10) {
                    i += 1;
                    num = num * 10 + (chars[i] as usize - '0' as usize);
                }
                tokens.push(Token::Index(num));
            },
            '!' => tokens.push(Token::Not),
            '&' => tokens.push(Token::And),
            '^' => tokens.push(Token::Xor),
            '|' => tokens.push(Token::Or),
            '(' => tokens.push(Token::LPar),
            ')' => tokens.push(Token::RPar),
            ' ' => {},
            _ => panic!("Unexpected character: {}", chars[i]),
        }
        i += 1;
    }

    tokens
}

fn rpn(tokens: &[Token]) -> VecDeque<RPNItem> {
    let mut result = VecDeque::<RPNItem>::new();
    let mut ops = VecDeque::<Token>::new();
    let precedence = |op: &Token| match op {
        Token::Not => 3,
        Token::And => 2,
        Token::Xor => 1,
        Token::Or => 0,
        _ => usize::MAX,
    };
    let rpnop = |op: &Token| match op {
        Token::Not => Some(RPNItem::Not),
        Token::And => Some(RPNItem::And),
        Token::Xor => Some(RPNItem::Xor),
        Token::Or => Some(RPNItem::Or),
        _ => None,
    };
    for token in tokens {
        match token {
            Token::Index(n) => result.push_back(RPNItem::Index(*n)),
            Token::Not | Token::And | Token::Xor | Token::Or => {
                while let Some(top) = ops.back() {
                    if *top != Token::LPar && precedence(top) >= precedence(token) && token != &Token::Not {
                        let op = ops.pop_back().unwrap();
                        result.push_back(rpnop(&op).unwrap());
                    } else {
                        break;
                    }
                }
                ops.push_back(token.clone());
            },
            Token::LPar => ops.push_back(Token::LPar),
            Token::RPar => {
                while let Some(top) = ops.pop_back() {
                    if top == Token::LPar {
                        break;
                    } else {
                        result.push_back(rpnop(&top).unwrap());
                    }
                }
            },
        }
    }
    while let Some(op) = ops.pop_back() {
        result.push_back(rpnop(&op).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_rpn() {
        let expr = "0 ^ !(1 | 2 & 3 ^ 4) & 5 | !!6 & 7";
        let result = parse(expr).to_string();
        let expected = "0 1 2 3 & 4 ^ | ! 5 & ^ 6 ! ! 7 & |".to_string();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_eval() {
        let expr = "0 ^ !(1 | 2 & 3 ^ 4) & 5 | !!6 & 7";
        let rpn = parse(expr);
        for i in 0..256 {
            let args = vec![
                (i & 0b00000001) != 0,
                (i & 0b00000010) != 0,
                (i & 0b00000100) != 0,
                (i & 0b00001000) != 0,
                (i & 0b00010000) != 0,
                (i & 0b00100000) != 0,
                (i & 0b01000000) != 0,
                (i & 0b10000000) != 0,
            ];
            let expected = args[0] ^ !(args[1] | args[2] & args[3] ^ args[4]) & args[5] | !!args[6] & args[7];
            let result = rpn.exec(&args);
            assert_eq!(result, expected, "Failed for args: {:?}", args);
        }
    }
}
