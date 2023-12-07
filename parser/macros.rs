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
