use super::Expression;
use rupert_parser::{InputStream, ParseResult, Span};

#[derive(Debug)]
pub struct NumberLiteral {
	span: Span,
	number: f64,
}

pub fn parse_number_literal(mut stream: InputStream) -> ParseResult<Expression> {
	let index = stream.get_index();
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
		let number = start.parse::<f64>().unwrap_or(0.0);
		let span = Span::new(index, stream.get_index());
		let node = NumberLiteral { span, number };

		ParseResult::Built(stream, Expression::NumberLiteral(node))
	}
}
