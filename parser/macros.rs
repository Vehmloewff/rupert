#[macro_export]
macro_rules! any {
    ($stream:ident; $($func:ident),+) => {
        'block: {
            let stream = $stream;

            $(
                let stream = match $func(stream) {
                    ParseResult::Reject(stream) => stream,
                    ParseResult::Built(stream, node) => break 'block ParseResult::Built(stream, node),
                };
            )*

            ParseResult::Reject(stream)
        }
    };
}

#[macro_export]
macro_rules! wrap {
    ($stream:ident, $node:ident; $($func:ident),+) => {
        'block: {
            let stream = $stream;
            let node = $node;

            $(
                let (stream, node) = match $func(stream, node) {
                    WrapResult::Reject(stream, node) => (stream, node),
                    WrapResult::Built(stream, node) => break 'block WrapResult::Built(stream, node),
                };
            )*

            WrapResult::Reject(stream, node)
        }
    };
}

#[macro_export]
macro_rules! wrap_recursive {
	($stream:ident, $node:ident; $($func:ident),+) => {
        {
            let mut result = $crate::wrap!($stream, $node; $($func),+);

			loop {
				if let WrapResult::Built(stream, node) = result {
					result = $crate::wrap!(stream, node; $($func),+);
				}

				break result;
			}
        }
    };
}

#[macro_export]
macro_rules! parse {
	($input:ident, $func:ident) => {{
		let input: &str = $input;
		let stream = InputStream::new(input);

		let res: ParseResult<_> = $func(stream);

		let (stream, ast) = match res {
			ParseResult::Built(stream, ast) => (stream, Some(ast)),
			ParseResult::Reject(stream) => (stream, None),
		};

		let index = stream.get_index();
		let (unparsed, mut diagnostics) = stream.finish();

		if unparsed.len() > 0 {
			diagnostics.push(Diagnostic::new("Could not parse input", Span::new(index, index + unparsed.len())))
		}

		(ast, diagnostics)
	}};
}
