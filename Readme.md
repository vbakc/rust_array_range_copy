# array_range_copy

Very simple crate that allows to do this:

```rust
let mut arr = [0; 20];
let arr1 = [1; 10];
arr.copy_array_to::<1>(arr1);
```
