use std::cmp::Ordering;
use std::fmt;

#[derive(Eq, PartialEq, Clone)]
pub struct Item {
    pub id: usize,
    pub profit: u64,
    pub weight: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChosenItem<'a> {
    pub item: &'a Item,
    pub take_fraction: f64,
}

impl<'a> ChosenItem<'a> {
    pub fn effective_weight(&self) -> f64 {
        (self.item.weight as f64) * self.take_fraction
    }

    pub fn effective_profit(&self) -> f64 {
        (self.item.profit as f64) * self.take_fraction
    }
}

impl Item {
    fn weight_profit_ration(&self) -> f64 {
        (self.weight as f64) / (self.profit as f64)
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Item")
            .field("id", &self.id)
            .field("weight", &self.weight)
            .field("profit", &self.profit)
            .field("ratio_z/p", &self.weight_profit_ration())
            .finish()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight_profit_ration()
            .partial_cmp(&other.weight_profit_ration())
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("Illegal values in item which make it unable to be sorted")
    }
}

pub fn fractional_knapsack(items: &[Item], weight_capacity: u64) -> Vec<ChosenItem<'_>> {
    let mut items_sorted_asc: Vec<&Item> = items.iter().collect();
    items_sorted_asc.sort();

    {
        let items_sorted_ids: Vec<usize> = items_sorted_asc.iter().map(|item| item.id).collect();
        println!("Sorted item ids: {:?}", items_sorted_ids);
    }

    let mut chosen_items: Vec<ChosenItem> = Vec::new();

    for (index, item) in items_sorted_asc.iter().enumerate() {
        let used_weight_capacity: f64 = chosen_items
            .iter()
            .map(|chosen_item| chosen_item.effective_weight())
            .sum();
        let available_capacity: f64 = (weight_capacity as f64) - used_weight_capacity;
        let reached_effective_profit: f64 = chosen_items
            .iter()
            .map(|chosen_item| chosen_item.effective_profit())
            .sum();

        if available_capacity <= 0.0 {
            break;
        }

        let take_fraction = ((available_capacity as f64) / (item.weight as f64)).min(1.0);
        let chosen_item = ChosenItem {
            item,
            take_fraction,
        };
        chosen_items.push(chosen_item);

        log::info!("round={:<2} current_id={:<2} take_fraction={:<3} available_capacity={:<3} used_capacity={:<3} effective_profit={:<2}",
				 index, item.id, take_fraction, available_capacity, used_weight_capacity, reached_effective_profit);
    }

    chosen_items
}
