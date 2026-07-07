//! A simple boolean expression evaluator crate
//! 
//! This crate provides functionality to parse and evaluate boolean expressions using Reverse Polish Notation (RPN).
//! 
//! # Quickstart
//! 
//! ```
//! use evalbit::{parse, eval, RPNTrait};
//! 
//! let expr = "0 ^ (1 | !2)";
//! let args = &[true, false, true];
//! let rpn = parse(expr);
//! rpn.print(); // => 0 1 2 ! | ^
//! assert_eq!(rpn.exec(args), true); // true ^ (false | !true) = true
//! 
//! // and you can directly evaluate expressions
//! assert_eq!(eval(expr, args), true);
//! ```

use std::collections::VecDeque;

#[derive(PartialEq, Clone)]
enum Token {
    Index(usize),
    Not,
    And,
    Xor,
    Or,
    LPar,
    RPar,
}

/// Reverse Polish Notation Item
/// 
/// `Vec<RPNItem>` has [RPNTrait] implemented
#[derive(PartialEq)]
pub enum RPNItem {
    /// Index of the argument
    Index(usize),
    /// Unary NOT operator
    Not,
    /// Binary AND operator
    And,
    /// Binary XOR operator
    Xor,
    /// Binary OR operator
    Or,
}

/// Trait for Reverse Polish Notation expressions
pub trait RPNTrait {
    /// Convert RPN expression to string
    /// 
    /// # Example
    /// 
    /// ```
    /// use evalbit::RPNTrait;
    /// 
    /// let expr = "0 ^ (1 | !2)";
    /// let rpn = evalbit::parse(expr);
    /// assert_eq!(rpn.to_string(), "0 1 2 ! | ^");
    /// ```
    fn to_string(&self) -> String;
    /// Print RPN expression
    /// 
    /// # Example
    /// 
    /// ```
    /// use evalbit::RPNTrait;
    /// 
    /// let expr = "0 ^ (1 | !2)";
    /// let rpn = evalbit::parse(expr);
    /// rpn.print(); // => 0 1 2 ! | ^
    /// ```
    fn print(&self);
    /// Execute RPN expression with given boolean arguments
    /// 
    /// # Example
    /// 
    /// ```
    /// use evalbit::RPNTrait;
    /// 
    /// let expr = "0 ^ (1 | !2)";
    /// let rpn = evalbit::parse(expr);
    /// let args = vec![true, false, true];
    /// let result = rpn.exec(&args);
    /// assert_eq!(result, true); // true ^ (false | !true) = true
    /// ```
    fn exec(&self, args: &[bool]) -> bool;
}

impl RPNTrait for Vec<RPNItem> {
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

/// Parse boolean expression into Reverse Polish Notation
/// 
/// # Example
/// 
/// ```
/// use evalbit::RPNTrait;
/// 
/// let expr = "0 ^ (1 | !2)";
/// let rpn = evalbit::parse(expr);
/// assert_eq!(rpn.to_string(), "0 1 2 ! | ^");
/// ```
pub fn parse(expr: &str) -> Vec<RPNItem> {
    let tokens = tokenize(expr);
    rpn(&tokens)
}

/// Evaluate boolean expression with given arguments
///
/// # Example
/// 
/// ```
/// let expr = "0 ^ (1 | !2)";
/// let args = vec![true, false, true];
/// let result = evalbit::eval(expr, &args);
/// assert_eq!(result, true); // true ^ (false | !true) = true
/// ```
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

fn rpn(tokens: &[Token]) -> Vec<RPNItem> {
    let mut result = Vec::<RPNItem>::new();
    let mut ops = Vec::<Token>::new();
    
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
            Token::Index(n) => result.push(RPNItem::Index(*n)),
            Token::Not | Token::And | Token::Xor | Token::Or => {
                while let Some(top) = ops.last() {
                    if *top != Token::LPar && precedence(top) >= precedence(token) && token != &Token::Not {
                        let op = ops.pop().unwrap();
                        result.push(rpnop(&op).unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(token.clone());
            },
            Token::LPar => ops.push(Token::LPar),
            Token::RPar => {
                while let Some(top) = ops.pop() {
                    if top == Token::LPar {
                        break;
                    } else {
                        result.push(rpnop(&top).unwrap());
                    }
                }
            },
        }
    }
    while let Some(op) = ops.pop() {
        result.push(rpnop(&op).unwrap());
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
