//! This example shows the usage of the subset sum algorithms.
//!
//! This example just calls the aud2 binary with a target sum and some numbers, which can be used to reach this target
//! sum. and a weight limit. Other subset sum command work mostly the same.

use std::ops::Not;
use std::process;

fn main() {
    let success = process::Command::new("cargo")
        .args([
            "run",
            "--",
            "subsum-row",
            "--sum",
            "174", // Sum to reach
            // Numbers which can be used below
            "7",
            "13",
            "17",
            "20",
            "29",
            "31",
            "31",
            "35",
            "57",
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
