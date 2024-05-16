use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(Ok(SyntaxKind::Number)) | Some(Ok(SyntaxKind::Ident)) => p.bump(),
        Some(Ok(SyntaxKind::Minus)) => {
            let op = PrefixOp::Neg;
            let ((), right_binding_power) = op.binding_power();

            p.bump();

            p.start_node_at(checkpoint, SyntaxKind::PrefixExpr);
            expr_binding_power(p, right_binding_power);
            p.finish_node();
        }
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(Ok(SyntaxKind::Plus)) => InfixOp::Add,
            Some(Ok(SyntaxKind::Minus)) => InfixOp::Sub,
            Some(Ok(SyntaxKind::Star)) => InfixOp::Mul,
            Some(Ok(SyntaxKind::Slash)) => InfixOp::Div,
            _ => return,
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinaryExpr);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl InfixOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
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
          BinaryExpr@0..3
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
              BinaryExpr@0..7
                BinaryExpr@0..5
                  BinaryExpr@0..3
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
              BinaryExpr@0..7
                BinaryExpr@0..5
                  Number@0..1 "1"
                  Plus@1..2 "+"
                  BinaryExpr@2..5
                    Number@2..3 "2"
                    Star@3..4 "*"
                    Number@4..5 "3"
                Minus@5..6 "-"
                Number@6..7 "4""#]],
        );
    }
    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
            Root@0..3
              PrefixExpr@0..3
                Minus@0..1 "-"
                Number@1..3 "10""#]],
        );
    }
    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
            Root@0..6
              BinaryExpr@0..6
                PrefixExpr@0..3
                  Minus@0..1 "-"
                  Number@1..3 "20"
                Plus@3..4 "+"
                Number@4..6 "20""#]],
        );
    }
}
