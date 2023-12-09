mod binary;
mod number_literal;

use crate::whitespace::nerf_whitespace;

use self::{
	binary::{wrap_binary_expression, BinaryExpression},
	number_literal::{parse_number_literal, NumberLiteral},
};
use rupert_parser::{any, wrap_recursive, InputStream, ParseResult, WrapResult};

#[derive(Debug)]
pub enum Expression {
	NumberLiteral(NumberLiteral),
	BinaryExpression(BinaryExpression),
	Never,
}

pub fn parse_expression(stream: InputStream) -> ParseResult<Expression> {
	let result = any!(stream; parse_number_literal);

	match result {
		ParseResult::Built(stream, expression) => {
			let result = wrap_recursive!(stream, expression; nerf_whitespace, wrap_binary_expression);

			match result {
				WrapResult::Built(stream, full_expression) => ParseResult::Built(stream, full_expression),
				WrapResult::Reject(stream, expression) => ParseResult::Built(stream, expression),
			}
		}
		ParseResult::Reject(stream) => ParseResult::Reject(stream),
	}
}
