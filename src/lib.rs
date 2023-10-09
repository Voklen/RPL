struct Runner<T>(T);

trait Print<F, O> {
	fn run(self, func: F) -> Vec<O>;
}

// specialized implementation
impl<T: std::iter::IntoIterator, F, O> Print<F, O> for Runner<T>
where
	F: Fn(T::Item) -> O,
{
	fn run(self, func: F) -> Vec<O> {
		self.0.into_iter().map(func).collect()
	}
}

trait DefaultPrint<A, B> {
	fn run(self, func: fn(A) -> B) -> B;
}

// default implementation
//
// Note that the Self type of this impl is &Printer<T> and so the
// method argument is actually &&T!
// That makes this impl lower priority during method
// resolution than the implementation for `Print` above.
// Copy is a temporary fix here
impl<T: Copy, B> DefaultPrint<T, B> for &Runner<T> {
	fn run(self, func: fn(T) -> B) -> B {
		func(self.0)
	}
}

#[macro_export]
macro_rules! run {
    ($function: expr, $message:tt) => {{
		#[allow(unused_imports)]
		use	crate::{Runner, DefaultPrint, Print};
		Runner($message).run($function)
    }};

    ($function: expr, $first_arg:tt, $($message:tt)*) => {{
		// use	crate::{Runner,Print};
		// Runner($first_arg).run($function);
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
	fn monadic_array() {
		let array = vec![2, 3];
		let result = run!(add_one, array);
		assert_eq!(result, vec![3, 4]);
	}

	#[test]
	fn dyadic() {
		let result = run!(add, 2, 3);
		assert_eq!(result, 5);
	}
}
