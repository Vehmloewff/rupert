#[macro_export]
macro_rules! any {
    ($stream:ident; $($func:ident),+) => {
        'block: {
            let stream = $stream;

            $(
                let stream = match $func(stream) {
                    $crate::ParseResult::Reject(stream) => stream,
                    $crate::ParseResult::Built(stream, node) => break 'block $crate::ParseResult::Built(stream, node),
                };
            )*

            $crate::ParseResult::Reject(stream)
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
                    $crate::WrapResult::Reject(stream, node) => (stream, node),
                    $crate::WrapResult::Built(stream, node) => break 'block $crate::WrapResult::Built(stream, node),
                };
            )*

            $crate::WrapResult::Reject(stream, node)
        }
    };
}

#[macro_export]
macro_rules! wrap_recursive {
	($stream:ident, $node:ident; $($func:ident),+) => {
        {
            let mut result = $crate::wrap!($stream, $node; $($func),+);

			loop {
				if let $crate::WrapResult::Built(stream, node) = result {
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
		let stream = $crate::InputStream::new(input);

		let res: $crate::ParseResult<_> = $func(stream);

		let (stream, ast) = match res {
			$crate::ParseResult::Built(stream, ast) => (stream, Some(ast)),
			$crate::ParseResult::Reject(stream) => (stream, None),
		};

		let index = stream.get_index();
		let (unparsed, mut diagnostics) = stream.finish();

		if unparsed.len() > 0 {
			diagnostics.push($crate::Diagnostic::new_error(
				$crate::Span::new(index, index + unparsed.len()),
				"Could not parse input",
			))
		}

		(ast, diagnostics)
	}};
}
