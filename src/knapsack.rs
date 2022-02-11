//! Solving of various [knapsack problems](https://en.wikipedia.org/wiki/Knapsack_problem).
//!
//! From Wikipedia: "Given a set of items, each with a weight and a value, determine the number of each item to include
//! in a collection so that the total weight is less than or equal to a given limit and the total value is as large as
//! possible"

use std::borrow::Borrow;
use std::cmp::{max, Ordering};
use std::fmt;

use fraction::Fraction;
use serde::Deserialize;

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
pub struct PackedItem<TakePortion, ItemRef>
where
    ItemRef: Borrow<Item>,
{
    /// The original item.
    pub item: ItemRef,
    /// A number indicating how much of the item or how many items were put into the knapsack.
    pub take_portion: TakePortion,
}

// Partial packed items
impl<ItemRef> PackedItem<Fraction, ItemRef>
where
    ItemRef: Borrow<Item>,
{
    /// Calculates the weight this item weights considering its take_fraction, i.e. partial packed items.
    pub fn effective_weight(&self) -> Fraction {
        Fraction::from(self.item.borrow().weight) * self.take_portion
    }

    /// Calculates the profit this items gives considering its take_fraction, i.e partial packed items.
    pub fn effective_profit(&self) -> Fraction {
        Fraction::from(self.item.borrow().profit) * self.take_portion
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

/// Helper function making it possible to sort a Vec<Item> with .sort_by(cmp_items).
/// Although the f64 returned by self.weight_profit_ratio() does not implement Ord, this is needed to be able to sort a
/// collection. So we simply assert that ordering the f64 is always possible, i.e. NaN and infinity are not allowed as
/// weight_profit_ratio.
fn cmp_items<ItemRef: Borrow<Item>>(item_a: &ItemRef, item_b: &ItemRef) -> Ordering {
    let item_a_ration = item_a.borrow().weight_profit_ratio();
    let item_b_ration = item_b.borrow().weight_profit_ratio();
    item_a_ration
        .partial_cmp(&item_b_ration)
        .expect("Illegal values in item which make it unable to be sorted")
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Illegal values in item which make it unable to be sorted")
    }
}

// ------- Solving Algorithms ----------------------------------

/// Solves the [fractional knapsack problem](https://en.wikipedia.org/wiki/Continuous_knapsack_problem) by using the
/// [greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm).
pub fn fractional_knapsack_greedy<'a, ItemRef>(
    items: &'a [ItemRef],
    weight_limit: u64,
) -> Vec<PackedItem<Fraction, &'a ItemRef>>
where
    &'a ItemRef: Borrow<Item>,
{
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let items_sorted_asc: Vec<&ItemRef> = {
        let mut items: Vec<&ItemRef> = Vec::from_iter(items);
        items.sort_by(cmp_items);
        items
    };

    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc
            .iter()
            .map(|item| item.borrow().id)
            .collect();
        log::debug!("Sorted item ids: {:?}", items_sorted_ids);
    }

    // Items that are selected for to be contained in the knapsack
    let mut knapsack: Vec<PackedItem<Fraction, &ItemRef>> = Vec::new();

    for (item_index, new_item) in items_sorted_asc.iter().enumerate() {
        // Calculate already used weight, remaining available weight and the currently reached profit
        let used_knapsack_weight: Fraction = knapsack
            .iter()
            .map(|packed_item| packed_item.effective_weight())
            .sum();
        let available_knapsack_weight: Fraction =
            Fraction::from(weight_limit) - used_knapsack_weight;
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
            let take_fraction =
                available_knapsack_weight / Fraction::from(new_item.borrow().weight);
            if take_fraction > Fraction::from(1) {
                Fraction::from(1)
            } else {
                take_fraction
            }
        };
        // Add item to knapsack
        let knapsack_item = PackedItem {
            item: *new_item,
            take_portion: take_fraction,
        };
        knapsack.push(knapsack_item);

        log::debug!("round={:<2} current_id={:<2} take_fraction={} available_capacity={:<3} used_capacity={:<3} effective_profit={:<2}",
            item_index, new_item.borrow().id, take_fraction, available_knapsack_weight, used_knapsack_weight, reached_knapsack_profit);
    }
    knapsack
}

