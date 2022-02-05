use aud2::knapsack::{fractional_knapsack, Item, PackedItem};
use aud2::subset_sum::subset_sum;
use std::borrow::Borrow;

fn main() {
    init_logger();

    let items = {
        vec![
            Item {
                id: 1,
                profit: 3,
                weight: 20,
            },
            Item {
                id: 2,
                profit: 3,
                weight: 32,
            },
            Item {
                id: 3,
                profit: 10,
                weight: 40,
            },
            Item {
                id: 4,
                profit: 5,
                weight: 8,
            },
            Item {
                id: 5,
                profit: 2,
                weight: 16,
            },
            Item {
                id: 6,
                profit: 4,
                weight: 4,
            },
            Item {
                id: 7,
                profit: 2,
                weight: 32,
            },
            Item {
                id: 8,
                profit: 9,
                weight: 40,
            },
            Item {
                id: 9,
                profit: 2,
                weight: 8,
            },
            Item {
                id: 10,
                profit: 5,
                weight: 32,
            },
            Item {
                id: 11,
                profit: 3,
                weight: 28,
            },
            Item {
                id: 12,
                profit: 9,
                weight: 20,
            },
            Item {
                id: 13,
                profit: 10,
                weight: 16,
            },
            Item {
                id: 14,
                profit: 3,
                weight: 20,
            },
            Item {
                id: 15,
                profit: 10,
                weight: 40,
            },
            Item {
                id: 16,
                profit: 4,
                weight: 24,
            },
        ]
    };
    let weight_capacity = 120;

    // fractional_knapsack_autoprint(&items, weight_capacity);

    subset_sum_autoprint(&[7, 13, 17, 20, 29, 31, 31, 35, 57]);

    println!("Hello, world!");
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
    let table = subset_sum(numbers);
    /*for (i, row) in table.into_iter().enumerate() {
        println!("i={}: {:?}", i, row);
    }*/

    println!("{:?}", table);
}

/// Initialize the logger.
fn init_logger() {
    simple_logger::SimpleLogger::new().init().unwrap();
}
