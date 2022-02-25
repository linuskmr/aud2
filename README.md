# aud2

Algorithms taught at my university in the course "Algorithms and Data Structures 2" implemented in Rust.

Supported problems and solving algorithms:

- [Fractional Knapsack](https://en.wikipedia.org/wiki/Continuous_knapsack_problem)
  - Solving via the [Fractional Greedy Algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm)
- [Maximum Knapsack](https://en.m.wikipedia.org/wiki/Knapsack_problem)
  - Solving via [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming)
  - Solving via [Branch and Bound](https://en.wikipedia.org/wiki/Dynamic_programming)
  - Approximate solving via [GreedyK](https://en.wikipedia.org/wiki/Greedy_algorithm)
  - Heuristic solving via [Greedy0/integer greedy](https://en.wikipedia.org/wiki/Greedy_algorithm)
- [Subset Sum](https://en.wikipedia.org/wiki/Subset_sum_problem)
  - Solving via [Dynamic Programming](https://en.wikipedia.org/wiki/Dynamic_programming)

## Installation

[Install Rust and Cargo](https://rustup.rs/)

```
git clone https://github.com/linuskmr/aud2
cargo build --release
```

Run:

```
./target/release/aud2
```

### Install as executable

```
$ cargo install --git https://github.com/linuskmr/aud2
```

Run from anywhere on your system:

```
$ aud2 --help
```

### Use as dependency

AuD2 is split into a library and a binary part.
The library part can be used from your software.
The binary part provides an executable command line program.

```toml
[dependencies]
aud2 = { git = "https://github.com/linuskmr/aud2" }
```

## Documentation

```
cargo doc --open
```

## Debugging, logging and tracing

Set the environment variable `RUST_LOG` to trace, debug, info, warn or error to examine the execution.

```
RUST_LOG=debug aud2 --help
```