use fraction::{Fraction, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
use std::collections::VecDeque as Deque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub type EvaluationResult = Result<Fraction, EvaluationError>;

impl Operator {
    fn apply(self, left: Fraction, right: Fraction) -> EvaluationResult {
        match self {
            Self::Add => left.checked_add(&right).ok_or(EvaluationError::Overflow),
            Self::Sub => left.checked_sub(&right).ok_or(EvaluationError::Overflow),
            Self::Mul => left.checked_mul(&right).ok_or(EvaluationError::Overflow),
            Self::Div => {
                let raw = left.checked_div(&right).ok_or(EvaluationError::Overflow)?;
                match raw {
                    Fraction::Rational(_, _) => Ok(raw),
                    _ => Err(EvaluationError::ZeroDivision),
                }
            },
        }
    }

    fn precedence(self) -> usize {
        match self {
            Self::Mul | Self::Div => 0,
            Self::Add | Self::Sub => 1,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvaluationError {
    ZeroDivision,
    Overflow,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Num(u64),
    Parentheses(Parentheses),
}

impl Expression {
    pub fn eval(&self) -> EvaluationResult {
        Ok(match self {
            Self::Num(value) => Fraction::from(*value),
            Self::Parentheses(parenthese) => parenthese.eval()?,
        })
    }
}

impl From<u64> for Expression {
    fn from(frac: u64) -> Self {
        Expression::Num(frac)
    }
}
impl From<Parentheses> for Expression {
    fn from(parentheses: Parentheses) -> Self {
        Expression::Parentheses(parentheses)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Parentheses {
    exprs: Vec<Expression>,
    operators: Vec<Operator>,
}

impl Parentheses {
    pub fn new(exprs: Vec<Expression>, operators: Vec<Operator>) -> Self {
        if exprs.len() != operators.len()+1 {
            panic!()
        }
        Self {
            exprs, operators,
        }
    }

    fn apply_ops(values: &mut Deque<Fraction>, ops: &mut Deque<Operator>, precedence: usize) -> Result<(), EvaluationError> {
        let mut res_values = Deque::from(vec![values.pop_front().unwrap()]);
        let mut res_ops = Deque::new();

        while let (Some(value), Some(op)) = (values.pop_front(), ops.pop_front()) {
            if op.precedence() == precedence {
                let value = op.apply(res_values.pop_back().unwrap(), value);
                res_values.push_back(value?);
            } else {
                res_values.push_back(value);
                res_ops.push_back(op);
            }
        }

        *values = res_values;
        * ops = res_ops;
        Ok(())
    }

    pub fn eval(&self) -> EvaluationResult {
        let mut values = Deque::new();
        for expr in self.exprs.iter() {
            values.push_back(expr.eval()?);
        }
        let mut ops: Deque<_> = self.operators.iter().copied().collect();

        for prc in 0..2 {
            Self::apply_ops(&mut values, &mut ops, prc)?;
        }
        Ok(values[0])
    }
}


#[test]
fn test() {
    let one = Expression::from(1);
    let two = Expression::from(2);
    let three = Expression::from(3);

    let a = Expression::from(Parentheses::new(
        vec![one.clone(), one.clone()], vec![Operator::Add]
    )); // 1+1
    assert_eq!(a.eval(), two.eval());

    let b = Expression::from(Parentheses::new(
        vec![one.clone(), two.clone()], vec![Operator::Add]
    )); // 1+2
    assert_eq!(b.eval(), three.eval());

    let c = Expression::from(Parentheses::new(
        vec![three.clone(), one.clone()], vec![Operator::Sub]
    )); // 3-1
    assert_eq!(c.eval(), two.eval());

    let d = Expression::from(Parentheses::new(
        vec![two.clone(), one.clone()], vec![Operator::Mul]
    )); // 2*1
    assert_eq!(d.eval(), two.eval());

    let e = Expression::from(Parentheses::new(
        vec![one.clone(), two.clone(), two.clone()], vec![Operator::Div, Operator::Mul]
    )); // 1/2*2
    assert_eq!(e.eval(), one.eval());

    let f = Expression::from(Parentheses::new(
        vec![e.clone(), c.clone()], vec![Operator::Mul],
    )); // (1/2*2)*(3-1);
    assert_eq!(f.eval(), two.eval());

    let g = Expression::from(Parentheses::new(
        vec![one.clone(), e.clone()], vec![Operator::Div],
    )); // 1/(1/2);
    assert_eq!(g.eval(), one.eval());

    let h = Expression::from(Parentheses::new(
        vec![three.clone(), one.clone(), two.clone()], vec![Operator::Sub, Operator::Mul]
    )); // 3-1*2
    assert_eq!(h.eval(), one.eval());

    let i = Expression::from(Parentheses::new(
        vec![Expression::from(10u64.pow(10)), Expression::from(10u64.pow(10))], vec![Operator::Mul]
    )); // 10000000000*10000000000
    assert_eq!(i.eval(), Err(EvaluationError::Overflow));

    let j = Expression::from(Parentheses::new(
        vec![one.clone(), Expression::from(0)], vec![Operator::Div]
    )); // 1/0
    assert_eq!(j.eval(), Err(EvaluationError::ZeroDivision));
}