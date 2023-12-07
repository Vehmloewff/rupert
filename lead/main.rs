use rupert_lead::parse_expression;
use rupert_parser::InputStream;

fn main() {
	let code = "4 + 5 * 6";
	let stream = if true {
		InputStream::new(code)
	} else {
		InputStream::new(code)
	};

	dbg!(parse_expression(stream));
}
