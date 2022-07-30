pub struct Re0Parser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    declare_statement,
    declare_block,
    declare_item,
    declare_pair,
    Key,
    COLON,
    if_statement,
    else_statement,
    kw_if,
    kw_else,
    expression,
    term,
    function,
    op_infix,
    GT,
    LT,
    EQ,
    NE,
    GEQ,
    LEQ,
    ADD_ASSIGN,
    SUB_ASSIGN,
    list_block,
    block,
    Special,
    Number,
    Decimal,
    Integer,
    Sign,
    String,
    StringNormal,
    NS,
    SYMBOL,
    modifiers,
    Dot,
    COMMENT,
    WHITESPACE,
    LineComment,
    OmitComment,
    MultiLineComment,
    SEPARATOR,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for Re0Parser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::declare_statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::declare_statement(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::statement, |state| self::if_statement(state).or_else(|state| self::declare_block(state)).or_else(|state| self::expression(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn declare_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::declare_statement, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::modifiers(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::declare_block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn declare_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::declare_block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::declare_item(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::declare_item(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn declare_item(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::SEPARATOR(state).or_else(|state| self::declare_pair(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn declare_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::declare_pair, |state| state.sequence(|state| self::Key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::COLON(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::declare_block(state))).or_else(|state| state.sequence(|state| self::Key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::COLON(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::list_block(state))).or_else(|state| state.sequence(|state| self::COLON(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Key, |state| self::StringNormal(state).or_else(|state| self::SYMBOL(state)).or_else(|state| self::Integer(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COLON(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(":").or_else(|state| state.match_string("："))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::if_statement, |state| state.sequence(|state| self::kw_if(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state).or_else(|state| state.sequence(|state| self::block(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::else_statement(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn else_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::else_statement, |state| state.sequence(|state| state.match_string("else").and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn kw_if(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::kw_if, |state| state.match_string("若").or_else(|state| state.match_string("如果")).or_else(|state| state.match_string("if")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn kw_else(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::kw_else, |state| state.match_string("否则").or_else(|state| state.match_string("else")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::op_infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::op_infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::term, |state| self::function(state).or_else(|state| self::Special(state)).or_else(|state| self::Number(state)).or_else(|state| self::String(state)).or_else(|state| self::SYMBOL(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.sequence(|state| self::expression(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SEPARATOR(state)))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| state.sequence(|state| state.match_string("（").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.sequence(|state| self::expression(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::SEPARATOR(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expression(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SEPARATOR(state)))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("）")))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn op_infix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::GT(state).or_else(|state| self::LT(state)).or_else(|state| self::GEQ(state)).or_else(|state| self::LEQ(state)).or_else(|state| self::EQ(state)).or_else(|state| self::NE(state)).or_else(|state| self::ADD_ASSIGN(state)).or_else(|state| self::SUB_ASSIGN(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::GT, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">").or_else(|state| state.match_string("》")).or_else(|state| state.match_string("大于"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LT, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<").or_else(|state| state.match_string("《")).or_else(|state| state.match_string("小于"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EQ(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EQ, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("==").or_else(|state| state.match_string("等于")).or_else(|state| state.match_string("为")).or_else(|state| state.match_string("是"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NE, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!=").or_else(|state| state.match_string("不等于")).or_else(|state| state.match_string("非")).or_else(|state| state.match_string("不是"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GEQ(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::GEQ, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">=").or_else(|state| state.match_string("》=")).or_else(|state| state.match_string("大于等于")).or_else(|state| state.match_string("不小于"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LEQ(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LEQ, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<=").or_else(|state| state.match_string("《=")).or_else(|state| state.match_string("小于等于")).or_else(|state| state.match_string("不大于"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ADD_ASSIGN(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ADD_ASSIGN, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SUB_ASSIGN(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SUB_ASSIGN, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("-=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list_block, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).or_else(|state| self::SEPARATOR(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state).or_else(|state| self::SEPARATOR(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))).or_else(|state| state.sequence(|state| state.match_string("【").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).or_else(|state| self::SEPARATOR(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state).or_else(|state| self::SEPARATOR(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("】")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).or_else(|state| self::SEPARATOR(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state).or_else(|state| self::SEPARATOR(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Special(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Special, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null")).or_else(|state| state.match_string("真")).or_else(|state| state.match_string("假")).or_else(|state| state.match_string("空"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| self::Integer(state).or_else(|state| self::Decimal(state)).and_then(|state| state.optional(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0").or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+").or_else(|state| state.match_string("-"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::String, |state| self::StringNormal(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringNormal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::StringNormal, |state| state.sequence(|state| state.match_string("'").and_then(|state| state.repeat(|state| self::NS(state))).and_then(|state| state.match_string("'"))).or_else(|state| state.sequence(|state| state.match_string("\"").and_then(|state| state.repeat(|state| self::NS(state))).and_then(|state| state.match_string("\"")))).or_else(|state| state.sequence(|state| state.match_string("“").and_then(|state| state.repeat(|state| self::NS(state))).and_then(|state| state.match_string("”")))).or_else(|state| state.sequence(|state| state.match_string("‹").and_then(|state| state.repeat(|state| self::NS(state))).and_then(|state| state.match_string("›")))).or_else(|state| state.sequence(|state| state.match_string("«").and_then(|state| state.repeat(|state| self::NS(state))).and_then(|state| state.match_string("»"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NS(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NS, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("\\u").and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state))).or_else(|state| state.sequence(|state| state.match_string("\\u").and_then(|state| state.match_string("{")).and_then(|state| state.sequence(|state| self::ASCII_HEX_DIGIT(state).or_else(|state| self::SPACE_SEPARATOR(state)).and_then(|state| state.repeat(|state| self::ASCII_HEX_DIGIT(state).or_else(|state| self::SPACE_SEPARATOR(state)))))).and_then(|state| state.match_string("}")))).or_else(|state| state.sequence(|state| state.match_string("\\").and_then(|state| self::ANY(state)))).or_else(|state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("'").or_else(|state| state.match_string("\"")).or_else(|state| state.match_string("\\"))).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("'").or_else(|state| state.match_string("\"")).or_else(|state| state.match_string("\\"))).and_then(|state| self::ANY(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).or_else(|state| state.match_string("_")).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state).or_else(|state| state.match_string("_"))))).or_else(|state| state.sequence(|state| state.match_string("`").and_then(|state| state.repeat(|state| state.sequence(|state| state.match_string("\\").and_then(|state| self::ANY(state))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("\\").or_else(|state| state.match_string("`"))).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("`"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn modifiers(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::modifiers, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SYMBOL(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SYMBOL(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::LineComment(state).or_else(|state| self::OmitComment(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::SPACE_SEPARATOR(state)).or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineComment, |state| state.sequence(|state| state.match_string("///").or_else(|state| state.match_string("、")).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn OmitComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("//").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::MultiLineComment, |state| state.sequence(|state| state.match_string("/*").and_then(|state| state.repeat(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("*/")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string(",").or_else(|state| state.match_string(";")).or_else(|state| state.match_string("，")).or_else(|state| state.match_string("；"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::declare_statement => rules::declare_statement(state),
            Rule::declare_block => rules::declare_block(state),
            Rule::declare_item => rules::declare_item(state),
            Rule::declare_pair => rules::declare_pair(state),
            Rule::Key => rules::Key(state),
            Rule::COLON => rules::COLON(state),
            Rule::if_statement => rules::if_statement(state),
            Rule::else_statement => rules::else_statement(state),
            Rule::kw_if => rules::kw_if(state),
            Rule::kw_else => rules::kw_else(state),
            Rule::expression => rules::expression(state),
            Rule::term => rules::term(state),
            Rule::function => rules::function(state),
            Rule::op_infix => rules::op_infix(state),
            Rule::GT => rules::GT(state),
            Rule::LT => rules::LT(state),
            Rule::EQ => rules::EQ(state),
            Rule::NE => rules::NE(state),
            Rule::GEQ => rules::GEQ(state),
            Rule::LEQ => rules::LEQ(state),
            Rule::ADD_ASSIGN => rules::ADD_ASSIGN(state),
            Rule::SUB_ASSIGN => rules::SUB_ASSIGN(state),
            Rule::list_block => rules::list_block(state),
            Rule::block => rules::block(state),
            Rule::Special => rules::Special(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::Integer => rules::Integer(state),
            Rule::Sign => rules::Sign(state),
            Rule::String => rules::String(state),
            Rule::StringNormal => rules::StringNormal(state),
            Rule::NS => rules::NS(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::modifiers => rules::modifiers(state),
            Rule::Dot => rules::Dot(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::LineComment => rules::LineComment(state),
            Rule::OmitComment => rules::OmitComment(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::SEPARATOR => rules::SEPARATOR(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
