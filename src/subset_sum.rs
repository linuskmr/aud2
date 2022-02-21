//! Solving of the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem).
//!
//! From Wikipedia: "In its most general formulation, there is a multiset S of integers and a target-sum T, and the
//! question is to decide whether any subset of the integers sum to precisely T."

use std::collections::HashSet;
use std::ops::Not;

/// Solves the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) via
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn subset_sum_row_sum_set(numbers: &[u64]) -> Vec<HashSet<u64>> {
    // table is a list of iterations. Each iteration contains a set of sum that are producible by using (some of)
    // the first i numbers.
    let mut table: Vec<HashSet<u64>> = Vec::with_capacity(numbers.len());
    if numbers.is_empty() {
        return table;
    }
    // The number 0 can be produced with the first 0 numbers.
    table.push(HashSet::from([0]));

    // Examine which numbers are producible by using a new number from the number list.
    for new_number in numbers {
        let last_row = table.last().expect("Table always contains one row");
        // All previously reachable numbers are still reachable
        let mut new_row = last_row.clone();
        // In addition, each old number + new_number is now also reachable
        for already_reachable_number in last_row {
            new_row.insert(already_reachable_number + new_number);
        }
        table.push(new_row);
    }
    table
}

/// Solves the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) via
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn subset_sum_full_bool_table(numbers: &[u64]) -> Vec<Vec<bool>> {
    // table is a list of iterations i. Each iteration contains a list of bools indicating which numbers can be produced
    // by summing (some of) the first i numbers.
    let mut table: Vec<Vec<bool>> = Vec::with_capacity(numbers.len());
    if numbers.is_empty() {
        return table;
    }

    let total_sum: u64 = numbers.iter().sum();

    // The number 0 can be produced with the first 0 numbers.
    table.push({
        // All rows have the same length as the first row. Because we want to examine total_sum itself,
        // it must be included in the vec. Therefore we must add 1, so that first_row[total_sum] is defined.
        let mut first_row = vec![false; (total_sum + 1) as usize];
        first_row[0] = true;
        first_row
    });

    // Examine which numbers are producible by using a new number from the number list.
    for new_number in numbers {
        let last_row = table.last().expect("Table always contains one row");
        // All previously reachable numbers are still reachable
        let mut new_row = last_row.clone();
        // In addition, each old number + new_number is now also reachable
        for (num_last_row, producible_last_row) in last_row.iter().enumerate() {
            if producible_last_row.not() {
                // Number previously not producible, so no new sum here
                continue;
            }
            if let Some(cell) = new_row.get_mut(num_last_row + (*new_number as usize)) {
                // New number producible
                *cell = true;
            }
        }
        table.push(new_row);
    }
    table
}
