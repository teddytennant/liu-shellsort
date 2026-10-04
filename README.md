# liu_shellsort

Shellsort with the executable gap sequence of Bo Liu, arXiv 2609.29881.

```
cargo test
cargo run --release
```

There is no RL search, no complexity proof, and no large-N table. Unit companions h_s+1 are omitted; the paper says they matter only beyond 10^1000. Each pass counts every key comparison and every array write, and equal keys do not swap.
