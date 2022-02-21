use argh::FromArgs;

/// AuD2: Algorithms from "Algorithms and Data Structures 2" implemented in Rust.
#[derive(FromArgs, PartialEq, Debug)]
pub(crate) struct CliArgs {
    #[argh(subcommand)]
    pub(crate) subcommand: CliCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub(crate) enum CliCommands {
    FractionalKnapsack(FractionalKnapsack),
    KnapsackDynamicProgramming(KnapsackDynamicProgramming),
    KnapsackGreedyK(KnapsackGreedyK),
    SubsetSumRowSumSet(SubsetSumRowSumSet),
    SubsetSumFullBoolTable(SubsetSumFullBoolTable),
    Knapsack01(Knapsack01),
}

/// FractionalKnapsack
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frac-ks")]
pub(crate) struct FractionalKnapsack {
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

/// Solve maximum knapsack with the greedy_k approximation algorithm.
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
pub(crate) struct SubsetSumRowSumSet {}

/// SubsetSumFullBoolTable
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "subsum-full")]
pub(crate) struct SubsetSumFullBoolTable {}

/// 0-1-Knapsack
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ks-01")]
pub(crate) struct Knapsack01 {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    pub(crate) csv_file: String,
    /// enable this flag if your CSV is written from left to right.
    #[argh(switch, short = 'f')]
    pub(crate) flipped_csv: bool,
}
