//! Solving of various [knapsack problems](https://en.wikipedia.org/wiki/Knapsack_problem).
//!
//! From Wikipedia: "Given a set of items, each with a weight and a value, determine the number of each item to include
//! in a collection so that the total weight is less than or equal to a given limit and the total value is as large as
//! possible"

use std::borrow::Borrow;
use std::cmp::Ordering;
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
pub struct PartialPackedItem<'a, ItemRef>
where
    ItemRef: Borrow<Item>,
{
    /// The original item.
    pub item: &'a ItemRef,
    /// A fraction indicating how much of the item was put into the knapsack.
    pub take_portion: Fraction,
}

impl<'a, ItemRef> PartialPackedItem<'a, ItemRef>
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

// Allow items to be compared and sorted by their weight_profit_ration.
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// ------- Solving Algorithms ----------------------------------

/// Solves the [fractional knapsack problem](https://en.wikipedia.org/wiki/Continuous_knapsack_problem) by using the
/// [fractional greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm). The returned solution is optimal.
///
/// # Arguments
///
/// * `items` - Something that can be turned into an iterator yielding references to [Item]s or something that can be
/// borrowed as [Item].
/// * `weight_limit` - The maximum allowed weight of the knapsack.
///
/// # Returns
///
/// A list of [PartialPackedItem]s. They contain a fraction of how much of the item was put into the knapsack.
/// This is a value between 0 (exclusive) and 1 (inclusive). [Item]s that were not chosen are not contained in this list.
pub fn fractional_greedy<'a, ItemRef, ItemIter>(
    items: ItemIter,
    weight_limit: u64,
) -> Vec<PartialPackedItem<'a, ItemRef>>
where
    ItemRef: Borrow<Item>,
    ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let items_sorted_asc: Vec<&ItemRef> = {
        let mut items: Vec<&ItemRef> = Vec::from_iter(items);
        items.sort_by_key(|item| <ItemRef as Borrow<Item>>::borrow(item));
        items
    };

    // Log sorted item id's. Only do this computation when logging is enabled for this level.
    let item_ids_log_level = log::Level::Debug;
    if log::log_enabled!(item_ids_log_level) {
        let items_sorted_ids: Vec<usize> = items_sorted_asc
            .iter()
            .map(|item| item.deref().borrow().id)
            .collect();
        log::log!(
            item_ids_log_level,
            "Sorted item ids: {:?}",
            items_sorted_ids
        );
    }

    // Items that are selected to be contained in the knapsack
    let mut knapsack: Vec<PartialPackedItem<'a, ItemRef>> = Vec::new();

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
                available_knapsack_weight / Fraction::from(new_item.deref().borrow().weight);
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
            item_index, new_item.deref().borrow().id, take_fraction, available_knapsack_weight, used_knapsack_weight, reached_knapsack_profit);
    }
    knapsack
}

/// Solves the [maximum knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) with
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming). The returned solution is optimal.
///
/// # Arguments
///
/// * `items` - Something that can be turned into an iterator yielding references to [Item]s or something that can be
/// borrowed as [Item].
/// * `weight_limit` - The maximum allowed weight of the knapsack.
///
/// # Returns
///
/// The knapsack, i.e. all items that are chosen to be in the knapsack.
pub fn dynamic_programming<'a, ItemIter, ItemRef>(
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
                new_knapsack.push(item);
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

/// Solves the [maximum knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) with
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming). The returned solution may not be optimal!
///
/// # Arguments
///
/// * `items` - Something that can be turned into an iterator yielding references to [Item]s or something that can be
/// borrowed as [Item].
/// * `weight_limit` - The maximum allowed weight of the knapsack.
///
/// # Returns
///
/// The knapsack, i.e. all items that are chosen to be in the knapsack.
pub fn integer_greedy<'a, ItemRef, ItemIter>(
    items: ItemIter,
    weight_capacity: u64,
) -> Vec<&'a ItemRef>
where
    ItemRef: Borrow<Item>,
    ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let items_sorted_asc: Vec<&ItemRef> = {
        let mut items = Vec::from_iter(items);
        items.sort_by_key(|item| <ItemRef as Borrow<Item>>::borrow(item));
        items
    };

    // Log the sorted ids of the items. Only do this computation when logging is enabled for this level.
    let item_ids_log_level = log::Level::Debug;
    if log::log_enabled!(item_ids_log_level) {
        let items_sorted_ids: Vec<usize> = items_sorted_asc
            .iter()
            .map(|item| item.deref().borrow().id)
            .collect();
        log::log!(
            item_ids_log_level,
            "Sorted item ids: {:?}",
            items_sorted_ids
        );
    }

    // Items that are selected to be contained in the knapsack
    let mut knapsack: Vec<&ItemRef> = Vec::new();

    for (item_index, new_item) in items_sorted_asc.iter().enumerate() {
        // Calculate already used weight, remaining available weight and the currently reached profit
        let used_knapsack_weight: u64 = knapsack
            .iter()
            .map(|item| item.deref().borrow().weight)
            .sum();
        let available_knapsack_weight: u64 = weight_capacity - used_knapsack_weight;
        log::debug!(
            "round={:<2} current_id={:<2} available_weight={} used_weight={}",
            item_index,
            new_item.deref().borrow().id,
            available_knapsack_weight,
            used_knapsack_weight
        );

        if available_knapsack_weight == 0 {
            // The knapsack is full / reached its weight capacity. We can not put any more elements into it.
            break;
        }

        if available_knapsack_weight < new_item.deref().borrow().weight {
            // Item weights too much
            log::debug!(
                "Item id={:<2} weights too much. item.weight={} > available_weight={}",
                new_item.deref().borrow().id,
                new_item.deref().borrow().weight,
                available_knapsack_weight
            );
            continue;
        }
        // Item fits in knapsack, so put item into the knapsack
        log::debug!("Taking item id={:<2}", new_item.deref().borrow().id);
        knapsack.push(new_item);
    }
    knapsack
}

