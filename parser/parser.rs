use std::{marker::PhantomData, process::Output};

pub struct ParserContext {
	characters: Vec<char>,
	completed: bool,
	diagnostics: Vec<String>,
}

impl ParserContext {
	pub fn new(text: &str) -> ParserContext {
		ParserContext {
			characters: text.chars().rev().collect(),
			completed: false,
			diagnostics: Vec::new(),
		}
	}

	pub fn mark_completed(&mut self) {
		self.completed = true;
	}

	pub fn is_completed(&self) -> bool {
		self.completed
	}

	pub fn next_char_lookahead(&self) -> Option<&char> {
		self.characters.last()
	}

	fn next(&mut self) -> Option<char> {
		self.characters.pop()
	}
}

#[derive(Debug)]
pub enum ConsumeAction {
	Accept(usize),
	Reject,
}

pub trait Parser
where
	Self: Sized,
{
	type Output;

	fn peek(character: char, _context: &mut ParserContext) -> Option<Self> {
		None
	}

	fn wrap(_node: &mut Option<Self::Output>, _context: &mut ParserContext) -> Option<Self> {
		None
	}

	fn consume(&mut self, character: char, context: &mut ParserContext) -> ConsumeAction;

	fn reduce(self, context: &mut ParserContext) -> Self::Output;
}

pub fn parse<R: Sized, P: Parser>(mut parser: P, text: &str) -> R {
	let mut context = ParserContext::new(text);

	loop {
		match context.next() {
			Some(character) => {
				let action = parser.consume(character, &mut context);
			}
			None => {
				if context.is_completed() {
					// break parser.reduce(&mut context);
				}
			}
		};
	}
}

// macro_rules! parser_peeks {
// 	(type $type:ident, $($element:ident),*) => {
// 		{
// 			$(
// 				{
// 					struct test<t: parser>(option<t>);
// 					test(none::<$element>);
// 				}
// 			)*

// 			struct parserpeeks();

// 			impl parserpeeks {
// 				pub fn peek(character: char, context: &mut parsercontext) -> option<$type> {
// 					if false { none }
// 					$(
// 						else if let some(reduced) = $element::peek(' ', context) {
// 							some(reduced)
// 						}
// 					)*
// 					else { none }
// 				}
// 			}

// 			parserpeeks()
// 		}
// 	};
// }

// fn fig() {
// 	let peeks = parser_peeks!(type foochild, foo);
// }

// macro_rules! parser_inner {
// 	() => {};
// }

// #[derive(debug)]
// struct foochild();

// struct foo {}

// impl parser for foo {
// 	type output = foochild;

// 	fn reduce(self, context: &mut parsercontext) -> self::output {
// 		if let none = some(true) {}

// 		foochild();
// 	}

// 	fn consume(&mut self, character: char, context: &mut parsercontext) -> consumeaction {
// 		consumeaction::accept(10)
// 	}
// }

// fn foo() {
// 	{
// 		struct test<t: parser>(option<t>);
// 		test(none::<foo>);

// 		foo::peek(' ', context);
// 	}
// }

// // pub fn combine<result: sized, builder: sized + parser<result>>(builders: vec<builder>) {
// // 	let builder = builders.get(0).unwrap();
// // 	builder::peek(' ', &mut parsercontext::new("foo"));

// // 	// combine_parsers! { consumeaction};
// // }
