mod binary;
mod number_literal;
mod string_literal;

use crate::whitespace::nerf_whitespace;

use self::{
	binary::{wrap_binary_expression, BinaryExpression},
	number_literal::{parse_number_literal, NumberLiteral},
	string_literal::{parse_string_literal, StringLiteral},
};
use rupert::{any, wrap_recursive, InputStream, ParseResult, WrapResult};

#[derive(Debug, Clone)]
pub enum Expression {
	NumberLiteral(NumberLiteral),
	StringLiteral(StringLiteral),
	BinaryExpression(BinaryExpression),
	Never,
}

pub fn parse_expression(stream: InputStream) -> ParseResult<Expression> {
	let result = any!(stream; parse_number_literal, parse_string_literal);

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
