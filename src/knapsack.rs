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
