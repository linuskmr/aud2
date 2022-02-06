# aud2

Algorithms taught at my university in the course "Algorithms and Data Structures 2" implemented in Rust.

Currently supported problems/algorithms:

- [Fractional Knapsack](https://en.wikipedia.org/wiki/Continuous_knapsack_problem)
  - Solving via [Greedy Algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm)
- [Subset Sum](https://en.wikipedia.org/wiki/Subset_sum_problem)
  - Solving via [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming)
- [Maximum Knapsack](https://en.m.wikipedia.org/wiki/Knapsack_problem)
  - Solving via [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming)
- [0-1-Knapsack](https://en.m.wikipedia.org/wiki/Knapsack_problem) (decision problem)
  - Solving via [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming)

## Installation

[Install Rust and Cargo](https://rustup.rs/)

### Install as executable

```
$ cargo install --git https://github.com/linuskmr/aud2
```

```
$ aud2 <...args>
```

### Use as dependency

```toml
[dependencies]
aud2 = { git = "https://github.com/linuskmr/aud2" }
```
