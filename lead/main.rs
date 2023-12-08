mod expression;
mod whitespace;

use rupert_parser::{parse, Diagnostic, InputStream, ParseResult, Span};

use crate::expression::parse_expression;

fn main() {
	let code = "4 + 5 * 6";
	let (ast, _) = parse!(code, parse_expression);

	dbg!(ast);
}
