mod expression;
mod whitespace;

use crate::expression::parse_expression;
use rupert_parser::{parse, Diagnostic};

fn main() {
	let code = "3 + 9 || 4 - 2 && 32 > 31 - 43 * 2 + '{\"Hello\"}{34}'";
	let (ast, diagnostics) = parse!(code, parse_expression);

	for Diagnostic { message, .. } in diagnostics {
		println!("{}", message)
	}
	dbg!(ast);
}
