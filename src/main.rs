mod cli;

use aud2::knapsack::{fractional_knapsack, maximum_knapsack, Item, PackedItem};
use aud2::subset_sum::{subset_sum_full_bool_table, subset_sum_row_sum_set};
use fraction::Fraction;

fn main() {
    init_logger();

    let cli_args: cli::CliArgs = argh::from_env();
    println!("{:#?}", cli_args);
}

/// Calls the library function fractional_knapsack() and prints its results.
fn fractional_knapsack_autoprint(items: &[Item], weight_capacity: u64) {
    let chosen_items = fractional_knapsack(&items, weight_capacity);
    for chosen_item in &chosen_items {
        println!(
            "id={:<2} x={:<3}",
            chosen_item.item.id, chosen_item.take_fraction
        );
    }
    let total_profit: Fraction = chosen_items.iter().map(PackedItem::effective_profit).sum();
    println!("total_profit={}", total_profit);
}

fn subset_sum_autoprint(numbers: &[u64]) {
    let table = subset_sum_row_sum_set(numbers);
    for (i, row) in table.iter().enumerate() {
        println!("i={}: {:?}", i, row);
    }
}

fn subset_sum2_autoprint(numbers: &[u64]) {
    let table = subset_sum_full_bool_table(numbers);
    for (i, row) in table.iter().enumerate() {
        let reachable_sums: Vec<_> = row
            .iter()
            .enumerate()
            .filter(|(_, cell)| **cell == true)
            .map(|(index, _)| index)
            .collect();
        println!("i={}: {:?}", i, reachable_sums);
    }

    println!("{:?}", table);
}

fn maximum_knapsack_autoprint(items: &[Item], weight_capacity: u64) {
    let table = maximum_knapsack(items, weight_capacity);
    for (i, row) in table.iter().enumerate() {
        println!("i={}: {:?}", i, row);
    }
}

/// Transpose a Vec<Vec<T>>, i.e. flip rows and columns. All inner Vec's must have the same length.
/// From https://stackoverflow.com/a/64499219/14350146
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        // No work to do for an empty vec
        return v;
    };
    // All inner vec's must have the same length!
    let len = v[0].len();
    // Get iterators from all inner vec
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            // Drive each iterator one forward and collect the results
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

/// Flips / transposes a CSV. This allows converting CSVs build from left to right to a normal CSV.
fn flip_csv(csv: String) -> String {
    let lines: Vec<Vec<&str>> = csv
        .lines() // Split the lines
        // Split at comma
        .map(|line| line.split(',').collect())
        .collect();
    // Transpose lines and columns
    let transposed_lines = transpose(lines);
    // Convert transposed lines back to a String
    transposed_lines
        .into_iter()
        // Join columns with comma
        .map(|line| line.join(","))
        .collect::<Vec<_>>()
        // And join lines with newline
        .join("\n")
}

/// Initialize the logger.
fn init_logger() {
    simple_logger::SimpleLogger::new().init().unwrap();
}
