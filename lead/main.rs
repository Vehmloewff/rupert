mod expression;
mod whitespace;

use crate::expression::parse_expression;
use rupert_parser::parse;

fn main() {
	let code = "3 + 9 || 4 - 2 && 32 > 31 - 43 * 2";
	let (ast, diagnostics) = parse!(code, parse_expression);

	println!("Parsed with {} diagnostic(s)", diagnostics.len());
	dbg!(ast);
}
