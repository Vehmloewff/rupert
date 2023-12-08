use crate::expression::Expression;
use rupert_parser::{InputStream, WrapResult};

pub fn nerf_whitespace(mut stream: InputStream, inner: Expression) -> WrapResult<Expression> {
	stream.consume_whitespace();

	WrapResult::Reject(stream, inner)
}
