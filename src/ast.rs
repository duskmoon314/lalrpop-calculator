#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// ADD `+`
    ADD,
    /// SUB `-`
    SUB,
    /// MUL `*`
    MUL,
    /// DIV `/`
    DIV,
    /// REM `%`
    REM,
    /// AND `&&`
    AND,
    /// OR `||`
    OR,
    /// NOT `!`
    NOT,
    /// EQ `==`
    EQ,
    /// NEQ `!=`
    NEQ,
    /// LT `<`
    LT,
    /// GT `>`
    GT,
    /// LE `<=`
    LE,
    /// GE `>=`
    GE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// `Number(f64)` stores a `f64` number
    Number(f64),
    /// `Unary` stores unary operation, like `!` or `-`
    Unary(Operator, Box<Expression>),
    /// `Binary` stores binary operation, like `+` `-` `*` `/` and so on
    Binary(Box<Expression>, Operator, Box<Expression>),
    /// `Ternary` stores question mark expression: `cond ? true-expr : false-expr`
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    /// `Variable` stores variable, not implemented yet
    Variable(String),
    /// `Function` stores function, and will used to call f64 function of Rust
    Function(String, Vec<Box<Expression>>),
}

impl Expression {
    pub fn eval(&self) -> f64 {
        match self {
            Expression::Number(x) => *x,
            Expression::Unary(Operator::SUB, expr) => -expr.eval(),
            Expression::Unary(Operator::NOT, expr) => {
                if expr.eval() != 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::ADD, rexpr) => lexpr.eval() + rexpr.eval(),
            Expression::Binary(lexpr, Operator::SUB, rexpr) => lexpr.eval() - rexpr.eval(),
            Expression::Binary(lexpr, Operator::MUL, rexpr) => lexpr.eval() * rexpr.eval(),
            Expression::Binary(lexpr, Operator::DIV, rexpr) => lexpr.eval() / rexpr.eval(),
            Expression::Binary(lexpr, Operator::REM, rexpr) => lexpr.eval() % rexpr.eval(),
            Expression::Binary(lexpr, Operator::AND, rexpr) => {
                if lexpr.eval() != 0.0 && rexpr.eval() != 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::OR, rexpr) => {
                if lexpr.eval() != 0.0 || rexpr.eval() != 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::EQ, rexpr) => {
                if lexpr.eval() == rexpr.eval() {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::NEQ, rexpr) => {
                if lexpr.eval() != rexpr.eval() {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::LT, rexpr) => {
                if lexpr.eval() < rexpr.eval() {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::LE, rexpr) => {
                if lexpr.eval() <= rexpr.eval() {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::GT, rexpr) => {
                if lexpr.eval() > rexpr.eval() {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Binary(lexpr, Operator::GE, rexpr) => {
                if lexpr.eval() >= rexpr.eval() {
                    1.0
                } else {
                    0.0
                }
            }
            Expression::Ternary(cond, texpr, fexpr) => {
                if cond.eval() != 0.0 {
                    texpr.eval()
                } else {
                    fexpr.eval()
                }
            }
            Expression::Variable(_) => 0.0,
            Expression::Function(fun, exprs) => match fun.as_str() {
                "floor" => f64::floor(exprs[0].eval()),
                "ceil" => f64::ceil(exprs[0].eval()),
                "round" => f64::round(exprs[0].eval()),
                "trunc" => f64::trunc(exprs[0].eval()),
                "fract" => f64::fract(exprs[0].eval()),
                "abs" => f64::abs(exprs[0].eval()),
                "signum" => f64::signum(exprs[0].eval()),
                "mul_add" => f64::mul_add(exprs[0].eval(), exprs[1].eval(), exprs[2].eval()),
                "div_euclid" => f64::div_euclid(exprs[0].eval(), exprs[1].eval()),
                "rem_euclid" => f64::rem_euclid(exprs[0].eval(), exprs[1].eval()),
                "powf" => f64::powf(exprs[0].eval(), exprs[1].eval()),
                "sqrt" => f64::sqrt(exprs[0].eval()),
                "exp" => f64::exp(exprs[0].eval()),
                "exp2" => f64::exp2(exprs[0].eval()),
                "ln" => f64::ln(exprs[0].eval()),
                "log" => f64::log(exprs[0].eval(), exprs[1].eval()),
                "log2" => f64::log2(exprs[0].eval()),
                "log10" => f64::log10(exprs[0].eval()),
                "cbrt" => f64::cbrt(exprs[0].eval()),
                "hypot" => f64::hypot(exprs[0].eval(), exprs[1].eval()),
                "sin" => f64::sin(exprs[0].eval()),
                "cos" => f64::cos(exprs[0].eval()),
                "tan" => f64::tan(exprs[0].eval()),
                "asin" => f64::asin(exprs[0].eval()),
                "acos" => f64::acos(exprs[0].eval()),
                "atan" => f64::atan(exprs[0].eval()),
                "atan2" => f64::atan2(exprs[0].eval(), exprs[1].eval()),
                "exp_m1" => f64::exp_m1(exprs[0].eval()),
                "ln_1p" => f64::ln_1p(exprs[0].eval()),
                "sinh" => f64::sinh(exprs[0].eval()),
                "cosh" => f64::cosh(exprs[0].eval()),
                "tanh" => f64::tanh(exprs[0].eval()),
                "asinh" => f64::asinh(exprs[0].eval()),
                "acosh" => f64::acosh(exprs[0].eval()),
                "atanh" => f64::atanh(exprs[0].eval()),
                _ => panic!("Not support function"),
            },

            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ast::*, calculator};
    #[test]
    fn test_factor() {
        assert_eq!(
            calculator::FactorParser::new()
                .parse("2.5e10")
                .unwrap()
                .eval(),
            2.5e10
        );
        assert_eq!(
            calculator::FactorParser::new()
                .parse("!2.5e10")
                .unwrap()
                .eval(),
            1.0
        );
        assert_eq!(
            calculator::FactorParser::new().parse("abc").unwrap(),
            Box::new(Expression::Variable("abc".to_string()))
        );
        assert_eq!(
            calculator::FactorParser::new().parse("abc(1, 2)").unwrap(),
            Box::new(Expression::Function(
                "abc".to_string(),
                vec![
                    Box::new(Expression::Number(1.0)),
                    Box::new(Expression::Number(2.0))
                ]
            ))
        );
        assert_eq!(
            calculator::FactorParser::new()
                .parse("abs(-1)")
                .unwrap()
                .eval(),
            1.0
        )
    }

    #[test]
    fn test_multiplicative() {
        assert_eq!(
            calculator::MultiplicativeParser::new()
                .parse("1 * 2")
                .unwrap()
                .eval(),
            2.0
        )
    }

    #[test]
    fn test_additive() {
        assert_eq!(
            calculator::AdditiveParser::new()
                .parse("1 * 2 + 3")
                .unwrap()
                .eval(),
            5.0
        )
    }

    #[test]
    fn test_relational() {
        assert_eq!(
            calculator::RelationalParser::new()
                .parse("4 < 1 * 2 + 3")
                .unwrap()
                .eval(),
            1.0
        )
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            calculator::EqualityParser::new()
                .parse("1 * 2 + 3 == 5")
                .unwrap()
                .eval(),
            1.0
        )
    }

    #[test]
    fn test_logic_and() {
        assert_eq!(
            calculator::LogicAndParser::new()
                .parse("1 * 2 + 3 && 4")
                .unwrap()
                .eval(),
            1.0
        )
    }

    #[test]
    fn test_logic_or() {
        assert_eq!(
            calculator::LogicOrParser::new()
                .parse("1 * 2 + 3 || 0")
                .unwrap()
                .eval(),
            1.0
        )
    }

    #[test]
    fn test_conditional() {
        assert_eq!(
            calculator::ConditionalParser::new()
                .parse("1 ? 2 : 3")
                .unwrap()
                .eval(),
            2.0
        )
    }

    #[test]
    fn test_expression() {
        assert_eq!(
            calculator::ExprParser::new()
                .parse("1 * 2 + 3 == 5 ? (6 < 4 || 7 >= 7) : (8 != 8 && 9 >= 10)")
                .unwrap()
                .eval(),
            1.0
        )
    }
}
