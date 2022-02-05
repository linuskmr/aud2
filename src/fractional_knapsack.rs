use crate::knapsack::{Item, Knapsack, KnapsackItem};
use std::borrow::Borrow;

/// Solves the [fractional knapsack problem](https://en.wikipedia.org/wiki/Continuous_knapsack_problem) by using the
/// [greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm).
pub fn fractional_knapsack(items: &[Item], weight_capacity: u64) -> Knapsack {
    // Sort items ascending according to their weight profit ratio. This causes valuable elements to be at the front
    // and not so valuable elements at the back.
    let mut items_sorted_asc: Vec<&Item> = items.iter().collect();
    items_sorted_asc.sort();

    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc.iter().map(|item| item.id).collect();
        println!("Sorted item ids: {:?}", items_sorted_ids);
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
