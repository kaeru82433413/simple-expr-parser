use std::io::stdin;
use simple_expr_parser::{parse::{parse, ParseError}, structures::EvaluationError};
use unicode_width::UnicodeWidthStr;

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        let res = parse(&input);
        match res {
            Ok(expr) => {
                match expr.eval() {
                    Ok(result) => println!("{}", result),
                    Err(err) => {
                        match err {
                            EvaluationError::Overflow => println!("途中計算に算術オーバーフローが発生しました。"),
                            EvaluationError::ZeroDivision => println!("途中計算にゼロ除算が発生しました。"),
                        }
                    }
                }
            },
            Err(err) => {
                match err {
                    ParseError::ExceptedExpr(_, len) => {
                        let at = input[..len].width();
                        println!("{}^ 式が期待されます。", " ".repeat(at));
                    },
                    ParseError::ExceptedOp(_, len) => {
                        let at = input[..len].width();
                        println!("{}^ 演算子または閉じ括弧が期待されます。", " ".repeat(at));
                    },
                    ParseError::InvalidCloseParenthese(len) => {
                        let at = input[..len].width();
                        println!("{}^ 対応する開き括弧がありません。", " ".repeat(at));
                    },
                    ParseError::UncloseParentheses => {
                        println!("括弧が閉じられていません。");
                    },
                    ParseError::Overflow(num) => {
                        println!("{}は大きすぎて計算不能です。", num);
                    }
                }
            },
        }
    }
}
