# comp Macro Project

This is a fun little project that implements Rust procedural macro comprehensions! These comprehensions are micking Python's list comprehension syntax. They allow you to iterate over iterables and apply transformations concisely and in a Pythonic way :D

Example
```rust
let my_vec = list_comp![x * 2 for x in vec![1, 2, 3]];
assert_eq(my_vec, vec![2, 4, 6]);

let my_set = set_comp![x * 2 for x in vec![1, 2, 3]];
assert_eq(my_set, HashSet::from([2, 4, 6]));

let my_iter = iter_comp![x * 2 for x in vec![1, 2, 3] if x != 2];
assert!(Iterator::eq(my_iter, vec![2, 6]))

let input = vec![
    vec![vec![1, 2, 3], vec![1, 2, 3, 4]],
    vec![vec![3, 2, 1], vec![3, 2, 1, 0]],
];
let my_iter = iter_comp![x for outer in input for inner in outer if inner.len() == 3 for x in inner];
assert!(Iterator::eq(my_iter, vec![1, 2, 3, 3, 2, 1]));
```

### Disclaimer
This project is made for fun. Things might change quickly. Use at your own risk!