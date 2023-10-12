# Array based rust

This crate allows you to pass arrays and vecs into fuctions with a simple wrapper

```rust
use array_based::*;

fn main() {
	let result = run!(add, 2, vec![4, 5, 6]);
	assert_eq!(result, vec![6, 7, 8])
}

fn add(num1: usize, num2: usize) -> usize {
	return num1 + num2;
}
```