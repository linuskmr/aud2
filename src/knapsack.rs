//! Solving of various [knapsack problems](https://en.wikipedia.org/wiki/Knapsack_problem).
//!
//! From Wikipedia: "Given a set of items, each with a weight and a value, determine the number of each item to include
//! in a collection so that the total weight is less than or equal to a given limit and the total value is as large as
//! possible"

use fraction::Fraction;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt;

// ------- Item ----------------------------------

/// An item is an object that has a profit and weight. An item can be put into a knapsack, which caused the item to be
/// wrapped in an [PackedItem].
#[derive(Eq, PartialEq, Clone, Deserialize)]
pub struct Item {
    /// An unique identifier.
    pub id: usize,
    /// How much benefit / value this item provides.
    pub profit: u64,
    /// How much weight / size this item takes up.
    pub weight: u64,
}

impl Item {
    /// Calculates `weight / profit`. This is an indicator how much value an item has. The lower the ratio, the better it is.
    /// A low ratio means much profit at low weight. A high ratio means low profit at high weight.
    fn weight_profit_ratio(&self) -> f64 {
        (self.weight as f64) / (self.profit as f64)
    }
}

// ------- KnapsackItem ----------------------------------

/// A [Item] that was put inside a knapsack, storing how much of the item or how many items were put into the knapsack.
#[derive(Debug, PartialEq, Clone)]
pub struct PackedItem<'a, TakeNum> {
    /// The original item.
    pub item: &'a Item,
    /// A number indicating how much of the item or how many items were put into the knapsack.
    pub take_fraction: TakeNum,
}

// Partial packed items
impl<'a> PackedItem<'a, Fraction> {
    /// Calculates the weight this item weights considering its take_fraction, i.e. partial packed items.
    pub fn effective_weight(&self) -> Fraction {
        Fraction::from(self.item.weight) * self.take_fraction
    }

    /// Calculates the profit this items gives considering its take_fraction, i.e partial packed items.
    pub fn effective_profit(&self) -> Fraction {
        Fraction::from(self.item.profit) * self.take_fraction
    }
}

// Include the weight_profit_ratio in the debug output.
impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item")
            .field("id", &self.id)
            .field("weight", &self.weight)
            .field("profit", &self.profit)
            .field("weight_profit_ratio", &self.weight_profit_ratio())
            .finish()
    }
}

// Allow items to be compared by their weight_profit_ration.
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight_profit_ratio()
            .partial_cmp(&other.weight_profit_ratio())
    }
}

// Although the f64 returned by self.weight_profit_ratio() does not implement Ord, this is needed to be able to sort a
// collection. So we simply assert that ordering the f64 is always possible, i.e. NaN and infinity are not allowed as
// weight_profit_ratio.
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Illegal values in item which make it unable to be sorted")
    }
}

// ------- Solving Algorithms ----------------------------------

/// Solves the [fractional knapsack problem](https://en.wikipedia.org/wiki/Continuous_knapsack_problem) by using the
/// [greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm).
pub fn fractional_knapsack(items: &[Item], weight_capacity: u64) -> Vec<PackedItem<Fraction>> {
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let mut items_sorted_asc: Vec<&Item> = items.iter().collect();
    items_sorted_asc.sort();

    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc.iter().map(|item| item.id).collect();
        log::debug!("Sorted item ids: {:?}", items_sorted_ids);
    }

    // Items that are selected for to be contained in the knapsack
    let mut knapsack: Vec<PackedItem<Fraction>> = Vec::new();

    for (index, item) in items_sorted_asc.iter().enumerate() {
        // Calculate already used weight, remaining available weight and the currently reached profit
        let used_knapsack_weight: Fraction = knapsack
            .iter()
            .map(|packed_item| packed_item.effective_weight())
            .sum();
        let available_knapsack_weight: Fraction =
            Fraction::from(weight_capacity) - used_knapsack_weight;
        let reached_knapsack_profit: Fraction = knapsack
            .iter()
            .map(|packed_item| packed_item.effective_profit())
            .sum();

        if available_knapsack_weight <= Fraction::from(0) {
            // The knapsack is full / reached its weight capacity. We can not put any more elements into it.
            break;
        }

        // How much of the element do we want to take? Maximum 100% or less, if there is not enough space for the entire
        // item.
        let take_fraction: Fraction = {
            let take_fraction = available_knapsack_weight / Fraction::from(item.weight);
            if take_fraction > Fraction::from(1) {
                Fraction::from(1)
            } else {
                take_fraction
            }
        };
        // Add item to knapsack
        let knapsack_item = PackedItem {
            item,
            take_fraction,
        };
        knapsack.push(knapsack_item);

        log::debug!("round={:<2} current_id={:<2} take_fraction={} available_capacity={:<3} used_capacity={:<3} effective_profit={:<2}",
				 index, item.id, take_fraction, available_knapsack_weight, used_knapsack_weight, reached_knapsack_profit);
    }

    knapsack
}

