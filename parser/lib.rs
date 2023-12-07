mod binary_expression;
mod expression;
mod number_literal;
mod parser;
mod string_literal;

#[derive(Debug)]
pub struct Span {
	start: usize,
	end: usize,
}

#[derive(Debug)]
pub struct Diagnostic {
	message: String,
	span: Span,
}

#[derive(Debug)]
pub struct InputStream {
	reverse_chars: Vec<char>,
	diagnostics: Vec<Diagnostic>,
}

impl InputStream {
	pub fn new(text: &str) -> InputStream {
		InputStream {
			reverse_chars: text.chars().rev().collect::<Vec<char>>(),
			diagnostics: Vec::new(),
		}
	}

	pub fn lookahead(&self, length: usize) -> String {
		let chars_length = self.reverse_chars.len();

		self.reverse_chars[chars_length - length..chars_length]
			.iter()
			.rev()
			.collect::<String>()
	}

	pub fn consume(&mut self, count: usize) -> String {
		let mut string = String::with_capacity(count);

		for _ in 0..count {
			let character = match self.reverse_chars.pop() {
				Some(character) => character,
				None => break,
			};

			string.push(character)
		}

		string
	}

	pub fn consume_char(&mut self, expected_char: char) -> Option<char> {
		self.consume_any_char(&[expected_char])
	}

	pub fn consume_any_char(&mut self, expected_chars: &[char]) -> Option<char> {
		match self.reverse_chars.last() {
			Some(character) => {
				if expected_chars.contains(character) {
					Some(self.reverse_chars.pop().unwrap())
				} else {
					None
				}
			}
			None => None,
		}
	}

	pub fn consume_single_whitespace(&mut self) -> Option<char> {
		match self.reverse_chars.last() {
			Some(character) => {
				if character.is_whitespace() {
					Some(self.reverse_chars.pop().unwrap())
				} else {
					None
				}
			}
			None => None,
		}
	}

	pub fn consume_single_digit(&mut self) -> Option<char> {
		match self.reverse_chars.last() {
			Some(character) => {
				if character.is_ascii_digit() {
					Some(self.reverse_chars.pop().unwrap())
				} else {
					None
				}
			}
			None => None,
		}
	}

	pub fn consume_text(&mut self, text: &str) -> Option<String> {
		let compare = self.lookahead(text.len());

		if &compare == text {
			Some(self.consume(text.len()))
		} else {
			None
		}
	}

	pub fn consume_whitespace(&mut self) -> Option<String> {
		let mut string = match self.consume_single_whitespace() {
			Some(character) => String::from(character),
			None => return None,
		};

		loop {
			string.push(match self.consume_single_whitespace() {
				Some(character) => character,
				None => break,
			})
		}

		Some(string)
	}

	pub fn consume_digits(&mut self) -> Option<String> {
		let mut string = match self.consume_single_digit() {
			Some(character) => String::from(character),
			None => return None,
		};

		loop {
			string.push(match self.consume_single_digit() {
				Some(character) => character,
				None => break,
			})
		}

		Some(string)
	}
}

macro_rules! any {
    ($stream:ident; $($func:ident),+) => {
        'block: {
            let stream = $stream;

            $(
                let stream = match $func(stream) {
                    ParseResult::Reject(stream) => stream,
                    ParseResult::Built(stream, node) => break 'block ParseResult::Built(stream, node),
                };
            )*

            ParseResult::Reject(stream)
        }
    };
}

macro_rules! wrap {
    ($stream:ident, $node:ident; $($func:ident),+) => {
        'block: {
            let stream = $stream;
            let node = $node;

            $(
                let (stream, node) = match $func(stream, node) {
                    WrapResult::Reject(stream, node) => (stream, node),
                    WrapResult::Built(stream, node) => break 'block WrapResult::Built(stream, node),
                };
            )*

            WrapResult::Reject(stream, node)
        }
    };
}

macro_rules! wrap_recursive {
	($stream:ident, $node:ident; $($func:ident),+) => {
        {
            let mut result = wrap!($stream, $node; $($func),+);

			loop {
				if let WrapResult::Built(stream, node) = result {
					result = wrap!(stream, node; $($func),+);
				}

				break result;
			}
        }
    };
}

pub enum WrapResult<T> {
	Reject(InputStream, T),
	Built(InputStream, T),
}

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

#[derive(Debug)]
pub enum ParseResult<T> {
	Reject(InputStream),
	Built(InputStream, T),
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
