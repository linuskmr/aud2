use argh::FromArgs;

/// AuD2: Algorithms from "Algorithms and Data Structures 2" implemented in Rust.
#[derive(FromArgs, PartialEq, Debug)]
pub struct CliArgs {
    #[argh(subcommand)]
    subcommand: MySubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommands {
    FractionalKnapsack(FractionalKnapsack),
    MaximumKnapsack(MaximumKnapsack),
    SubsetSumRowSumSet(SubsetSumRowSumSet),
    SubsetSumFullBoolTable(SubsetSumFullBoolTable),
}

/// FractionalKnapsack
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frac-ks")]
struct FractionalKnapsack {
    /// path to a csv file with the input elements (id, weight, profit).
    #[argh(positional)]
    csv_file: String,
    /// enable this flag if your CSV is written from left to right.
    #[argh(switch)]
    flipped_csv: bool,
}

/// MaximumKnapsack
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "max-ks")]
struct MaximumKnapsack {}

/// SubsetSumRowSumSet
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "subsum-row")]
struct SubsetSumRowSumSet {}

/// SubsetSumFullBoolTable
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "subsum-full")]
struct SubsetSumFullBoolTable {}
