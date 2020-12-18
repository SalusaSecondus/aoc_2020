use std::fmt::Display;

use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
enum MathToken {
    Constant(i64),
    Addition,
    Multiplication,
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug, Clone)]
enum MathExpression {
    Constant(i64),
    Operation(MathToken),
    Parens(MathFormula),
}

impl Display for MathToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathToken::Constant(val) => write!(f, "{}", val),
            MathToken::Addition => write!(f, " + "),
            MathToken::Multiplication => write!(f, " * "),
            MathToken::OpenParenthesis => write!(f, "("),
            MathToken::CloseParenthesis => write!(f, ")"),
        }
    }
}

impl Display for MathExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathExpression::Constant(val) => write!(f, "{}", val),
            MathExpression::Operation(op) => write!(f, "{}", op),
            MathExpression::Parens(formula) => write!(f, "({})", formula),
        }
    }
}

impl MathToken {
    fn parse_line(line: &str) -> Result<Vec<MathToken>> {
        lazy_static! {
            static ref TOKEN_RE: Regex = Regex::new(r"((?:\d+)|[+*()])").unwrap();
        }
        let mut result = vec![];

        for token in TOKEN_RE.captures_iter(line) {
            let next_token = token.get(0).unwrap().as_str();
            let next_token = match next_token {
                "+" => MathToken::Addition,
                "*" => MathToken::Multiplication,
                "(" => MathToken::OpenParenthesis,
                ")" => MathToken::CloseParenthesis,
                _ => MathToken::Constant(next_token.parse()?),
            };
            result.push(next_token);
        }

        Ok(result)
    }
}

impl MathExpression {
    fn evaluate(&self) -> Result<i64> {
        // println!("Evaluating self: {:?}",  self);
        match self {
            MathExpression::Constant(val) => Ok(*val),
            MathExpression::Parens(paren_value) => paren_value.evaluate(),
            _ => bail!("Unexpected expression"),
        }
    }

    fn apply_precedence(&self) -> Result<MathExpression> {
        if let MathExpression::Parens(inner_formula) = self {
            if inner_formula.expressions.len() == 1 {
                Ok(MathExpression::Parens(inner_formula.to_owned()))
            } else {
                Ok(MathExpression::Parens(inner_formula.apply_precedence()?))
            }
        } else {
            Ok(self.to_owned())
        }
    }
}

#[derive(Debug, Clone)]
struct MathFormula {
    expressions: Vec<MathExpression>,
}

impl Display for MathFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for exp in &self.expressions {
            write!(f, "{}", exp)?;
        }
        Ok(())
    }
}

impl MathFormula {
    fn parse_tokens(tokens: &[MathToken]) -> Result<MathFormula> {
        let mut iterator = tokens.iter();
        let expressions = MathFormula::parse_tokens_inner(&mut iterator)?;
        Ok(MathFormula { expressions })
    }

    fn parse_tokens_inner<'a, I>(tokens: &mut I) -> Result<Vec<MathExpression>>
    where
        I: Iterator<Item = &'a MathToken>,
    {
        let mut result = vec![];
        while let Some(token) = tokens.next() {
            let expression = match token {
                MathToken::Addition => MathExpression::Operation(MathToken::Addition),
                MathToken::Multiplication => MathExpression::Operation(MathToken::Multiplication),
                MathToken::Constant(val) => MathExpression::Constant(*val),
                MathToken::OpenParenthesis => MathExpression::Parens(MathFormula {
                    expressions: MathFormula::parse_tokens_inner(tokens)?,
                }),
                MathToken::CloseParenthesis => break,
            };
            result.push(expression);
        }

        Ok(result)
    }

    fn evaluate(&self) -> Result<i64> {
        let mut accumulator = 0;
        let mut expressions = self.expressions.iter();
        while let Some(exp) = expressions.next() {
            // println!("Accumulator: {}", accumulator);
            if let MathExpression::Operation(op) = exp {
                accumulator = match op {
                    MathToken::Addition => {
                        accumulator + expressions.next().context("Missing token")?.evaluate()?
                    }
                    MathToken::Multiplication => {
                        accumulator * expressions.next().context("Missing token")?.evaluate()?
                    }
                    _ => bail!("Invalid token!"),
                };
            } else {
                accumulator = exp.evaluate()?;
            }
        }

        Ok(accumulator)
    }

