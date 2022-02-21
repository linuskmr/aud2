//! Solving of various [knapsack problems](https://en.wikipedia.org/wiki/Knapsack_problem).
//!
//! From Wikipedia: "Given a set of items, each with a weight and a value, determine the number of each item to include
//! in a collection so that the total weight is less than or equal to a given limit and the total value is as large as
//! possible"

use std::borrow::Borrow;
use std::cmp::{max, Ordering};
use std::fmt;
use std::ops::{Deref, Not};

use fraction::Fraction;
use itertools::Itertools;
use log::log_enabled;
use serde::Deserialize;

// ------- Item ----------------------------------

/// An item is an object that has a profit and weight. An item can be put into a knapsack, which causes the item to be
/// wrapped in an [PartialPackedItem].
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
    /// Calculates `weight / profit`. This is an indicator how much value an item has. The lower the ratio, the better
    /// it is. A low ratio means much profit at low weight. A high ratio means low profit at high weight.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fraction::Fraction;
    /// # use aud2::knapsack::Item;
    /// let item = Item {
    ///     id: 0,
    ///     profit: 5,
    ///     weight: 2
    /// };
    /// assert_eq!(item.weight_profit_ratio(), Fraction::new(2u64, 5u64));
    /// ```
    pub fn weight_profit_ratio(&self) -> Fraction {
        Fraction::new(self.weight, self.profit)
    }
}

// Include the weight_profit_ratio in the debug output.
impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item")
            .field("id", &self.id)
            .field("weight", &self.weight)
            .field("profit", &self.profit)
            .field(
                "weight_profit_ratio",
                &format!("{:.4}", self.weight_profit_ratio()),
            )
            .finish()
    }
}

// ------- PartialPackedItem ----------------------------------

/// An [Item] that was put inside a knapsack, storing how much of the item was put into the knapsack.
#[derive(Debug, PartialEq, Clone)]
pub struct PartialPackedItem<ItemRef>
where
    ItemRef: Borrow<Item>,
{
    /// The original item.
    pub item: ItemRef,
    /// A fraction indicating how much of the item was put into the knapsack.
    pub take_portion: Fraction,
}

