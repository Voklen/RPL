struct Runner<T>(T);

trait Print<A, B> {
	fn run(self, func: fn(A) -> B);
}

// specialized implementation
impl<T: std::iter::Iterator, B> Print<T, B> for Runner<T> {
	fn run(self, func: fn(T) -> B) {
		func(self.0);
	}
}

trait DefaultPrint<A, B> {
	fn run(self, func: fn(A) -> B);
}

// default implementation
//
// Note that the Self type of this impl is &Printer<T> and so the
// method argument is actually &&T!
// That makes this impl lower priority during method
// resolution than the implementation for `Print` above.
impl<T, B> DefaultPrint<T, B> for &Runner<T> {
	fn run(self, func: fn(T) -> B) {
		println!("I cannot be printed");
	}
}

fn main() {
	let not_printable = Runner(());
	let printable = Runner("Hello World");

	not_printable.run(|_| {});
	printable.run(|_: &str| {});
}

#[macro_export]
macro_rules! run {
    ($function: expr, $message:tt) => {{
		use	crate::{Runner, DefaultPrint, Print};
		Runner($message).run($function);
        $function($message)
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
		use super::main;
		main();
		let result = run!(add_one, 2);
		assert_eq!(result, 3);
	}

	#[test]
	fn dyadic() {
		let result = run!(add, 2, 3);
		assert_eq!(result, 5);
	}
}
