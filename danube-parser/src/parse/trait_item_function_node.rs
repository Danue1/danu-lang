use super::*;

pub(super) fn parse_trait_item_function_node(s: Tokens) -> ParseResult<TraitItemFunctionNode> {
  map(
    tuple((
      opt(parse_keyword(Keyword::Async)),
      parse_keyword(Keyword::Function),
      parse_ident_node,
      opt(parse_generic_node),
      parse_function_argument_list,
      opt(parse_function_type),
      parse_function_body,
    )),
    |(is_async, _, ident, generic, (self_type, argument_list), return_type, block)| {
      TraitItemFunctionNode {
        is_async: is_async.is_some(),
        ident,
        generic,
        self_type,
        argument_list,
        return_type,
        block,
      }
    },
  )(s)
}

fn parse_function_argument_list(
  s: Tokens,
) -> ParseResult<(Option<ImmutablityKind>, Vec<FunctionArgumentNode>)> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      opt(map(
        tuple((
          parse_immutablity_kind,
          parse_keyword(Keyword::VariableSelf),
          opt(parse_symbol(Symbol::Comma)),
        )),
        |(self_type, _, _)| self_type,
      )),
      separated_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, self_type, argument_list, _, _)| (self_type, argument_list),
  )(s)
}

fn parse_function_type(s: Tokens) -> ParseResult<TypeKind> {
  map(
    tuple((parse_symbol(Symbol::ReturnArrow), parse_type_kind)),
    |(_, ty)| ty,
  )(s)
}

fn parse_function_body(s: Tokens) -> ParseResult<Option<BlockNode>> {
  alt((
    map(parse_function_body_shrotcut, Some),
    map(parse_block_node, Some),
    map(parse_symbol(Symbol::Semicolon), |_| None),
  ))(s)
}

fn parse_function_body_shrotcut(s: Tokens) -> ParseResult<BlockNode> {
  map(
    tuple((
      parse_symbol(Symbol::Assign),
      parse_expression_kind,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, expression, _)| BlockNode {
      statement_list: vec![StatementKind::ExpressionKind(expression)],
    },
  )(s)
}
