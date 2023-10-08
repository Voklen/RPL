#[macro_export]
macro_rules! run {
    ($function: expr, $message:tt) => {{
        $function($message)
    }};

    ($function: expr, $first_arg:tt, $($message:tt)*) => {{
        $function($first_arg, $($message)*)
    }}
}

#[cfg(test)]
mod tests {
	fn add_one(num: usize) -> usize {
		num + 1
	}

	pub fn add(left: usize, right: usize) -> usize {
		left + right
	}

	#[test]
	fn monadic() {
		let result = run!(add_one, 2);
		assert_eq!(result, 3);
	}

	#[test]
	fn dyadic() {
		let result = run!(add, 2, 3);
		assert_eq!(result, 5);
	}
}
