//! This example shows the usage of the fractional knapsack / fractional greedy algorithm.
//!
//! The items are listed in `fractional_knapsack.csv` in flipped direction. A normal CSV would list entries in rows.
//! Sometimes it is more comfortable to write the CSV from left to right, so AuD2 supports this with the `--flipped-csv`
//! flag.
//!
//! This example just calls the aud2 binary with the above-mentioned CSV file and a weight limit. Other knapsack
//! commands work mostly the same.

use std::ops::Not;
use std::process;

fn main() {
    let success = process::Command::new("cargo")
        .args([
            "run",
            "--",
            "frac-ks",
            "--items-csv",
            "examples/fractional_knapsack.csv",
            "--weight-limit",
            "120",
            "--flipped-csv",
        ])
        .spawn()
        .expect("Failed to execute aud2 process")
        .wait()
        .expect("Can not wait for aud2 subprocess")
        .success();
    if success.not() {
        panic!("Subprocess aud2 terminated with non-ok exit code");
    }
}
