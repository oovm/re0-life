use std::lazy::SyncLazy;

use pest::iterators::Pair;
use pest::prec_climber::Operator;
use pest::prec_climber::{Assoc, PrecClimber};

use crate::{
    ast::{parser::ParseContext, ASTNode},
    Rule,
};

static OPERATORS: SyncLazy<PrecClimber<Rule>> = SyncLazy::new(|| {
    use Assoc::*;
    use Rule::*;
    macro_rules! left {
        ($i:ident, $($j:ident),*) => {
            Operator::new($i, Left) $(|Operator::new($j, Left))*
        };
    }
    macro_rules! right {
        ($i:ident, $($j:ident),*) => {
            Operator::new($i, Right) $(|Operator::new($j, Right))*
        };
    }
    // 越往下, 优先级越高
    PrecClimber::new(vec![left!(ADD_ASSIGN, SUB_ASSIGN), left!(GT, LT, GEQ, LEQ)])
});

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParseContext {
    pub(super) fn expression(&mut self, pairs: Pair<Rule>) -> ASTNode {
        let out = OPERATORS.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::term => self.term(pair),
                _ => debug_cases!(pair),
            },
            |lhs: ASTNode, op: Pair<Rule>, rhs: ASTNode| match op.as_rule() {
                Rule::GT => ASTNode::binary_expression(lhs, rhs, ">"),
                Rule::LT => ASTNode::binary_expression(lhs, rhs, "<"),
                Rule::GEQ => ASTNode::binary_expression(lhs, rhs, ">="),
                Rule::LEQ => ASTNode::binary_expression(lhs, rhs, "<="),
                Rule::ADD_ASSIGN => ASTNode::binary_expression(lhs, rhs, "+="),
                Rule::SUB_ASSIGN => ASTNode::binary_expression(lhs, rhs, "-="),
                _ => debug_cases!(op),
            },
        );
        return out;
    }
    fn term(&mut self, pairs: Pair<Rule>) -> ASTNode {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::data => match self.data(pair) {
                    Ok(o) => return o,
                    Err(e) => {
                        self.errors.push(e);
                    }
                },
                _ => unreachable!("{:?}", pair.as_rule()),
            }
        }
        todo!()
    }
}
