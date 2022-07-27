use std::lazy::SyncLazy;

use re0_pest::{Assoc, Operator, Pair, Pairs, PrecClimber, Rule};

use crate::{
    ast::{parser::ParseContext, ASTNode},
    Result,
};

static OPERATORS: SyncLazy<PrecClimber<Rule>> = SyncLazy::new(|| {
    use Assoc::*;
    use Rule::*;
    macro_rules! left {
        ($i:ident, $($j:ident),*) => {
            Operator::new($i, Left) $(|Operator::new($j, Left))*
        };
    }
    macro_rules!  right {
        ($i:ident, $($j:ident),*) => {
            Operator::new($i, Right) $(|Operator::new($j, Right))*
        };
    }
    /// 越往下, 优先级越高
    PrecClimber::new(vec![left!(ADD_ASSIGN, SUB_ASSIGN), left!(GT, LT, GEQ, LEQ)])
});

impl ParseContext {
    pub(super) fn expression(&mut self, pairs: Pair<Rule>) -> ASTNode {
        let out = OPERATORS.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::term => self.term(pair),
                _ => unreachable!("{:?}", pair.as_rule()),
            },
            |lhs: ASTNode, op: Pair<Rule>, rhs: ASTNode| match op.as_rule() {
                _ => unreachable!("{:?}", op.as_rule()),
            },
        );
        return out;
    }
    fn term(&mut self, pairs: Pair<Rule>) -> ASTNode {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::data => {
                    match self.data(pair) {
                        Ok(o) => {return o}
                        Err(e) => {self.errors.push(e);}
                    }


                },
                _ => unreachable!("{:?}", pair.as_rule()),
            }
        }
        todo!()
    }
}
