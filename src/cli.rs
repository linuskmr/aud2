//! Type definitions for command line argument parsing via [argh].

use argh::FromArgs;

/// AuD2: Algorithms from "Algorithms and Data Structures 2" implemented in Rust.
#[derive(FromArgs, PartialEq, Debug)]
pub(crate) struct CliArgs {
    #[argh(subcommand)]
    pub(crate) subcommand: CliCommands,
}

/// Enum of all subcommands.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub(crate) enum CliCommands {
    KnapsackFractionalGreedy(KnapsackFractionalGreedy),
    KnapsackDynamicProgramming(KnapsackDynamicProgramming),
    KnapsackBranchBound(KnapsackBranchBound),
    KnapsackGreedyK(KnapsackGreedyK),
    SubsetSumRowSumSet(SubsetSumRowSumSet),
    SubsetSumFullBoolTable(SubsetSumFullBoolTable),
    KnapsackIntegerGreedy(KnapsackIntegerGreedy),
}

/// FractionalKnapsack
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frac-ks")]
pub(crate) struct KnapsackFractionalGreedy {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    pub(crate) items_csv: String,

    /// enable this flag if your CSV is written from left to right.
    #[argh(switch, short = 'f')]
    pub(crate) flipped_csv: bool,

    /// maximum weight of the knapsack.
    #[argh(positional)]
    pub(crate) weight_limit: u64,
}

/// Solve maximum knapsack with dynamic programming.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ks-dp")]
pub(crate) struct KnapsackDynamicProgramming {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    pub(crate) items_csv: String,

    /// enable this flag if your CSV is written from left to right.
    #[argh(switch, short = 'f')]
    pub(crate) flipped_csv: bool,

    /// maximum weight of the knapsack.
    #[argh(positional)]
    pub(crate) weight_limit: u64,
}

/// Solve maximum knapsack with branch and bound.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ks-bb")]
pub(crate) struct KnapsackBranchBound {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    pub(crate) items_csv: String,

    /// enable this flag if your CSV is written from left to right.
    #[argh(switch, short = 'f')]
    pub(crate) flipped_csv: bool,

    /// maximum weight of the knapsack.
    #[argh(positional)]
    pub(crate) weight_limit: u64,
}

/// Solve maximum knapsack with the greedy_k approximation algorithm. The result may not be optimal.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ks-greedyk")]
pub(crate) struct KnapsackGreedyK {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    pub(crate) items_csv: String,

    /// enable this flag if your CSV is written from left to right.
    #[argh(switch, short = 'f')]
    pub(crate) flipped_csv: bool,

    /// maximum weight of the knapsack.
    #[argh(positional)]
    pub(crate) weight_limit: u64,

    /// number of fixed items.
    #[argh(positional)]
    pub(crate) k: usize,
}

/// SubsetSumRowSumSet
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "subsum-row")]
pub(crate) struct SubsetSumRowSumSet {
    /// numbers of the subset sum instance.
    #[argh(positional)]
    pub(crate) numbers: Vec<u64>,
}

/// SubsetSumFullBoolTable
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "subsum-full")]
pub(crate) struct SubsetSumFullBoolTable {}

/// Solve maximum knapsack with integer greedy. The result may not be optimal.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ks-ig")]
pub(crate) struct KnapsackIntegerGreedy {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    pub(crate) items_csv: String,

    /// enable this flag if your CSV is written from left to right.
    #[argh(switch, short = 'f')]
    pub(crate) flipped_csv: bool,

    /// maximum weight of the knapsack.
    #[argh(positional)]
    pub(crate) weight_limit: u64,
}