/// Solves the [maximum knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) with
/// [integer greedy algorithm](https://en.wikipedia.org/wiki/Dynamic_programming).
/// This is a heuristic algorithm, so the returned solution may not be optimal.
///
/// # Arguments
///
/// * `items` - Something that can be turned into an iterator yielding references to [Item]s or something that can be
/// borrowed as [Item]. The trick is that this is a reference, so that this function is able to iterate over items
/// multiple times.
/// * `weight_limit` - The maximum allowed weight of the knapsack.
/// * `k` - How many items should be fixed brute-forced like before running a integer greedy.
///
/// # Returns
///
/// The knapsack, i.e. all items that are chosen to be in the knapsack.
pub fn greedy_k<'a, ItemRef, ItemIter>(
    items: &'a ItemIter,
    weight_limit: u64,
    k: usize,
) -> Vec<&'a ItemRef>
where
    ItemRef: 'a + Borrow<Item>,
    &'a ItemRef: Borrow<Item>,
    &'a ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    (0..=k)
        // Get all combinations with 0 elements fixed, 1 element fixed, 2 elements fixed, ..., k elements fixed
        .map(|k_| Itertools::combinations(items.into_iter(), k_))
        .flatten()
        // Remove combinations with too much weight
        .filter(|fixed_items| {
            let total_weight: u64 = fixed_items
                .iter()
                .map(|item| item.deref().borrow().weight)
                .sum();
            total_weight < weight_limit
        })
        // Perform a normal integer greedy on the remaining items
        .map(|fixed_items| {
            // Get all items that are not already included in fixed_items
            let remaining_items = items.into_iter().filter(|&item| {
                fixed_items
                    .iter()
                    .any(|&fixed_item| fixed_item.borrow() == item.borrow())
                    .not()
            });
            let fixed_items_weight: u64 =
                fixed_items.iter().map(|&item| item.borrow().weight).sum();
            let remaining_weight_limit = weight_limit - fixed_items_weight;
            let remaining_greedy = integer_greedy(remaining_items, remaining_weight_limit);
            let knapsack = {
                let mut knapsack = remaining_greedy;
                knapsack.extend_from_slice(fixed_items.as_slice());
                knapsack
            };
            log::info!(
                "Knapsack={:?}, fixed={:?}, fixed_weight={}, remaining_weight={}",
                knapsack
                    .iter()
                    .map(|item| item.borrow().id)
                    .collect::<Vec<usize>>(),
                fixed_items
                    .iter()
                    .map(|item| item.borrow().id)
                    .collect::<Vec<usize>>(),
                fixed_items_weight,
                remaining_weight_limit
            );
            knapsack
        })
        // Get the best knapsack, i.e. the selection with the most profit
        .max_by_key(|items| items.iter().map(|&item| item.borrow().profit).sum::<u64>())
        // Get either the result or an empty vec
        .unwrap_or_default()
}

/// Solves the [maximum knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) with
/// [branch and bound](https://en.wikipedia.org/wiki/Branch_and_bound).
/// The returned solution is optimal.
///
/// # Arguments
///
/// * items - List of objects that can be borrowed as an [Item]. It is assumed that the items are already sorted.
/// The trick is that this is a reference, so that this function is able to iterate over items multiple times.
/// * weight_limit - The currently remaining weight limit. This includes weight consumes by earlier decisions
///
/// # Returns
///
/// The knapsack, i.e. all items that are chosen to be in the knapsack.
pub fn branch_and_bound<'a, ItemRef, ItemIter>(
    items: &'a ItemIter,
    weight_limit: u64,
) -> Vec<&'a ItemRef>
where
    ItemRef: 'a + Borrow<Item>,
    &'a ItemIter: IntoIterator<Item = &'a ItemRef>,
{
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let items_sorted: Vec<&ItemRef> = {
        let mut items = Vec::from_iter(items);
        items.sort_by_key(|item| <ItemRef as Borrow<Item>>::borrow(item));
        items
    };

    branch_and_bound_recursive(&items_sorted, weight_limit, &[], &[])
}

