mod cli;

use crate::cli::{CliArgs, CliCommands, FractionalKnapsack};
use anyhow::Context;
use aud2::knapsack::{
    fractional_knapsack_greedy, knapsack_dynamic_programming, knapsack_integer_greedy, Item,
    PackedItem,
};
use aud2::subset_sum::{subset_sum_full_bool_table, subset_sum_row_sum_set};
use fraction::Fraction;
use std::fs;

fn main() -> anyhow::Result<()> {
    init_logger();

    let cli_args: cli::CliArgs = argh::from_env();
    match cli_args.subcommand {
        CliCommands::FractionalKnapsack(frac_knapsack) => {
            fractional_knapsack_greedy_autoprint(frac_knapsack)
        }
        CliCommands::MaximumKnapsackDynamicProgramming(max_knapsack) => {
            maximum_knapsack_dynamic_programming_autoprint(max_knapsack)
        }
        _ => unimplemented!(),
    }
}

/// Calls the library function fractional_knapsack() and prints its results.
fn fractional_knapsack_greedy_autoprint(cli_args: cli::FractionalKnapsack) -> anyhow::Result<()> {
    let FractionalKnapsack {
        items_csv,
        weight_limit: weight_capacity,
        flipped_csv,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;

    let chosen_items = fractional_knapsack_greedy(&items, weight_capacity);
    for chosen_item in &chosen_items {
        println!(
            "id={:<2} x={:<3}",
            chosen_item.item.id, chosen_item.take_portion
        );
    }
    let total_profit: Fraction = chosen_items.iter().map(PackedItem::effective_profit).sum();
    println!("total_profit={}", total_profit);

    Ok(())
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

fn maximum_knapsack_dynamic_programming_autoprint(
    cli_args: cli::MaximumKnapsackDynamicProgramming,
) -> anyhow::Result<()> {
    let cli::MaximumKnapsackDynamicProgramming {
        items_csv,
        flipped_csv,
        weight_limit,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;
    let max_profit = knapsack_dynamic_programming(&items, weight_limit);
    println!("Maximum reachable profit: {:?}", max_profit);
    Ok(())
}

fn knapsack_integer_greedy_autoprint(items: &[Item], weight_capacity: u64) {
    let knapsack = knapsack_integer_greedy(items, weight_capacity);
    println!("Knapsack: {:#?}", knapsack);
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

/// Read and parse the csv file `filename` into a `Vec<T>`.
fn read_csv<T>(filename: &str, flipped: bool) -> anyhow::Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut csv =
        fs::read_to_string(filename).with_context(|| format!("Open csv file {}", filename))?;
    if flipped {
        csv = flip_csv(csv);
    }
    let mut csv_reader = csv::Reader::from_reader(csv.as_bytes());
    let items: Result<Vec<T>, _> = csv_reader.deserialize::<T>().collect();
    items.context("Parse csv file")
}

/// Initialize the logger.
fn init_logger() {
    env_logger::init();
}
