#[macro_export]
macro_rules! run {
    ($function: expr, $($message:tt)*) => {{
        $function($($message)*)
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
	fn basic() {
		let result = run!(add, 2, 2);
		assert_eq!(result, 4);
	}
}