/// This function recursively calls itself and performs the main logic of the branch and bound knapsack.
///
/// # Arguments
///
/// * items - List of objects that can be borrowed as an [Item]. [Items] which should be excluded may not be included in
/// this list. It is assumed that the items are already sorted. The trick is that this is a reference, so that this
/// function is able to iterate over items multiple times.
/// * weight_limit - The currently remaining weight limit. This includes weight consumes by earlier decisions
/// about whether items should be included or excluded.
/// * fixed_items - Items which are fixed, i.e. always included.
/// * best_profit - The currently best known profit.
///
/// # Returns
///
/// The knapsack, i.e. all items that are chosen to be in the knapsack.
fn branch_and_bound_recursive<'a, 'b, ItemRef>(
    items: &'b [&'a ItemRef],
    weight_limit: u64,
    fixed_items: &'b [&'a ItemRef],
    best_knapsack: &'b [&'a ItemRef],
) -> Vec<&'a ItemRef>
where
    ItemRef: Borrow<Item>,
{
    let mut best_knapsack: Vec<&ItemRef> = best_knapsack.to_vec();

    // First, calculate the lower bound. Then, update best_knapsack, if lower bound is an improvement
    let lower_bound_knapsack: Vec<&ItemRef> = {
        let mut lower_bound_knapsack = integer_greedy(items.iter().copied(), weight_limit);
        lower_bound_knapsack.extend(fixed_items);
        lower_bound_knapsack
    };
    let lower_bound_profit = knapsack_profit(&lower_bound_knapsack);
    if lower_bound_profit > knapsack_profit(&best_knapsack) {
        // Would lower_bound be an improvement? If yes, update it
        best_knapsack = lower_bound_knapsack;
    }

    // Secondly, calculate the upper bound
    let upper_bound_profit = {
        let packed_items = fractional_greedy(items.iter().copied(), weight_limit);
        let upper_bound_profit: Fraction = packed_items
            .into_iter()
            .map(|packed_item| packed_item.effective_profit())
            .sum();
        // Relaxation of upper bound: Round upper bound down, since integer knapsack can never reach a decimal profit.
        let upper_bound_profit = fraction_to_u64(upper_bound_profit);
        upper_bound_profit + knapsack_profit(fixed_items)
    };

    log::info!(
        "lower_bound={} upper_bound={} current_best={}",
        lower_bound_profit,
        upper_bound_profit,
        knapsack_profit(&best_knapsack)
    );

    // Is it worth it to analyse the subtree?
    if upper_bound_profit <= knapsack_profit(&best_knapsack) {
        // Skip subtree because it can not be better than best_profit
        log::info!(
            "Skipping subtree because upper_bound={} <= best_profit={}",
            upper_bound_profit,
            knapsack_profit(&best_knapsack)
        );
        return best_knapsack;
    }

    let (first, tail) = match items.split_first() {
        Some(x) => x,
        // We are at a leaf in the enumeration tree. No profit can be reached
        None => return Vec::new(),
    };

    // Calculate the reachable profit if we exclude the first item
    log::info!("Exclude item id={}", first.deref().borrow().id);
    let knapsack_exclude_first =
        branch_and_bound_recursive(tail, weight_limit, fixed_items, &best_knapsack);
    // Update best_knapsack if a better knapsack was found in the excluding subtree
    if knapsack_profit(&knapsack_exclude_first) > knapsack_profit(&best_knapsack) {
        best_knapsack = knapsack_exclude_first;
    }

    // Calculate the reachable profit if we include the first item
    log::info!("Include item id={}", first.deref().borrow().id);
    let knapsack_include_first = if weight_limit >= first.deref().borrow().weight {
        // weight_limit - first.weight is greater or equal 0
        let fixed_items_with_first = {
            let mut fixed_items_with_first = fixed_items.to_vec();
            fixed_items_with_first.push(first);
            fixed_items_with_first
        };
        branch_and_bound_recursive(
            tail,
            weight_limit - first.deref().borrow().weight,
            &fixed_items_with_first,
            &best_knapsack,
        )
    } else {
        // weight_limit would be negative, which is not allowed
        log::info!("weight_limit would be negative");
        Vec::new()
    };
    // Update best_knapsack if a better knapsack was found in the including subtrees
    if knapsack_profit(&knapsack_include_first) > knapsack_profit(&best_knapsack) {
        best_knapsack = knapsack_include_first;
    }

    best_knapsack
}

/// Calculates the total profit of all items.
pub fn knapsack_profit<ItemRef>(items: &[&ItemRef]) -> u64
where
    ItemRef: Borrow<Item>,
{
    items.iter().map(|&item| item.borrow().profit).sum()
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