/// Solves the [maximum knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) with
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn knapsack_dynamic_programming<I>(items: &[I], weight_capacity: u64) -> Vec<Vec<u64>>
where
    I: Borrow<Item>,
{
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
                    current_weight_capacity.saturating_sub(new_item.borrow().weight as usize);
                // What profit can be reached with the capacity left?
                let profit_reachable_with_left_weight = last_row[free_weight_capacity];
                // In the end, we can get the profit of the new item + the profit reachable with the weight left
                new_item.borrow().profit + profit_reachable_with_left_weight
            };
            log::info!(
                "New item id={}, weight={}, with current_weight_capacity={}, new_reachable_profit={}",
                new_item.borrow().id,
                new_item.borrow().weight,
                current_weight_capacity,
                new_reachable_profit
            );
            if new_item.borrow().weight > current_weight_capacity as u64 {
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
/// This function returns the knapsack containing all chosen items. This solution may not be optimal!
pub fn knapsack_integer_greedy<'a, ItemRef, ItemIter>(
    items: ItemIter,
    weight_capacity: u64,
) -> Vec<&'a ItemRef>
where
    &'a ItemRef: Borrow<Item>,
    ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let items_sorted_asc: Vec<&ItemRef> = {
        let mut items = Vec::from_iter(items);
        items.sort_by(cmp_items);
        items
    };

    // Log the sorted ids of the items
    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc
            .iter()
            .map(|item| item.borrow().id)
            .collect();
        log::debug!("Sorted item ids: {:?}", items_sorted_ids);
    }

    // Items that are selected to be contained in the knapsack
    let mut knapsack: Vec<&ItemRef> = Vec::new();

    for (item_index, new_item) in items_sorted_asc.iter().enumerate() {
        // Calculate already used weight, remaining available weight and the currently reached profit
        let used_knapsack_weight: u64 = knapsack.iter().map(|item| item.borrow().weight).sum();
        let available_knapsack_weight: u64 = weight_capacity - used_knapsack_weight;
        log::debug!(
            "round={:<2} current_id={:<2} available_weight={} used_weight={}",
            item_index,
            new_item.borrow().id,
            available_knapsack_weight,
            used_knapsack_weight
        );

        if available_knapsack_weight <= 0 {
            // The knapsack is full / reached its weight capacity. We can not put any more elements into it.
            break;
        }

        if available_knapsack_weight < new_item.borrow().weight {
            // Item weights too much
            log::info!(
                "Item id={:<2} weights too much. item.weight={} > available_weight={}",
                new_item.borrow().id,
                new_item.borrow().weight,
                available_knapsack_weight
            );
            continue;
        }
        // Item fits in knapsack, so out item into the knapsack
        log::debug!("Taking item id={:<2}", new_item.borrow().id);
        knapsack.push(new_item);
    }
    knapsack
}

pub fn knapsack_branch_and_bound<'a, ItemRef, ItemIter>(items: ItemIter, weight_limit: u64) -> u64
where
    ItemRef: 'a,
    &'a ItemRef: Borrow<Item>,
    ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let items_sorted = {
        let mut items = Vec::from_iter(items);
        items.sort_by(cmp_items);
        items
    };

    knapsack_branch_and_bound_recursive(
        items_sorted,
        weight_limit,
        0, // current_profit
        0, // best_profit
    )
}

