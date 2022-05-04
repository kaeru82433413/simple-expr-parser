use crate::structures::{Expression, Parentheses, Operator};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExceptedExpr(Option<char>, usize),
    ExceptedOp(char, usize),
    InvalidCloseParenthese(usize),
    UncloseParentheses,
    Overflow(String),
}

type ParseResult<T> = Result<T, ParseError>;


pub fn parse(mut input: &str) -> ParseResult<Expression> {
    let mut offset = 0;
    skip_whitespace(&mut input, &mut offset);
    parse_paren(&mut input, &mut offset, true).map(|p| Expression::from(p))
}

fn next(text: &mut &str, offset: &mut usize) {
    let c = text.chars().next().unwrap();
    *text = &text[c.len_utf8()..];
    *offset += c.len_utf8();
    skip_whitespace(text, offset);
}

fn skip_whitespace(text: &mut &str, offset: &mut usize) {
    while let Some(c) = text.chars().next() {
        if !c.is_whitespace() {
            break;
        }
        next(text, offset);
    }
}

fn parse_expr(text: &mut &str, offset: &mut usize) -> ParseResult<Expression> {
    if let Some(c) = text.chars().next() {
        if c.is_ascii_digit() {
            let mut num = String::new();
            while let Some(c) = text.chars().next() {
                if c.is_ascii_digit() {
                    next(text, offset);
                    num.push(c);
                } else {
                    break;
                }
            }
            if let Ok(num) = num.parse::<u64>() {
                Ok(Expression::from(num))
            } else {
                Err(ParseError::Overflow(num))
            }
        } else if c == '(' {
            next(text, offset);
            Ok(Expression::from(parse_paren(text, offset, false)?))
        } else {
            Err(ParseError::ExceptedExpr(Some(c), *offset))
        }
    } else {
        return Err(ParseError::ExceptedExpr(None, *offset))
    }
}

fn parse_paren(text: &mut &str, offset: &mut usize, outermost: bool) -> ParseResult<Parentheses> {
    let mut exprs = vec![Expression::from(parse_expr(text, offset)?)];
    let mut ops = vec![];
    
    while let Some(c) = text.chars().next() {
        if let Some(op) = Operator::from_char(c) {
            next(text, offset);
            ops.push(op);
            
            exprs.push(parse_expr(text, offset)?);
            
        } else if c == ')' {
            if !outermost {
                next(text, offset);
                return Ok(Parentheses::new(exprs, ops));
            } else {
                return Err(ParseError::InvalidCloseParenthese(*offset));
            }
        } else {
            return Err(ParseError::ExceptedOp(c, *offset));
        }
    }
    
    if outermost {
        Ok(Parentheses::new(exprs, ops))
    } else {
        Err(ParseError::UncloseParentheses)
    }
}


#[test]
fn test() {
    let one = Expression::from(1);
    let two = Expression::from(2);
    let three = Expression::from(3);
    let a = Expression::from(Parentheses::new(vec![one.clone(), two.clone(), three.clone()], vec![Operator::Add, Operator::Mul]));
    let b = Expression::from(Parentheses::new(vec![a.clone(), two.clone()], vec![Operator::Mul]));
    let c = Expression::from(Parentheses::new(vec![one.clone(), a.clone(), two.clone(), b.clone(), three.clone()], vec![Operator::Add, Operator::Sub, Operator::Mul, Operator::Div]));
    let d = Expression::from(Parentheses::new(vec![Expression::from(Parentheses::new(vec![one.clone()], vec![]))], vec![]));

    assert_eq!(parse("1+2*3"), Ok(a));
    assert_eq!(parse("( 1 +2*3) *2"), Ok(b));
    assert_eq!(parse("1 + (1+2*3) - 2 * ((1+2*3) * 2 ) / 3"), Ok(c));
    assert_eq!(parse("(1)"), Ok(d));

    assert_eq!(parse(""), Err(ParseError::ExceptedExpr(None, 0)));
    assert_eq!(parse("a"), Err(ParseError::ExceptedExpr(Some('a'), 0)));
    assert_eq!(parse("(0"), Err(ParseError::UncloseParentheses));
    assert_eq!(parse("(0+)"), Err(ParseError::ExceptedExpr(Some(')'), 3)));
}