    fn apply_precedence(&self) -> Result<MathFormula> {
        let mut current_formula = self.expressions.iter();
        let mut expressions = vec![];

        let mut prev: Option<MathExpression> = Option::None;

        while let Some(exp) = current_formula.next() {
            // println!("Parse status({:?}): {:?}", prev, expressions);
            match exp {
                MathExpression::Operation(op) => match op {
                    MathToken::Addition => {
                        let new_parens = vec![
                            prev.context("Missing previous")?.apply_precedence()?,
                            exp.apply_precedence()?,
                            current_formula
                                .next()
                                .context("Missing expression")?
                                .apply_precedence()?,
                        ];
                        prev = Some(MathExpression::Parens(MathFormula {
                            expressions: new_parens,
                        }));
                    }
                    MathToken::Multiplication => {
                        expressions.push(prev.context("Missing previous")?);
                        expressions.push(exp.apply_precedence()?);
                        prev = Some(
                            current_formula
                                .next()
                                .context("Missing expression")?
                                .apply_precedence()?,
                        );
                    }
                    _ => bail!("Invalid token"),
                },
                _ => {
                    if let Some(prev_value) = prev {
                        expressions.push(prev_value);
                    }
                    prev = Some(exp.apply_precedence()?);
                }
            }
        }

        // println!("Parse status({:?}): {:?}", prev, expressions);
        if let Some(prev_value) = prev {
            expressions.push(prev_value);
        }
        Ok(MathFormula { expressions })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_smoke1() -> Result<()> {
        let expression =
            MathFormula::parse_tokens(&MathToken::parse_line("1 + 2 * 3 + 4 * 5 + 6")?)?;
        assert_eq!(71, expression.evaluate()?);
        let expression =
            MathFormula::parse_tokens(&MathToken::parse_line("1 + (2 * 3) + (4 * (5 + 6))")?)?;
        assert_eq!(51, expression.evaluate()?);
        let expression = MathFormula::parse_tokens(&MathToken::parse_line(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )?)?;
        assert_eq!(13632, expression.evaluate()?);
        Ok(())
    }

    #[test]
    fn day18_1() -> Result<()> {
        let mut accumulator = 0;

        for line in crate::read_file("day18.txt")? {
            let line = line?;
            let expression = MathFormula::parse_tokens(&MathToken::parse_line(&line)?)?;
            accumulator += expression.evaluate()?;
        }

        println!("Day 18.1: {}", accumulator);
        assert_eq!(12956356593940, accumulator);
        Ok(())
    }

    #[test]
    fn day18_smoke2() -> Result<()> {
        let expression =
            MathFormula::parse_tokens(&MathToken::parse_line("1 + 2 * 3 + 4 * 5 + 6")?)?
                .apply_precedence()?;
        println!("Day 18 smoke2: {}", expression);
        assert_eq!(231, expression.evaluate()?);
        let expression =
            MathFormula::parse_tokens(&MathToken::parse_line("1 + (2 * 3) + (4 * (5 + 6))")?)?
                .apply_precedence()?;
        println!("Day 18 smoke2: {}", expression);
        assert_eq!(51, expression.evaluate()?);
        let expression = MathFormula::parse_tokens(&MathToken::parse_line(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )?)?
        .apply_precedence()?;
        println!("Day 18 smoke2: {}", expression);
        assert_eq!(23340, expression.evaluate()?);

        Ok(())
    }

    #[test]
    fn day18_2() -> Result<()> {
        let mut accumulator = 0;

        for line in crate::read_file("day18.txt")? {
            let line = line?;
            let expression =
                MathFormula::parse_tokens(&MathToken::parse_line(&line)?)?.apply_precedence()?;
            accumulator += expression.evaluate()?;
        }

        println!("Day 18.2: {}", accumulator);
        assert_eq!(94240043727614, accumulator);
        Ok(())
    }
}
