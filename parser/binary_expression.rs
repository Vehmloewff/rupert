// use crate::{
// 	expression::Expression,
// 	parser::{ConsumeAction, Parser, ParserContext},
// };

// pub enum BinaryOperator {
// 	Addition,
// }

// pub struct BinaryExpression {
// 	right: Box<Expression>,
// 	left: Box<Expression>,
// 	operator: BinaryOperator,
// }

// pub struct BinaryExpressionBuilder {
// 	left: Expression,
// 	right: Option<Expression>,
// 	operator: Vec<char>,
// }

// impl BinaryExpressionBuilder {
// 	pub fn new(left: Expression, operator_start: char) -> BinaryExpressionBuilder {
// 		BinaryExpressionBuilder {
// 			left,
// 			right: None,
// 			operator: vec![operator_start],
// 		}
// 	}
// }

// impl Parser<BinaryExpression> for BinaryExpressionBuilder {
// 	fn consume(&mut self, character: char, context: &mut ParserContext) -> ConsumeAction {
// 		ConsumeAction::Accept
// 	}

// 	fn reduce(self, _: &mut ParserContext) -> BinaryExpression {
// 		BinaryExpression {
// 			left: Box::new(self.left),
// 			right: Box::new(self.right.unwrap()),
// 			operator: BinaryOperator::Addition,
// 		}
// 	}
// }
