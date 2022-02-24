//! Solving of the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem).
//!
//! From Wikipedia: "In its most general formulation, there is a multiset S of integers and a target-sum T, and the
//! question is to decide whether any subset of the integers sum to precisely T."

use log::log_enabled;
use std::collections::HashSet;
use std::ops::Not;

/// Solves the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) via
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn subset_sum_set(numbers: &[u64], limit: u64) -> bool {
    // Contains a set of sum that are producible by using (some of) the first i numbers.
    let mut row: HashSet<u64> = HashSet::new();
    // The number 0 can be produced with the first 0 numbers.
    row.insert(0);

    /// Helper function for logging
    fn log_row(row: &HashSet<u64>, i: usize) {
        // Log this row as sorted sums. Only do this computation when logging is enabled for this level.
        let log_level = log::Level::Debug;
        if log_enabled!(log_level) {
            let mut sorted_sums = Vec::from_iter(row);
            sorted_sums.sort();
            log::log!(
                log_level,
                "i={}: reachable {} sums: {:?}",
                i,
                sorted_sums.len(),
                sorted_sums
            );
        }
    }
    log_row(&row, 0);

    // Examine which numbers are producible by using a new number from the number list.
    for (i, new_number) in numbers.iter().enumerate() {
        let last_row = row.clone();
        // All previously reachable numbers are still reachable
        // In addition, each old number + new_number is now also reachable
        for already_reachable_sum in last_row {
            let new_reachable_sum = already_reachable_sum + new_number;
            if new_reachable_sum > limit {
                continue;
            }
            row.insert(new_reachable_sum);
        }
        // The first row is actually before this loop
        log_row(&row, i + 1);
    }
    row.contains(&limit)
}

/// Solves the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) via
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn subset_sum_vec(numbers: &[u64], limit: u64) -> bool {
    // Convert u64 to usize to provide a consistent API for subset sum implementations
    let limit = limit as usize;
    // Contains a list of bools indicating which numbers can be produced by summing (some of) the first i numbers.
    let mut row: Vec<bool> = vec![false; limit as usize + 1];
    // The number 0 can be produced with the first 0 numbers.
    row[0] = true;

    /// Helper function for logging
    fn log_row(row: &[bool], i: usize) {
        let log_level = log::Level::Debug;
        if log_enabled!(log_level) {
            // Extract sums that are producible
            let row_sums: Vec<usize> = row
                .iter()
                .enumerate()
                .filter(|(_sum, &reachable)| reachable)
                .map(|(sum, _reachable)| sum)
                .collect();
            log::log!(
                log_level,
                "i={} reachable {} sums: {:?}",
                i,
                row_sums.len(),
                row_sums
            );
        }
    }
    log_row(&row, 0);

    // Examine which numbers are producible by using a new number from the number list.
    for (i, new_number) in numbers.iter().copied().enumerate() {
        let last_row = row.clone();
        // All previously reachable numbers are still reachable
        // In addition, each old number + new_number is now also reachable
        for (sum, reachable) in last_row.iter().enumerate() {
            if reachable.not() {
                // Sum previously not producible, so no new sum here
                continue;
            }
            // New sum producible. Only set bool to true if not out of bounds (would result in .get_mut() return None)
            if let Some(new_sum) = row.get_mut(sum + (new_number as usize)) {
                *new_sum = true;
            }
        }
        log_row(&row, i + 1);
    }
    row[limit]
}
