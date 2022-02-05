use aud2::knapsack::{fractional_knapsack, maximum_knapsack, Item, PackedItem};
use aud2::subset_sum::{subset_sum_full_bool_table, subset_sum_row_sum_set};
use std::borrow::Borrow;

fn main() {
    init_logger();

    let items = [
        Item {
            id: 1,
            profit: 6,
            weight: 2,
        },
        Item {
            id: 2,
            profit: 5,
            weight: 3,
        },
        Item {
            id: 3,
            profit: 8,
            weight: 6,
        },
        Item {
            id: 4,
            profit: 9,
            weight: 7,
        },
        Item {
            id: 5,
            profit: 6,
            weight: 5,
        },
        Item {
            id: 6,
            profit: 7,
            weight: 9,
        },
        Item {
            id: 7,
            profit: 3,
            weight: 4,
        },
    ];

    maximum_knapsack_autoprint(&items, 9);
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
    let total_profit: f64 = chosen_items
        .borrow()
        .into_iter()
        .map(PackedItem::effective_profit)
        .sum();
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

/// Initialize the logger.
fn init_logger() {
    simple_logger::SimpleLogger::new().init().unwrap();
}
