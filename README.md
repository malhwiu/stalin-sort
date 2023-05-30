# Stalin sort

Stalin sort is a simple and fast sorting algorithm which simply removes elements that are out of order

## How to:

### Add dependency

```toml
[dependencies]
stalin_sort = { git = "https://github.com/malhwiu/stalin-sort.git", version="0.1.0" }
```

### Use

```rust
use stalin_sort::stalin_sort;

array = stalin_sort(array, true);
```

> array must be mutable!