impl<ItemRef> PartialPackedItem<ItemRef>
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
///
/// # Arguments
/// * `items` - Something that can be turned into an iterator yielding references to [Item]s or something that can be
/// borrowed as [Item].
/// * `weight_limit`: The maximum allowed weight of the knapsack.
///
/// # Returns
///
/// A list of [PackedItem]s. They contain a fraction of how much of the item was put into the knapsack. This is a value
/// between 0 (exclusive) and 1 (inclusive). [Item]s that were not chosen are not contained in this list.
pub fn fractional_knapsack_greedy<'a, ItemRef, ItemIter>(
    items: ItemIter,
    weight_limit: u64,
) -> Vec<PartialPackedItem<&'a ItemRef>>
where
    ItemRef: Borrow<Item>,
    &'a ItemRef: Borrow<Item>,
    ItemIter: IntoIterator<Item = &'a ItemRef>,
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
    let mut knapsack: Vec<PartialPackedItem<&ItemRef>> = Vec::new();

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
        let knapsack_item = PartialPackedItem {
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
pub fn knapsack_dynamic_programming<'a, ItemIter, ItemRef>(
    items: ItemIter,
    weight_capacity: u64,
) -> Vec<&'a ItemRef>
where
    ItemIter: IntoIterator<Item = &'a ItemRef>,
    ItemRef: Borrow<Item>,
{
    // Row stores the current row. Each cell contains a list of items, which are included in the knapsack. This takes
    // the first item_nr items into account and the knapsack limited by the weight specified by the index of the cell.
    let mut row: Vec<Vec<&ItemRef>> = vec![Vec::new(); (weight_capacity + 1) as usize];

    // Print the weight limits for each cell, which is just its index
    log::debug!("weight_limits={:?}", (0..row.len()).collect_vec());

    // Examine which profits are producible by using a new item from the item list.
    for (item_nr, item) in items.into_iter().enumerate() {
        // Create the new row by inspecting the old one and inspect if improvement can be made by using the new item.
        // Because we override the old row, we go from right ro left.
        for index in (0..row.len()).rev() {
            let current_weight_limit = index;
            /*log::trace!(
                "New item id={}, weight={}, profit={} with weight_limit={}",
                item.borrow().id,
                item.borrow().weight,
                item.borrow().profit,
                current_weight_limit,
            );*/

            // Can we afford the item?
            if item.borrow().weight > current_weight_limit as u64 {
                // Item is too expensive / weights to much
                log::debug!(
                    "Item id={} with weight={} at index={} is too expensive for weight_limit={}",
                    item.borrow().id,
                    item.borrow().weight,
                    index,
                    current_weight_limit
                );
                continue;
            }

            // If we would take item, how much profit would be reachable with it?

            // How much weight would be free, if we use the new item?
            let remaining_weight =
                current_weight_limit.saturating_sub(item.borrow().weight as usize);
            assert!(
                remaining_weight < row.len(),
                "More weight reaming than slots allocated in the row. This is a logic error"
            );
            // What profit can be reached with the remaining weight?
            let other_items = &row[remaining_weight];
            let additional_profit: u64 = other_items
                .iter()
                .map(|item| item.deref().borrow().profit)
                .sum();

            // As result, we can get the profit of the new item + the profit reachable with the weight left
            let new_profit = item.borrow().profit + additional_profit;

            // Calculate old profit, to see whether the new profit is better
            let old_profit: u64 = row[index]
                .iter()
                .map(|item| item.deref().borrow().profit)
                .sum();

            if new_profit <= old_profit {
                // Item brings no improvement
                log::debug!(
                    "Item id={} at index={} would bring profit={}. This is no improvement to old profit={}",
                    item.borrow().id,
                    index,
                    new_profit,
                    old_profit
                );
                continue;
            }

            // We can afford the item and it brings improvement
            log::debug!(
                "Item id={} at index={} is affordable and brings improvement. Profit={} instead of old profit={}",
                item.borrow().id,
                index,
                new_profit,
                old_profit
            );
            row[index] = {
                let mut new_knapsack = other_items.clone();
                new_knapsack.push(&item);
                new_knapsack
            };
        }

        // Print the profit for each cell. Only do this computation when logging is enabled for this level.
        let profits_log_level = log::Level::Info;
        if log_enabled!(profits_log_level) {
            // Sum the profit of each knapsack
            let row_profits: Vec<u64> = row
                .iter()
                .map(|knapsack| {
                    knapsack
                        .iter()
                        .map(|item| item.deref().borrow().profit)
                        .sum::<u64>()
                })
                .collect();
            log::log!(profits_log_level, "Row i={}: {:?}", item_nr, row_profits);
        }
    }
    row.pop().unwrap_or_default()
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

/*
pub fn greedy_k<ItemRef, ItemIter>(items: ItemIter, weight_limit: u64, k: usize)
where
    ItemRef: Borrow<Item> + Clone + PartialEq,
    ItemIter: IntoIterator<Item = ItemRef>,
{
    let items = Vec::from_iter(items);

    // Get all combinations with k elements and fix and include them
    let knapsack = Itertools::combinations(items.iter(), k)
        // Remove combinations with too much weight
        .filter(|fixed_items| {
            let weight: u64 = fixed_items
                .iter()
                .map(|item| item.deref().borrow().weight)
                .sum();
            weight < weight_limit
        })
        // Do a normal integer greedy with the remaining items
        .map(|fixed_items| {
            let remaining_items = items.iter().filter(|item| fixed_items.contains(item).not());
            let remaining_weight: u64 = fixed_items
                .iter()
                .map(|item| item.deref().borrow().weight)
                .sum();
            let remaining_items_vec: Vec<&ItemRef> = remaining_items.collect();
            let remaining_greedy = knapsack_integer_greedy(remaining_items_vec, remaining_weight);
            let mut knapsack = remaining_greedy;
            knapsack.extend_from_slice(fixed_items.as_slice());
            knapsack
        })
        // Get the best knapsack, i.e. the selection with the most profit
        .max_by_key(|items| {
            items
                .iter()
                .map(|item| item.deref().borrow().profit)
                .sum::<u64>()
        })
        // Get either the result or an empty vec
        .unwrap_or_default();

    println!("{:#?}", knapsack);
}
*/

/* pub fn knapsack_branch_and_bound<'a, ItemRef, ItemIter>(items: ItemIter, weight_limit: u64) -> u64
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
    ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    let mut best_profit = best_profit;
    let items = Vec::from_iter(items);

    let lower_bound: u64 = current_profit
        + items_profit_sum(knapsack_integer_greedy(
            items.iter().map(|&item| item),
            weight_limit,
        ));
    // Profit improved? If yes, set it
    best_profit = max(best_profit, lower_bound);

    // Relaxation of upper bound: Integer knapsack can never reach a decimal profit, so we can round it down
    let upper_bound: u64 = {
        let packed_items = fractional_knapsack_greedy(items, weight_limit);
        let packed_items_profit = packed_items_profit_sum(&packed_items);
        let packed_items_profit = fraction_to_u64(&packed_items_profit);
        current_profit + packed_items_profit
    };

    if upper_bound > best_profit {
        let (first, tail) = match items.split_first() {
            Some(x) => x,
            // No items, so no profit can be reached
            None => return 0,
        };

        let profit_exclude_first = knapsack_branch_and_bound_recursive(
            tail.into_iter().map(|item| *item), // items
            weight_limit,                       // weight_limit
            current_profit,                     // current_profit
            best_profit,                        // best_profit
        );

        let profit_include_first = knapsack_branch_and_bound_recursive(
            tail.into_iter().map(|item| *item),     // items
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
}*/

/// Calculates the total profit of all items.
fn items_profit_sum<'a, ItemRef, Iter>(items: Iter) -> u64
where
    ItemRef: 'a, // The items yielded by the iterator may life as long as this function
    &'a ItemRef: Borrow<Item>, // The item types yielded by the iterator may be borrowed as items
    Iter: IntoIterator<Item = &'a ItemRef>, // The iterator yields references to ItemRefs
{
    items.into_iter().map(|item| (&item).borrow().weight).sum()
}

/// Calculates the total effective profit of all items.
fn packed_items_profit_sum<'a, IterItem, Iter>(items: Iter) -> Fraction
where
    IterItem: 'a,
    &'a IterItem: Borrow<Item>,
    Iter: IntoIterator<Item = &'a PartialPackedItem<&'a IterItem>>,
{
    items
        .into_iter()
        .map(|item| item.borrow().effective_profit())
        .sum()
}

/// Converts a [Fraction] into a u64 by removing the digits after the dot and parsing its string representation.
///
/// # Examples
///
/// ```
/// # use fraction::Fraction;
/// # use aud2::knapsack::fraction_to_u64;
/// assert_eq!(fraction_to_u64(Fraction::from(1)), 1);
/// assert_eq!(fraction_to_u64(Fraction::from(4.2)), 4);
/// assert_eq!(fraction_to_u64(Fraction::from(2.9)), 2);
/// ```
pub fn fraction_to_u64(fraction: impl Borrow<Fraction>) -> u64 {
    format!("{:.0}", fraction.borrow())
        .parse()
        .expect("Parsing fraction with 0 zero digits after the dot always succeeds")
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
            PartialPackedItem {
                item: &ITEMS[5],
                take_portion: 1.0.into(),
            },
            PartialPackedItem {
                item: &ITEMS[3],
                take_portion: 1.0.into(),
            },
            PartialPackedItem {
                item: &ITEMS[12],
                take_portion: 1.0.into(),
            },
            PartialPackedItem {
                item: &ITEMS[11],
                take_portion: 1.0.into(),
            },
            PartialPackedItem {
                item: &ITEMS[2],
                take_portion: 1.0.into(),
            },
            PartialPackedItem {
                item: &ITEMS[8],
                take_portion: 1.0.into(),
            },
            PartialPackedItem {
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
                id: 0,
                profit: 6,
                weight: 2,
            },
            Item {
                id: 1,
                profit: 5,
                weight: 3,
            },
            Item {
                id: 2,
                profit: 8,
                weight: 6,
            },
            Item {
                id: 3,
                profit: 9,
                weight: 7,
            },
            Item {
                id: 4,
                profit: 6,
                weight: 5,
            },
            Item {
                id: 5,
                profit: 7,
                weight: 9,
            },
            Item {
                id: 6,
                profit: 3,
                weight: 4,
            },
        ];
        let weight_limit = 9;
        let actual_knapsack = knapsack_dynamic_programming(&max_knapsack_items, weight_limit);
        assert!(
            actual_knapsack.iter().map(|item| item.weight).sum::<u64>() <= weight_limit,
            "Knapsack solution too heavy"
        );
        let expected_knapsack = [&max_knapsack_items[0], &max_knapsack_items[3]];
        assert_eq!(
            actual_knapsack, expected_knapsack,
            "Algorithm chose the wrong items"
        );
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
