// use crate::parser::{ConsumeAction, Parser, ParserContext};

// pub struct NumberLiteralBuilder {
// 	is_negative: bool,
// 	has_decimal: bool,
// 	characters: Vec<char>,
// }

// impl NumberLiteralBuilder {
// 	pub fn new(
// 		with_negative: bool,
// 		start_number: Option<char>,
// 		context: &mut ParserContext,
// 	) -> NumberLiteralBuilder {
// 		let mut characters = Vec::new();

// 		match start_number {
// 			Some(character) => characters.push(character),
// 			None => (),
// 		}

// 		if !characters.is_empty() {
// 			context.mark_completed()
// 		}

// 		NumberLiteralBuilder {
// 			is_negative: with_negative,
// 			has_decimal: false,
// 			characters,
// 		}
// 	}
// }

// impl Parser<f64> for NumberLiteralBuilder {
// 	fn consume(&mut self, character: char, context: &mut ParserContext) -> ConsumeAction {
// 		if character.is_whitespace() {
// 			if self.characters.is_empty() {
// 				ConsumeAction::Accept
// 			} else {
// 				ConsumeAction::Reject
// 			}
// 		} else if character.is_numeric() {
// 			self.characters.push(character);

// 			ConsumeAction::Accept
// 		} else if character == '.' {
// 			if self.has_decimal {
// 				ConsumeAction::Reject
// 			} else if context.next_char_lookahead().is_some()
// 				&& context.next_char_lookahead().unwrap().is_numeric()
// 			{
// 				self.characters.push(character);
// 				self.has_decimal = true;

// 				ConsumeAction::Accept
// 			} else {
// 				ConsumeAction::Reject
// 			}
// 		} else {
// 			ConsumeAction::Reject
// 		}
// 	}

// 	fn reduce(self, _: &mut ParserContext) -> f64 {
// 		let num = self
// 			.characters
// 			.iter()
// 			.collect::<String>()
// 			.parse()
// 			.unwrap_or(0.0);

// 		if self.is_negative {
// 			num * -1.0
// 		} else {
// 			num
// 		}
// 	}
// }
