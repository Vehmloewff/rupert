use rupert_parser::{any, wrap_recursive, InputStream, ParseResult, WrapResult};

fn nerf_whitespace(mut stream: InputStream, inner: Expression) -> WrapResult<Expression> {
	stream.consume_whitespace();

	WrapResult::Reject(stream, inner)
}

fn wrap_additive_expression(mut stream: InputStream, left: Expression) -> WrapResult<Expression> {
	let operator = match stream.consume_any_char(&['+', '-']) {
		Some(character) => character,
		None => return WrapResult::Reject(stream, left),
	};

	stream.consume_whitespace();

	let (stream, right) = match parse_expression(stream) {
		ParseResult::Built(stream, expression) => (stream, expression),
		ParseResult::Reject(stream) => (stream, Expression::NullLiteral),
	};

	WrapResult::Built(
		stream,
		Expression::AdditiveExpression {
			left: Box::new(left),
			operator,
			right: Box::new(right),
		},
	)
}

fn wrap_multiplicative_expression(
	mut stream: InputStream,
	left: Expression,
) -> WrapResult<Expression> {
	let operator = match stream.consume_any_char(&['*', '/']) {
		Some(character) => character,
		None => return WrapResult::Reject(stream, left),
	};

	stream.consume_whitespace();

	let (stream, right) = match parse_expression(stream) {
		ParseResult::Built(stream, expression) => (stream, expression),
		ParseResult::Reject(stream) => (stream, Expression::NullLiteral),
	};

	WrapResult::Built(
		stream,
		Expression::MultiplicativeExpression {
			left: Box::new(left),
			operator,
			right: Box::new(right),
		},
	)
}

fn parse_number_literal(mut stream: InputStream) -> ParseResult<Expression> {
	let mut start = String::new();
	let mut has_decimal = false;

	loop {
		if let Some(digits) = stream.consume_digits() {
			start.push_str(digits.as_str())
		} else if !has_decimal && stream.consume_char('.').is_some() {
			has_decimal = true;
			start.push('.')
		} else {
			break;
		}
	}

	if start.is_empty() {
		ParseResult::Reject(stream)
	} else {
		let num = start.parse::<f64>().unwrap_or(0.0);

		ParseResult::Built(stream, Expression::NumberLiteral(num))
	}
}

#[derive(Debug)]
pub enum Expression {
	NumberLiteral(f64),
	AdditiveExpression {
		left: Box<Expression>,
		operator: char,
		right: Box<Expression>,
	},
	MultiplicativeExpression {
		left: Box<Expression>,
		operator: char,
		right: Box<Expression>,
	},
	NullLiteral,
}

pub fn parse_expression(stream: InputStream) -> ParseResult<Expression> {
	let result = any!(stream; parse_number_literal);

	match result {
		ParseResult::Built(stream, expression) => {
			let result = wrap_recursive!(stream, expression; nerf_whitespace, wrap_additive_expression, wrap_multiplicative_expression);

			match result {
				WrapResult::Built(stream, full_expression) => {
					ParseResult::Built(stream, full_expression)
				}
				WrapResult::Reject(stream, expression) => ParseResult::Built(stream, expression),
			}
		}
		ParseResult::Reject(stream) => ParseResult::Reject(stream),
	}

	// if let ParseResult::Built(stream, expression) = result {
	// return ParseResult::Reject(stream);
	// } else {
	// result
	// }

	// if let ParseResult::Built() = result {}

	// result
}
