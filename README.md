# comp Macro Project

This is a fun little project that implements a Rust procedural macro comprehensions! mimicking Python's list comprehension syntax. It allows you to iterate over iterables and apply transformations concisely.

Example
```rust
let my_vec = list_comp![x * 2 for x in vec![1, 2, 3]];
assert_eq(my_vec, vec![2, 4, 6]);
```

### Disclaimer
This project is made for fun. Things might change quickly. Use at your own risk!