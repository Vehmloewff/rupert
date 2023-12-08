#[derive(Debug)]
pub enum WrapResult<T> {
	Reject(InputStream, T),
	Built(InputStream, T),
}

#[derive(Debug)]
pub enum ParseResult<T> {
	Reject(InputStream),
	Built(InputStream, T),
}

#[derive(Debug)]
pub struct Span {
	pub start: usize,
	pub end: usize,
}

impl Span {
	pub fn new<U: Into<usize>>(start: U, end: U) -> Span {
		Span {
			start: start.into(),
			end: end.into(),
		}
	}
}

#[derive(Debug)]
pub struct Diagnostic {
	pub message: String,
	pub span: Span,
}

impl Diagnostic {
	pub fn new<S: Into<String>>(message: S, span: Span) -> Diagnostic {
		Diagnostic {
			message: message.into(),
			span,
		}
	}
}

#[derive(Debug)]
pub struct InputStream {
	index: usize,
	reverse_chars: Vec<char>,
	diagnostics: Vec<Diagnostic>,
}

impl InputStream {
	pub fn new(text: &str) -> InputStream {
		InputStream {
			index: 0,
			reverse_chars: text.chars().rev().collect::<Vec<char>>(),
			diagnostics: Vec::new(),
		}
	}

	pub fn get_index(&self) -> usize {
		self.index
	}

	pub fn finish(mut self) -> (Vec<char>, Vec<Diagnostic>) {
		self.reverse_chars.reverse();

		(self.reverse_chars, self.diagnostics)
	}

	pub fn lookahead(&self, length: usize) -> String {
		let chars_length = self.reverse_chars.len();

		self.reverse_chars[chars_length - length..chars_length]
			.iter()
			.rev()
			.collect::<String>()
	}

	fn pop(&mut self) -> Option<char> {
		self.index += 1;
		self.reverse_chars.pop()
	}

	pub fn consume(&mut self, count: usize) -> String {
		let mut string = String::with_capacity(count);

		for _ in 0..count {
			let character = match self.pop() {
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
					Some(self.pop().unwrap())
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
					Some(self.pop().unwrap())
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
					Some(self.pop().unwrap())
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
