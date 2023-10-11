use std::marker::{Send, Sync};

#[macro_export]
macro_rules! run {
	($function: expr, $arg:expr) => {{
		#[allow(unused_imports)]
		use crate::{ArrayRun, Runner, ScalarRun};
		Runner($arg).run($function)
	}};

	($function: expr, $first_arg:expr, $($other_args:expr),+) => {{
		#[allow(unused_imports)]
		use crate::{ArrayRun, Runner, ScalarRun};
		let runified = |arg| {
			run!(
				make_closure!(
					$function,
					arg
					$(,$other_args)+
				)
				$(,$other_args.clone())+
			)
		};
		Runner($first_arg).run(runified)
	}};
}

#[allow(unused_macros)]
macro_rules! make_closure {
	($function: expr, $arg: expr, $a:expr) => {
		|a| $function($arg, a)
	};
	($function: expr, $arg: expr, $a:expr, $b:expr) => {
		|a, b| $function($arg, a, b)
	};
	($function: expr, $arg: expr, $a:expr, $b:expr, $c:expr) => {
		|a, b, c| $function($arg, a, b, c)
	};
	($function: expr, $arg: expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
		|a, b, c, d| $function($arg, a, b, c, d)
	};
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
impl<T, F, O> ArrayRun<F, O> for Runner<T>
where
	T: rayon::iter::IntoParallelIterator,
	F: Fn(T::Item) -> O + Send + Sync,
	O: Send,
{
	fn run(self, func: F) -> Vec<O> {
		use rayon::prelude::*;
		self.0.into_par_iter().map(func).collect()
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
		let result = run!(add_one, vec![2, 3]);
		assert_eq!(result, vec![3, 4]);
	}

	#[test]
	fn dyadic() {
		let result = run!(add, 2, 3);
		assert_eq!(result, 5);
	}

	#[test]
	fn dyadic_1d_array_first_arg() {
		let result = run!(add, vec![2, 3], 2);
		assert_eq!(result, vec![4, 5]);
	}

	#[test]
	fn dyadic_1d_array_second_arg() {
		let result = run!(add, 2, vec![2, 3]);
		assert_eq!(result, vec![4, 5]);
	}

	#[test]
	fn dyadic_2d_array() {
		let result = run!(add, vec![2, 3], vec![4, 5]);
		assert_eq!(result, vec![vec![6, 7], vec![7, 8]]);
	}

	#[test]
	fn triadic() {
		let result = run!(tri_add, 2, 3, 10);
		assert_eq!(result, 15);
	}

	#[test]
	fn triadic_1d_array_first_arg() {
		let result = run!(tri_add, vec![2, 3], 2, 5);
		assert_eq!(result, vec![9, 10]);
	}

	#[test]
	fn triadic_3d_array() {
		let result = run!(tri_add, vec![2, 3], vec![0, 1], vec![4, 6]);
		assert_eq!(
			result,
			vec![vec![vec![6, 8], vec![7, 9]], vec![vec![7, 9], vec![8, 10]]]
		);
	}

	fn add_one(num: usize) -> usize {
		num + 1
	}

	fn add(left: usize, right: usize) -> usize {
		left + right
	}

	fn tri_add(first: usize, second: usize, third: usize) -> usize {
		first + second + third
	}
}
