use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    expr_binding_powe(p, 0);
}

fn expr_binding_powe(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(Ok(SyntaxKind::Number)) | Some(Ok(SyntaxKind::Ident)) => p.bump(),
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(Ok(SyntaxKind::Plus)) => Op::Add,
            Some(Ok(SyntaxKind::Minus)) => Op::Sub,
            Some(Ok(SyntaxKind::Star)) => Op::Mul,
            Some(Ok(SyntaxKind::Slash)) => Op::Div,
            _ => return,
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinOp);
        expr_binding_powe(p, right_binding_power);
        p.finish_node();
    }
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![
                r#"
                Root@0..3
                  Number@0..3 "123""#
            ],
        )
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "abc",
            expect![
                r#"
        Root@0..3
          Ident@0..3 "abc""#
            ],
        )
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![
                r#"
        Root@0..3
          BinOp@0..3
            Number@0..1 "1"
            Plus@1..2 "+"
            Number@2..3 "2""#
            ],
        )
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
            Root@0..7
              BinOp@0..7
                BinOp@0..5
                  BinOp@0..3
                    Number@0..1 "1"
                    Plus@1..2 "+"
                    Number@2..3 "2"
                  Plus@3..4 "+"
                  Number@4..5 "3"
                Plus@5..6 "+"
                Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
            Root@0..7
              BinOp@0..7
                BinOp@0..5
                  Number@0..1 "1"
                  Plus@1..2 "+"
                  BinOp@2..5
                    Number@2..3 "2"
                    Star@3..4 "*"
                    Number@4..5 "3"
                Minus@5..6 "-"
                Number@6..7 "4""#]],
        );
    }
}
