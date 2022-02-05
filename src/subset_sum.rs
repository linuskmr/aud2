use std::collections::HashSet;

/// Solves the [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) via
/// [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
pub fn subset_sum(numbers: &[u64]) -> Vec<HashSet<u64>> {
    // table is a list of iterations. Each iteration contains a set of sum that are producible by using (some of)
    // the first i numbers.
    let mut table: Vec<HashSet<u64>> = Vec::with_capacity(numbers.len());
    if numbers.len() == 0 {
        return table;
    }
    // The number 0 can be produced with the first 0 numbers.
    table.push(HashSet::from([0]));

    // Examine which numbers are producible by using a new number from the number list.
    for new_number in numbers {
        let last_row = table.last().expect("Table always contains one row").clone();
        // All previously reachable numbers are still reachable
        let mut new_row = last_row.clone();
        // In addition, each old number + new_number is now also reachable
        for already_reachable_number in last_row {
            new_row.insert(already_reachable_number + new_number);
        }
        table.push(new_row);
    }
    table
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subset_sum() {
        let actual_table = subset_sum(&[7, 13, 17, 20, 29, 31, 31, 35, 57]);
        let expected_table = [
            HashSet::from([0]),
            HashSet::from([7, 0]),
            HashSet::from([7, 0, 20, 13]),
            HashSet::from([13, 17, 0, 24, 20, 7, 30, 37]),
            HashSet::from([13, 33, 17, 0, 24, 44, 20, 27, 50, 57, 7, 40, 30, 37]),
            HashSet::from([
                42, 33, 17, 73, 24, 36, 50, 57, 62, 69, 59, 79, 30, 13, 44, 27, 37, 0, 46, 29, 20,
                53, 49, 56, 7, 40, 86, 66,
            ]),
            HashSet::from([
                117, 60, 73, 24, 36, 50, 62, 69, 59, 58, 104, 79, 30, 44, 37, 56, 55, 0, 77, 49,
                87, 90, 97, 84, 38, 42, 33, 17, 48, 88, 110, 71, 57, 64, 51, 67, 66, 13, 27, 46,
                53, 93, 29, 81, 20, 68, 75, 31, 7, 40, 86, 80, 61, 100,
            ]),
            HashSet::from([
                100, 119, 73, 24, 36, 50, 62, 69, 104, 79, 30, 37, 56, 55, 141, 0, 91, 115, 49, 97,
                84, 42, 110, 128, 57, 112, 13, 46, 53, 95, 81, 111, 20, 68, 75, 108, 86, 89, 98,
                60, 59, 58, 118, 92, 44, 77, 102, 99, 87, 90, 38, 33, 17, 48, 88, 71, 135, 82, 131,
                64, 51, 121, 67, 66, 27, 93, 31, 124, 106, 29, 7, 117, 148, 40, 80, 61,
            ]),
            HashSet::from([
                36, 183, 62, 116, 79, 37, 139, 126, 0, 85, 147, 115, 49, 153, 133, 96, 97, 130,
                143, 152, 128, 57, 159, 132, 46, 20, 65, 108, 89, 163, 118, 101, 44, 52, 77, 123,
                170, 135, 17, 48, 88, 71, 72, 150, 121, 51, 146, 31, 27, 93, 83, 106, 154, 166,
                119, 7, 148, 40, 156, 100, 73, 24, 69, 50, 127, 104, 55, 30, 56, 141, 145, 94, 91,
                35, 84, 42, 110, 134, 112, 13, 53, 95, 81, 111, 68, 75, 122, 86, 98, 60, 114, 59,
                58, 92, 137, 102, 99, 87, 90, 38, 33, 131, 82, 176, 64, 67, 66, 124, 29, 125, 103,
                117, 80, 61,
            ]),
            HashSet::from([
                174, 36, 183, 62, 172, 158, 140, 116, 151, 149, 79, 203, 37, 139, 126, 220, 0, 85,
                147, 200, 115, 49, 153, 133, 96, 97, 144, 233, 130, 143, 152, 128, 57, 159, 132,
                46, 105, 20, 65, 108, 74, 138, 89, 179, 163, 185, 118, 101, 44, 52, 77, 123, 184,
                191, 202, 170, 135, 17, 48, 88, 71, 72, 150, 121, 51, 146, 31, 196, 27, 93, 83,
                106, 154, 181, 166, 240, 119, 7, 148, 40, 156, 100, 190, 107, 73, 24, 69, 50, 189,
                127, 213, 70, 104, 155, 55, 30, 56, 141, 145, 94, 91, 209, 35, 175, 182, 160, 165,
                211, 84, 178, 167, 42, 180, 110, 134, 187, 112, 216, 13, 53, 95, 142, 81, 111, 68,
                204, 75, 122, 86, 136, 109, 205, 98, 60, 114, 192, 223, 59, 58, 173, 92, 137, 129,
                102, 171, 99, 87, 90, 169, 38, 33, 131, 82, 210, 227, 176, 207, 157, 64, 168, 67,
                66, 124, 113, 161, 188, 29, 198, 125, 103, 117, 80, 194, 61,
            ]),
        ];
        assert_eq!(actual_table, expected_table);
    }
}
