use super::core::ExprPart;
use crate::error::CompilerError;
use zasm::types::Operator;

pub fn shutting_yard(toks: Vec<ExprPart>) -> Result<Vec<ExprPart>, CompilerError> {
    let mut output = vec![];
    let mut operator_stack: Vec<ExprPart> = vec![];

    for token in toks {
        match token {
            ExprPart::Operator(operator) => {
                while !operator_stack.is_empty() {
                    let top = operator_stack.last().unwrap();
                    match top {
                        ExprPart::Operator(o2) => {
                            if precedence(o2) < precedence(&operator) {
                                break;
                            }
                        }
                        ExprPart::Lpar => break,

                        _ => panic!(),
                    };

                    output.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(ExprPart::Operator(operator));
            }
            ExprPart::Operand(operand) => output.push(ExprPart::Operand(operand)),
            ExprPart::Lpar => operator_stack.push(ExprPart::Lpar),
            ExprPart::Rpar => {
                loop {
                    let top = match operator_stack.last() {
                        Some(top) => top,
                        None => {
                            return Err(CompilerError::new(0, 0, 1, "TODO: Mismatched parentheses"))
                        }
                    };

                    if top == &ExprPart::Lpar {
                        break;
                    }

                    output.push(operator_stack.pop().unwrap());
                }
                operator_stack.pop().unwrap();
            }
        }
    }

    operator_stack.reverse();
    output.extend(operator_stack);

    Ok(output)
}

fn precedence(op: &Operator) -> u32 {
    match op {
        Operator::Add => 2,
        Operator::Sub => 2,
        Operator::Mult => 3,
        Operator::Div => 3,
        Operator::DoubleEquals => 1,
        Operator::Mod => 3,
        Operator::Greater => 1,
        Operator::GreaterEquals => 1,
        Operator::Less => 1,
        Operator::LessEquals => 1,
        Operator::NotEquals => 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        ast::{Constant, Node, Primitive},
        core::ExprPart,
    };
    use zasm::types::Operator;

    use super::*;

    #[test]
    fn test_rpn() {
        let test_case = vec![
            ExprPart::Operand(Node::Constant(Constant {
                value: Primitive::Int(5),
            })),
            ExprPart::Operator(Operator::Add),
            ExprPart::Operand(Node::Constant(Constant {
                value: Primitive::Int(3),
            })),
            ExprPart::Operator(Operator::Mult),
            ExprPart::Operand(Node::Constant(Constant {
                value: Primitive::Int(4),
            })),
        ];

        let expected = vec![
            ExprPart::Operand(Node::Constant(Constant {
                value: Primitive::Int(5),
            })),
            ExprPart::Operand(Node::Constant(Constant {
                value: Primitive::Int(3),
            })),
            ExprPart::Operand(Node::Constant(Constant {
                value: Primitive::Int(4),
            })),
            ExprPart::Operator(Operator::Mult),
            ExprPart::Operator(Operator::Add),
        ];

        let got = shutting_yard(test_case).unwrap();
        assert_eq!(expected, got);
    }
}
