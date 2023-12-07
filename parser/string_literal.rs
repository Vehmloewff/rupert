// use crate::parser::{ConsumeAction, Parser, ParserContext};

// pub struct StringLiteralBuilder {
// 	characters: Vec<char>,
// }

// impl StringLiteralBuilder {
// 	pub fn new() -> StringLiteralBuilder {
// 		StringLiteralBuilder {
// 			characters: Vec::new(),
// 		}
// 	}
// }

// impl Parser<String> for StringLiteralBuilder {
// 	fn consume(&mut self, character: char, context: &mut ParserContext) -> ConsumeAction {
// 		self.characters.push(character);

// 		if character == '"' {
// 			context.is_completed();
// 		}

// 		if context.is_completed() {
// 			ConsumeAction::Reject
// 		} else {
// 			ConsumeAction::Accept
// 		}
// 	}

// 	fn reduce(self, _: &mut ParserContext) -> String {
// 		self.characters.iter().collect::<String>()
// 	}
// }
