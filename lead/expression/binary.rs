use super::{parse_expression, Expression};
use rupert_parser::{wrap, InputStream, ParseResult, WrapResult};

#[derive(Debug)]
pub struct AdditiveExpression {
	left: Box<Expression>,
	operator: char,
	right: Box<Expression>,
}

#[derive(Debug)]
pub struct MultiplicativeExpression {
	left: Box<Expression>,
	operator: char,
	right: Box<Expression>,
}

pub fn wrap_binary_expression(stream: InputStream, left: Expression) -> WrapResult<Expression> {
	wrap!(stream, left; wrap_multiplicative_expression, wrap_additive_expression)
}

fn wrap_multiplicative_expression(mut stream: InputStream, left: Expression) -> WrapResult<Expression> {
	let operator = match stream.consume_any_char(&['*', '/']) {
		Some(character) => character,
		None => return WrapResult::Reject(stream, left),
	};

	stream.consume_whitespace();

	let (stream, right) = match parse_expression(stream) {
		ParseResult::Built(stream, expression) => (stream, expression),
		ParseResult::Reject(stream) => (stream, Expression::Never),
	};

	WrapResult::Built(
		stream,
		Expression::MultiplicativeExpression(MultiplicativeExpression {
			left: Box::new(left),
			operator,
			right: Box::new(right),
		}),
	)
}

fn wrap_additive_expression(mut stream: InputStream, left: Expression) -> WrapResult<Expression> {
	let operator = match stream.consume_any_char(&['+', '-']) {
		Some(character) => character,
		None => return WrapResult::Reject(stream, left),
	};

	stream.consume_whitespace();

	let (stream, right) = match parse_expression(stream) {
		ParseResult::Built(stream, expression) => (stream, expression),
		ParseResult::Reject(stream) => (stream, Expression::Never),
	};

	WrapResult::Built(
		stream,
		Expression::AdditiveExpression(AdditiveExpression {
			left: Box::new(left),
			operator,
			right: Box::new(right),
		}),
	)
}