/// This function recursively calls itself and performs the main logic of the branch and bound knapsack.
///
/// # Arguments
///
/// * items - List of objects that can be borrowed as an [Item]. It is assumed that the items are already sorted.
/// * weight_limit - The currently remaining weight limit. This includes weight consumes by earlier decisions
/// about whether items should be included or excluded.
/// * current_profit - The reached profit from earlier decisions whether items should be included or excluded.
/// * best_profit - The currently best known profit.
fn knapsack_branch_and_bound_recursive<'a, ItemRef, ItemIter>(
    items: ItemIter,
    weight_limit: u64,
    current_profit: u64,
    best_profit: u64,
) -> u64
where
    ItemRef: 'a,
    &'a ItemRef: Borrow<Item>,
    ItemIter: 'a,
    &'a ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    let mut best_profit = best_profit;

    let lower_bound =
        current_profit + items_profit_sum(knapsack_integer_greedy(&items, weight_limit));
    // Profit improved? If yes, set it
    best_profit = max(best_profit, lower_bound);

    // Relaxation of upper bound: Integer knapsack can never reach a decimal profit, so we can round it down
    let upper_bound = current_profit
        + (packed_items_profit_sum(fractional_knapsack_greedy(&items, weight_limit)) as u64);

    if upper_bound > best_profit {
        let (first, tail) = match items.split_first() {
            Some(x) => x,
            // No items, so no profit can be reached
            None => return 0,
        };

        let profit_exclude_first = knapsack_branch_and_bound_recursive(
            tail,           // items
            weight_limit,   // weight_limit
            current_profit, // current_profit
            best_profit,    // best_profit
        );

        let profit_include_first = knapsack_branch_and_bound_recursive(
            tail,                                   // items
            weight_limit - first.borrow().weight,   // weight_limit
            current_profit + first.borrow().profit, // current_profit
            best_profit,                            // best_profit
        );

        best_profit = max(best_profit, max(profit_exclude_first, profit_include_first));
    }
    /*
    // Include the first item.
    // The weight capacity must therefore be decreased by the amount of the first item. The lower and upper bounds
    // returned from the functions have to be added to the profit of the first fixed, included item.
    log::debug!("Including item with id={}", first.borrow().id);
    let profit_include_first = {
        let remaining_weight_capacity = weight_limit - first.borrow().weight;
        log::trace!("remaining_weight_capacity={}", remaining_weight_capacity);
        let lower_bound = current_profit
            + first.borrow().profit
            + items_profit_sum(knapsack_0_1(tail, remaining_weight_capacity));
        // Round down upper bounds, i.e. floor, because we only can reach integer numbers
        let upper_bound = (current_profit
            + first.borrow().profit
            + packed_items_profit_sum(fractional_knapsack(tail, remaining_weight_capacity)))
            as u64;
        log::debug!("lower_bound={} upper_bound={}", lower_bound, upper_bound);

        assert!(
            lower_bound <= upper_bound,
            "Lower bound is larger than upper bound. This is not good..."
        );

        if upper_bound < current_profit {
            // This subtree can not get better as our current maximum. It is not worth analyzing it further.
            log::debug!("This subtree can not get better as ")
            return 0;
        } else if upper_bound == lower_bound {
            // The lower bound is already the best solution, so we can omit analyzing the subtree further.
            return lower_bound;
        } else {
            max(lower_bound, branch_and_bound_recursive(tail, remaining_weight_capacity))
        }
    };

    // Exclude the first item.
    // The weight capacity therefore does not get decreased, but the lower and upper bounds also not added with the
    // profit of the the first item, since it is excluded.
    let profit_exclude_first = {
        let lower_bound = items_profit_sum(knapsack_0_1(tail, weight_limit));
        // Round down upper bounds, i.e. floor, because we only can reach integer numbers
        let upper_bound =
            packed_items_profit_sum(fractional_knapsack(tail, weight_limit)) as u64;
    };*/
    best_profit
}

/// Calculates the total profit of all items.
fn items_profit_sum<'a, ItemRef, Iter>(items: Iter) -> u64
where
    ItemRef: 'a, // The items yielded by the iterator may life as long as this function
    ItemRef: Borrow<Item>, // The item types yielded by the iterator may be borrowed as items
    Iter: IntoIterator<Item = &'a ItemRef>, // The iterator yields references to ItemRefs
{
    items.into_iter().map(|item| item.borrow().weight).sum()
}

/// Calculates the total effective profit of all items.
fn packed_items_profit_sum<'a, IterItem, Iter>(items: Iter) -> Fraction
where
    IterItem: 'a,
    IterItem: Borrow<Item>,
    Iter: IntoIterator<Item = &'a PackedItem<Fraction, IterItem>>,
{
    items
        .into_iter()
        .map(|item| item.borrow().effective_profit())
        .sum()
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
        let actual_chosen_items = fractional_knapsack_greedy(&ITEMS, weight_capacity);
        let expected_chosen_items = vec![
            PackedItem {
                item: &ITEMS[5],
                take_portion: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[3],
                take_portion: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[12],
                take_portion: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[11],
                take_portion: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[2],
                take_portion: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[8],
                take_portion: 1.0.into(),
            },
            PackedItem {
                item: &ITEMS[14],
                take_portion: Fraction::new(6u64, 10u64),
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
        let actual_table = knapsack_dynamic_programming(&max_knapsack_items, weight_capacity);
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
        let actual_knapsack = knapsack_integer_greedy(&ITEMS, weight_capacity);
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
