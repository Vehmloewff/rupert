// use crate::{
// 	binary_expression::{BinaryExpression, BinaryExpressionBuilder},
// 	number_literal::NumberLiteralBuilder,
// 	parser::{ConsumeAction, Parser, ParserContext},
// 	string_literal::StringLiteralBuilder,
// };

// pub enum Expression {
// 	BinaryExpression(BinaryExpression),
// 	NumberLiteral(f64),
// }

// pub enum ExpressionBuilder {
// 	Undecided,
// 	NumberLiteral(NumberLiteralBuilder),
// 	StringLiteral(StringLiteralBuilder),
// 	BinaryExpression(BinaryExpressionBuilder),
// }

// impl ExpressionBuilder {
// 	fn consume_considering<R: Sized, B: Parser<R>>(
// 		&mut self,
// 		builder: &mut B,
// 		character: char,
// 		context: &mut ParserContext,
// 	) -> ConsumeAction {
// 		let result = builder.consume(character, context);

// 		if let ConsumeAction::Accept = result {
// 			ConsumeAction::Accept
// 		} else if character.is_whitespace() {
// 			ConsumeAction::Accept
// 		} else if character == '+' || character == '-' || character == '*' || character == '/' {
// 			// *self = ExpressionBuilder::BinaryExpression(BinaryExpressionBuilder::new(
// 			// 	builder.reduce(context),
// 			// 	character,
// 			// ));

// 			ConsumeAction::Accept
// 		} else {
// 			ConsumeAction::Reject
// 		}
// 	}
// }

// impl Parser<Expression> for ExpressionBuilder {
// 	fn consume(&mut self, character: char, context: &mut ParserContext) -> ConsumeAction {
// 		match self {
// 			Self::Undecided => {
// 				if character.is_whitespace() {
// 					ConsumeAction::Accept
// 				} else if character == '-' {
// 					*self = ExpressionBuilder::NumberLiteral(NumberLiteralBuilder::new(
// 						true, None, context,
// 					));

// 					ConsumeAction::Accept
// 				} else if character.is_numeric() {
// 					*self = ExpressionBuilder::NumberLiteral(NumberLiteralBuilder::new(
// 						false,
// 						Some(character),
// 						context,
// 					));

// 					ConsumeAction::Accept
// 				} else if character == '"' {
// 					*self = ExpressionBuilder::StringLiteral(StringLiteralBuilder::new());

// 					ConsumeAction::Accept
// 				} else {
// 					ConsumeAction::Reject
// 				}
// 			}
// 			Self::NumberLiteral(builder) => builder.consume(character, context),
// 			Self::StringLiteral(builder) => builder.consume(character, context),
// 			_ => ConsumeAction::Reject,
// 		}
// 	}

// 	fn reduce(self, _: &mut ParserContext) -> Expression {
// 		Expression::NumberLiteral(40.0)
// 	}
// }
