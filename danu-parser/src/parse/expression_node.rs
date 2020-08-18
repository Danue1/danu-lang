use super::*;

pub(super) fn parse_expression_node(s: Tokens) -> ParseResult<ExpressionNode> {
  alt((
    map(
      parse_expression_conditional_node,
      ExpressionNode::Conditional,
    ),
    map(parse_loop_node, ExpressionNode::Loop),
    map(parse_while_node, ExpressionNode::While),
    map(parse_pattern_match_node, ExpressionNode::PatternMatch),
    map(parse_literal_value_node, ExpressionNode::Literal),
    map(parse_break, |_| ExpressionNode::Break),
    map(parse_continue, |_| ExpressionNode::Continue),
    map(parse_return_node, ExpressionNode::Return),
    map(parse_array, ExpressionNode::Array),
  ))(s)
}

fn parse_break(s: Tokens) -> ParseResult<()> {
  map(parse_keyword(Keyword::Break), |_| ())(s)
}

fn parse_continue(s: Tokens) -> ParseResult<()> {
  map(parse_keyword(Keyword::Continue), |_| ())(s)
}

fn parse_array(s: Tokens) -> ParseResult<Vec<ExpressionNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBracket),
      separated_list(parse_symbol(Symbol::Comma), parse_expression_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, expression_list, _, _)| expression_list,
  )(s)
}
