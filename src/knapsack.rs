use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;

// ------- Knapsack ----------------------------------

/// Knapsack is a backpack with limited size or weight capacity.
#[derive(Default, PartialEq)]
pub struct Knapsack<'a>(Vec<KnapsackItem<'a>>);

impl<'a> Knapsack<'a> {
    /// Create a new empty knapsack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a new item into the knapsack.
    pub fn insert(&mut self, item: KnapsackItem<'a>) {
        self.0.push(item);
    }
}

// Forward fmt::Debug to the underlying vector.
impl fmt::Debug for Knapsack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// Allow Vec<KnapsackItem> to be converted into a knapsack.
impl<'a> From<Vec<KnapsackItem<'a>>> for Knapsack<'a> {
    fn from(knapsack_vec: Vec<KnapsackItem<'a>>) -> Self {
        Self(knapsack_vec)
    }
}

// ------- Item ----------------------------------

/// An item is an object that has a profit and weight. An item can be put into a knapsack, which caused the item to be
/// wrapped in an [KnapsackItem].
#[derive(Eq, PartialEq, Clone)]
pub struct Item {
    /// An unique identifier.
    pub id: usize,
    /// How much benefit / value this item provides.
    pub profit: u64,
    /// How much weight / size this item takes up.
    pub weight: u64,
}

impl Item {
    /// Calculates weight / profit. This is an indicator how much value an item has. The lower the ratio, the better it is.
    /// A low ratio means much profit at low weight. A high ratio means low profit at high weight.
    fn weight_profit_ratio(&self) -> f64 {
        (self.weight as f64) / (self.profit as f64)
    }
}

// ------- KnapsackItem ----------------------------------

/// A item that was put inside a [Knapsack].
#[derive(Debug, PartialEq, Clone)]
pub struct KnapsackItem<'a> {
    /// The original item.
    pub item: &'a Item,
    /// A number between 0.0 and 1.0 indicating how much of the item was put into the knapsack.
    pub take_fraction: f64,
}

impl<'a> KnapsackItem<'a> {
    /// Calculates the weight this item weights considering its take_fraction, i.e. partial packed items.
    pub fn effective_weight(&self) -> f64 {
        (self.item.weight as f64) * self.take_fraction
    }

    /// Calculates the profit this items gives considering its take_fraction, i.e partial packed items.
    pub fn effective_profit(&self) -> f64 {
        (self.item.profit as f64) * self.take_fraction
    }
}

// Allow knapsacks to be iterated over by forwarding the iterator implementation to the underlying vec.
impl<'a> IntoIterator for &'a Knapsack<'a> {
    type Item = &'a KnapsackItem<'a>;
    type IntoIter = core::slice::Iter<'a, KnapsackItem<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
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
pub fn fractional_knapsack(items: &[Item], weight_capacity: u64) -> Knapsack {
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let mut items_sorted_asc: Vec<&Item> = items.iter().collect();
    items_sorted_asc.sort();

    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc.iter().map(|item| item.id).collect();
        log::debug!("Sorted item ids: {:?}", items_sorted_ids);
    }

    // Items that are selected for to be contained in the knapsack
    let mut knapsack = Knapsack::new();

    for (index, item) in items_sorted_asc.iter().enumerate() {
        // Calculate already used weight, ringing available weight and the currently reached profit
        let used_knapsack_weight: f64 = knapsack
            .borrow()
            .into_iter()
            .map(|chosen_item| chosen_item.effective_weight())
            .sum();
        let available_knapsack_weight: f64 = (weight_capacity as f64) - used_knapsack_weight;
        let reached_knapsack_profit: f64 = knapsack
            .borrow()
            .into_iter()
            .map(|chosen_item| chosen_item.effective_profit())
            .sum();

        if available_knapsack_weight <= 0.0 {
            // The knapsack is full / reached its weight capacity. We can not put any more elements into it.
            break;
        }

        // How much of the element do we want to take? Maximum 100% or less, if there is not enough space for the entire
        // item.
        let take_fraction = ((available_knapsack_weight as f64) / (item.weight as f64)).min(1.0);
        // Add item to knapsack
        let knapsack_item = KnapsackItem {
            item,
            take_fraction,
        };
        knapsack.insert(knapsack_item);

        log::debug!("round={:<2} current_id={:<2} take_fraction={:<3} available_capacity={:<3} used_capacity={:<3} effective_profit={:<2}",
				 index, item.id, take_fraction, available_knapsack_weight, used_knapsack_weight, reached_knapsack_profit);
    }

    knapsack
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fractional_knapsack() {
        let items = [
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

        let weight_capacity = 120;
        let actual_chosen_items = fractional_knapsack(&items, weight_capacity);
        let expected_chosen_items = Knapsack::from(vec![
            KnapsackItem {
                item: &items[5],
                take_fraction: 1.0,
            },
            KnapsackItem {
                item: &items[3],
                take_fraction: 1.0,
            },
            KnapsackItem {
                item: &items[12],
                take_fraction: 1.0,
            },
            KnapsackItem {
                item: &items[11],
                take_fraction: 1.0,
            },
            KnapsackItem {
                item: &items[2],
                take_fraction: 1.0,
            },
            KnapsackItem {
                item: &items[8],
                take_fraction: 1.0,
            },
            KnapsackItem {
                item: &items[14],
                take_fraction: 0.6,
            },
        ]);
        assert_eq!(actual_chosen_items, expected_chosen_items);
    }
}