/// Solves the [maximum knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) with
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn maximum_knapsack(items: &[Item], weight_capacity: u64) -> Vec<Vec<u64>> {
    // table is a list of iterations. Each iteration contains a set of sum that are producible by using (some of)
    // the first i numbers.
    let mut table: Vec<Vec<u64>> = Vec::with_capacity(items.len());
    if items.len() == 0 {
        return table;
    }
    // The number 0 can be produced with the first 0 numbers. Because we want to examine total_sum itself,
    // it must be included in the vec. Therefore we must add 1, so that table[0][weight_capacity] is defined.
    table.push(vec![0u64; (weight_capacity + 1) as usize]);

    // Examine which numbers are producible by using a new number from the number list.
    for (new_item_index, new_item) in items.iter().enumerate() {
        let last_row = table.last().expect("Table always contains one row");
        let mut new_row = Vec::with_capacity(last_row.len());
        // Create the new row by inspecting the old one and inspect if improvement can be made by using the new item.
        for (current_weight_capacity, old_profit) in last_row.iter().enumerate() {
            // Why new_item_index + 1? The first new item corresponds to row=1.
            let (row, column) = (new_item_index + 1, current_weight_capacity);
            // This is the profit we insert into this cell. This is determined by the if-else blocks below
            let profit: u64;
            // How much profit is possible with the new item?
            let new_reachable_profit = {
                // How much capacity would be free, if we use new_item here?
                let free_weight_capacity =
                    current_weight_capacity.saturating_sub(new_item.weight as usize);
                // What profit can be reached with the capacity left?
                let profit_reachable_with_left_weight = last_row[free_weight_capacity];
                // In the end, we can get the profit of the new item + the profit reachable with the weight left
                new_item.profit + profit_reachable_with_left_weight
            };
            log::info!(
                "New item id={}, weight={}, with current_weight_capacity={}, new_reachable_profit={}",
                new_item.id,
                new_item.weight,
                current_weight_capacity,
                new_reachable_profit
            );
            if new_item.weight > current_weight_capacity as u64 {
                // Item is too expensive / weights to much
                profit = *old_profit;
                log::debug!(
                    "New item in row={} at column={} is too expensive",
                    row,
                    column
                );
            } else if new_reachable_profit <= *old_profit {
                // Item brings no improvement
                profit = *old_profit;
                log::debug!(
                    "New item in row={} at column={} brings no improvement",
                    row,
                    column
                );
            } else {
                // We can afford the item and it brings improvement
                profit = new_reachable_profit;
                log::debug!(
                    "New item in row={} at index={} is affordable and brings improvement",
                    row,
                    column
                );
            }
            new_row.push(profit);
        }
        table.push(new_row);
    }
    table
}

/// Solves the decision problem [0-1-knapsack](https://en.wikipedia.org/wiki/Knapsack_problem)
/// via the (integer) [greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm).
pub fn knapsack_0_1(items: &[Item], weight_capacity: u64) -> Vec<&Item> {
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let mut items_sorted_asc: Vec<&Item> = items.iter().collect();
    items_sorted_asc.sort();

    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc.iter().map(|item| item.id).collect();
        log::debug!("Sorted item ids: {:?}", items_sorted_ids);
    }

    // Items that are selected for to be contained in the knapsack
    let mut knapsack: Vec<&Item> = Vec::new();

    for (item_index, new_item) in items_sorted_asc.iter().enumerate() {
        // Calculate already used weight, remaining available weight and the currently reached profit
        let used_knapsack_weight: u64 = knapsack.iter().map(|item| item.weight).sum();
        let available_knapsack_weight: u64 = weight_capacity - used_knapsack_weight;
        // let reached_knapsack_profit: u64 = knapsack.iter().map(|item| item.weight).sum();
        log::debug!(
            "round={:<2} current_id={:<2} available_weight={} used_weight={}",
            item_index,
            new_item.id,
            available_knapsack_weight,
            used_knapsack_weight
        );

        if available_knapsack_weight <= 0 {
            // The knapsack is full / reached its weight capacity. We can not put any more elements into it.
            break;
        }

        if available_knapsack_weight < new_item.weight {
            log::info!(
                "Item id={:<2} weights too much. item.weight={} > available_weight={}",
                new_item.id,
                new_item.weight,
                available_knapsack_weight
            );
            continue;
        }
        log::debug!("Taking item id={:<2}", new_item.id);
        knapsack.push(new_item);
    }
    knapsack
}

#[cfg(test)]
mod test {
    use super::*;

    static ITEMS: [Item; 16] = [
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
    ];

    #[test]
    fn test_fractional_knapsack() {
        let weight_capacity = 120;
        let actual_chosen_items = fractional_knapsack(&ITEMS, weight_capacity);
        let expected_chosen_items = vec![
            PackedItem {
                item: &ITEMS[5],
                take_fraction: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[3],
                take_fraction: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[12],
                take_fraction: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[11],
                take_fraction: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[2],
                take_fraction: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[8],
                take_fraction: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[14],
                take_fraction: Fraction::new(6u64, 10u64),
            },
        ];
        assert_eq!(actual_chosen_items, expected_chosen_items);
    }

    #[test]
    fn test_maximum_knapsack() {
        let max_knapsack_items = [
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
        let weight_capacity = 9;
        let actual_table = maximum_knapsack(&max_knapsack_items, weight_capacity);
        let expected_table = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 6, 6, 6, 6, 6, 6, 6, 6],
            [0, 0, 6, 6, 6, 11, 11, 11, 11, 11],
            [0, 0, 6, 6, 6, 11, 11, 11, 14, 14],
            [0, 0, 6, 6, 6, 11, 11, 11, 14, 15],
            [0, 0, 6, 6, 6, 11, 11, 12, 14, 15],
            [0, 0, 6, 6, 6, 11, 11, 12, 14, 15],
            [0, 0, 6, 6, 6, 11, 11, 12, 14, 15],
        ];
        assert_eq!(actual_table, expected_table);
    }

    #[test]
    fn test_knapsack_0_1() {
        let weight_capacity = 120;
        let actual_knapsack = knapsack_0_1(&ITEMS, weight_capacity);
        let expected_ids = [6, 4, 13, 12, 3, 9, 16];
        assert_eq!(
            actual_knapsack
                .iter()
                .map(|item| item.id)
                .collect::<Vec<_>>(),
            expected_ids
        );
    }
}
