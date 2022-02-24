use aud2::subset_sum::*;

/// Numbers that can be used to produce sums.
static NUMBERS: [u64; 9] = [7, 13, 17, 20, 29, 31, 31, 35, 57];

/// All sums that are reachable with [NUMBERS].
static EXPECTED_REACHABLE_SUMS: [u64; 174] = [
    174, 36, 183, 62, 172, 158, 140, 116, 151, 149, 79, 203, 37, 139, 126, 220, 0, 85, 147, 200,
    115, 49, 153, 133, 96, 97, 144, 233, 130, 143, 152, 128, 57, 159, 132, 46, 105, 20, 65, 108,
    74, 138, 89, 179, 163, 185, 118, 101, 44, 52, 77, 123, 184, 191, 202, 170, 135, 17, 48, 88, 71,
    72, 150, 121, 51, 146, 31, 196, 27, 93, 83, 106, 154, 181, 166, 240, 119, 7, 148, 40, 156, 100,
    190, 107, 73, 24, 69, 50, 189, 127, 213, 70, 104, 155, 55, 30, 56, 141, 145, 94, 91, 209, 35,
    175, 182, 160, 165, 211, 84, 178, 167, 42, 180, 110, 134, 187, 112, 216, 13, 53, 95, 142, 81,
    111, 68, 204, 75, 122, 86, 136, 109, 205, 98, 60, 114, 192, 223, 59, 58, 173, 92, 137, 129,
    102, 171, 99, 87, 90, 169, 38, 33, 131, 82, 210, 227, 176, 207, 157, 64, 168, 67, 66, 124, 113,
    161, 188, 29, 198, 125, 103, 117, 80, 194, 61,
];

type SubsetSumAlgorithm = fn(numbers: &[u64], limit: u64) -> bool;

/// Tests a subset algorithm.
fn test_subset_sum(algorithm: SubsetSumAlgorithm) {
    let max_reachable = *EXPECTED_REACHABLE_SUMS.iter().max().unwrap();
    for number in 0..=max_reachable {
        let expected_reachable = EXPECTED_REACHABLE_SUMS.contains(&number);
        let actual_reachable = algorithm(&NUMBERS, number);
        assert_eq!(expected_reachable, actual_reachable);
    }
}

#[test]
fn test_subset_sum_set() {
    test_subset_sum(subset_sum_set);
}

#[test]
fn test_subset_sum_vec() {
    test_subset_sum(subset_sum_vec);
}
