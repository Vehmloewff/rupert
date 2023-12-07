use rupert_parser::{parse_expression, InputStream};

fn main() {
	let code = "4 + 5 * 6";
	let stream = if true {
		InputStream::new(code)
	} else {
		InputStream::new(code)
	};

	dbg!(parse_expression(stream));
}
