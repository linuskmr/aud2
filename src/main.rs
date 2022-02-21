mod cli;

use crate::cli::{CliArgs, CliCommands, KnapsackFractionalGreedy};
use anyhow::Context;
use aud2::knapsack::{Item, PartialPackedItem};
use aud2::subset_sum::{subset_sum_full_bool_table, subset_sum_row_sum_set};
use fraction::Fraction;
use std::fs;

fn main() -> anyhow::Result<()> {
    init_logger();

    // Parse command line arguments and decide what to do
    let cli_args: CliArgs = argh::from_env();
    invoke_subcommand(cli_args)
}

/// Inspects the passed command line arguments and starts the corresponding cli wrapper function for the selected
/// subcommand.
fn invoke_subcommand(cli_args: CliArgs) -> anyhow::Result<()> {
    match cli_args.subcommand {
        CliCommands::KnapsackFractionalGreedy(sub_cli_args) => {
            knapsack_fractional_greedy_cli(sub_cli_args)
        }
        CliCommands::KnapsackIntegerGreedy(sub_cli_args) => {
            knapsack_integer_greedy_cli(sub_cli_args)
        }
        CliCommands::KnapsackDynamicProgramming(sub_cli_args) => {
            knapsack_dynamic_programming_cli(sub_cli_args)
        }
        CliCommands::KnapsackGreedyK(sub_cli_args) => knapsack_greedy_k_cli(sub_cli_args),
        CliCommands::KnapsackBranchBound(sub_cli_args) => knapsack_branch_and_bound(sub_cli_args),
        CliCommands::SubsetSumRowSumSet(sub_cli_args) => subset_sum_row_set_cli(sub_cli_args),
        CliCommands::SubsetSumFullTable(sub_cli_args) => subset_sum_full_table_cli(sub_cli_args),
    }
}

// The following functions are helper functions. They parse the command line arguments for the corresponding subcommand,
// call a library function and print its result.

/// CLI wrapper for [aud2::knapsack::fractional_greedy].
fn knapsack_fractional_greedy_cli(cli_args: cli::KnapsackFractionalGreedy) -> anyhow::Result<()> {
    let KnapsackFractionalGreedy {
        items_csv,
        weight_limit: weight_capacity,
        flipped_csv,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;

    let chosen_items = aud2::knapsack::fractional_greedy(&items, weight_capacity);
    for chosen_item in &chosen_items {
        println!(
            "id={:<2} x={:<3}",
            chosen_item.item.id, chosen_item.take_portion
        );
    }
    let total_profit: Fraction = chosen_items
        .iter()
        .map(PartialPackedItem::effective_profit)
        .sum();
    println!("total_profit={}", total_profit);

    Ok(())
}

/// CLI wrapper for [subset_sum_row_sum_set].
fn subset_sum_row_set_cli(cli_args: cli::SubsetSumRowSet) -> anyhow::Result<()> {
    let cli::SubsetSumRowSet { numbers } = cli_args;
    println!("Input numbers: {:?}", numbers);
    let table = subset_sum_row_sum_set(&numbers);
    for (i, row) in table.iter().enumerate() {
        println!("i={}: {:?}", i, row);
    }
    Ok(())
}

/// CLI wrapper for [subset_sum_full_bool_table].
fn subset_sum_full_table_cli(cli_args: cli::SubsetSumFullTable) -> anyhow::Result<()> {
    let cli::SubsetSumFullTable { numbers } = cli_args;
    println!("Input numbers: {:?}", numbers);
    let table = subset_sum_full_bool_table(&numbers);
    for row in table {
        println!("{:?}", row);
    }
    Ok(())
}

/// CLI wrapper for [aud2::knapsack::dynamic_programming].
fn knapsack_dynamic_programming_cli(
    cli_args: cli::KnapsackDynamicProgramming,
) -> anyhow::Result<()> {
    let cli::KnapsackDynamicProgramming {
        items_csv,
        flipped_csv,
        weight_limit,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;
    let knapsack = aud2::knapsack::dynamic_programming(&items, weight_limit);
    println!("Knapsack: {:#?}", knapsack);
    println!(
        "Total profit: {}",
        knapsack.iter().map(|item| item.profit).sum::<u64>()
    );
    println!(
        "Total weight {} of allowed weight limit {}",
        knapsack.iter().map(|item| item.weight).sum::<u64>(),
        weight_limit
    );
    Ok(())
}

/// CLI wrapper for [aud2::knapsack::integer_greedy].
fn knapsack_integer_greedy_cli(cli_args: cli::KnapsackIntegerGreedy) -> anyhow::Result<()> {
    let cli::KnapsackIntegerGreedy {
        items_csv,
        weight_limit,
        flipped_csv,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;
    let knapsack = aud2::knapsack::integer_greedy(&items, weight_limit);
    println!("Knapsack: {:#?}", knapsack);
    Ok(())
}

/// CLI wrapper for [aud2::knapsack::greedy_k].
fn knapsack_greedy_k_cli(cli_args: cli::KnapsackGreedyK) -> anyhow::Result<()> {
    let cli::KnapsackGreedyK {
        items_csv,
        flipped_csv,
        weight_limit,
        k,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;
    let knapsack = aud2::knapsack::greedy_k(&items, weight_limit, k);
    println!("Knapsack: {:#?}", knapsack);
    println!(
        "Total profit: {}",
        knapsack.iter().map(|item| item.profit).sum::<u64>()
    );
    println!(
        "Total weight {} of allowed weight limit {}",
        knapsack.iter().map(|item| item.weight).sum::<u64>(),
        weight_limit
    );
    Ok(())
}

/// CLI wrapper for [aud2::knapsack::branch_and_bound].
fn knapsack_branch_and_bound(cli_args: cli::KnapsackBranchBound) -> anyhow::Result<()> {
    let cli::KnapsackBranchBound {
        items_csv,
        flipped_csv,
        weight_limit,
    } = cli_args;
    let items: Vec<Item> = read_csv(&items_csv, flipped_csv).context("Read items")?;
    let knapsack = aud2::knapsack::branch_and_bound(&items, weight_limit);
    println!("Knapsack: {:#?}", knapsack);
    /*println!(
        "Total profit: {}",
        knapsack.iter().map(|item| item.profit).sum::<u64>()
    );
    println!(
        "Total weight {} of allowed weight limit {}",
        knapsack.iter().map(|item| item.weight).sum::<u64>(),
        weight_limit
    );*/
    Ok(())
}

// Other helper functions

/// Transpose a Vec<Vec<T>>, i.e. flip rows and columns. All inner Vec's must have the same length.
///
/// This function is used to flip the orientation of a CSV file.
/// From <https://stackoverflow.com/a/64499219/14350146>
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

/// Flips / transposes a CSV. This allows converting CSVs build from left to right to normal CSVs.
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
