#[macro_export]
macro_rules! run {
	($function: expr, $message:tt) => {{
		#[allow(unused_imports)]
		use crate::{ArrayRun, Runner, ScalarRun};
		Runner($message).run($function)
	}};

	($function: expr, $first_arg:tt, $second_arg:tt) => {{
		#[allow(unused_imports)]
		use crate::{ArrayRun, Runner, ScalarRun};
		let runified = |arg| run!(|sec| $function(arg, sec), $second_arg);
		Runner($first_arg).run(runified)
	}};
}

struct Runner<T>(T);

trait ScalarRun<F, O> {
	fn run(self, func: F) -> O;
}

// Default implementation for scalars
//
// Note that the Self type of this impl is &Printer<T> and so the
// method argument is actually &&T!
// That makes this impl lower priority during method
// resolution than the implementation for `Print` above.
// Copy is a temporary fix here
impl<T: Copy, F, O> ScalarRun<F, O> for &Runner<T>
where
	F: FnOnce(T) -> O,
{
	fn run(self, func: F) -> O {
		func(self.0)
	}
}

trait ArrayRun<F, O> {
	fn run(self, func: F) -> Vec<O>;
}

// Specialized implementation for arrays
impl<T: std::iter::IntoIterator, F, O> ArrayRun<F, O> for Runner<T>
where
	F: Fn(T::Item) -> O,
{
	fn run(self, func: F) -> Vec<O> {
		self.0.into_iter().map(func).collect()
	}
}

#[cfg(test)]
mod tests {
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

	#[test]
	fn dyadic_1d_array_first_arg() {
		let array = vec![2, 3];
		let result = run!(add, array, 2);
		assert_eq!(result, vec![4, 5]);
	}

	#[test]
	fn dyadic_1d_array_second_arg() {
		let array = vec![2, 3];
		let result = run!(add, 2, array);
		assert_eq!(result, vec![4, 5]);
	}

	#[ignore = "unimplemented"]
	#[test]
	fn dyadic_2d_array() {
		// let array1 = vec![2, 3];
		// let array2 = vec![4, 5];
		// let result = run!(add, array1, array2);
		// assert_eq!(result, vec![vec![6, 7], vec![7, 8]]);
	}

	fn add_one(num: usize) -> usize {
		num + 1
	}

	fn add(left: usize, right: usize) -> usize {
		left + right
	}
}
