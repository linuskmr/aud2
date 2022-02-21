use aud2::knapsack::*;
use fraction::Fraction;

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
fn test_fractional_greedy() {
    let weight_capacity = 120;
    let actual_chosen_items = fractional_greedy(&ITEMS, weight_capacity);
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
fn test_dynamic_programming() {
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
    let actual_knapsack = dynamic_programming(&max_knapsack_items, weight_limit);
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
fn test_greedy_k() {
    let items = [
        Item {
            id: 0,
            profit: 13,
            weight: 13,
        },
        Item {
            id: 1,
            profit: 11,
            weight: 11,
        },
        Item {
            id: 2,
            profit: 10,
            weight: 10,
        },
        Item {
            id: 3,
            profit: 8,
            weight: 8,
        },
    ];
    let weight_limit = 30;
    let k = 2;
    let actual_knapsack = greedy_k(&items, weight_limit, k);
    assert!(
        actual_knapsack.iter().map(|item| item.weight).sum::<u64>() <= weight_limit,
        "Knapsack solution too heavy"
    );
    let expected_knapsack = [&items[1], &items[2], &items[3]];
    assert_eq!(actual_knapsack, expected_knapsack);
}

#[test]
fn test_integer_greedy() {
    let weight_capacity = 120;
    let actual_knapsack = integer_greedy(&ITEMS, weight_capacity);
    let expected_ids = [6, 4, 13, 12, 3, 9, 16];
    assert_eq!(
        actual_knapsack
            .iter()
            .map(|item| item.id)
            .collect::<Vec<_>>(),
        expected_ids
    );
}

#[test]
fn test_branch_and_bound_1() {
    let items = [
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
    let actual_knapsack = branch_and_bound(&items, 9);
    let expected_knapsack = [&items[3], &items[0]];
    assert_eq!(actual_knapsack, expected_knapsack);
}

#[test]
fn test_branch_and_bound_2() {
    let items = [
        Item {
            id: 0,
            profit: 14,
            weight: 11,
        },
        Item {
            id: 1,
            profit: 6,
            weight: 5,
        },
        Item {
            id: 2,
            profit: 13,
            weight: 13,
        },
        Item {
            id: 3,
            profit: 16,
            weight: 18,
        },
        Item {
            id: 4,
            profit: 9,
            weight: 7,
        },
    ];
    let actual_knapsack = branch_and_bound(&items, 33);
    let expected_knapsack = [&items[2], &items[4], &items[0]];
    assert_eq!(actual_knapsack, expected_knapsack);
}
