//! Independent checks. The reference counter below does not call the library sort.

fn reference_counter(a: &mut [u64], gaps: &[u64]) -> (u64, u64) {
    let n = a.len();
    let mut comparisons = 0u64;
    let mut moves = 0u64;
    for &gap in gaps {
        let h = gap as usize;
        for i in h..n {
            let tmp = a[i];
            let mut j = i;
            while j >= h {
                comparisons += 1;
                if a[j - h] <= tmp {
                    break;
                }
                a[j] = a[j - h];
                moves += 1;
                j -= h;
            }
            a[j] = tmp;
            moves += 1;
        }
    }
    (comparisons, moves)
}

#[test]
fn first_16_exec_gaps() {
    let expected = [
        1, 3, 8, 20, 47, 116, 300, 585, 1416, 3303, 7703, 17963, 41887, 97670, 227746, 531052,
    ];
    assert_eq!(liu_shellsort::exec_gaps(16), expected);
    let mut below = liu_shellsort::gaps_below(531053, 32);
    below.reverse();
    assert_eq!(below, expected);
}

#[test]
fn backbone_and_tuned_replacements() {
    let backbone: Vec<u64> = (4..=15).map(liu_shellsort::h_t).collect();
    assert_eq!(
        backbone,
        vec![1, 3, 8, 20, 47, 111, 260, 607, 1416, 3303, 7703, 17963]
    );
    let exec = liu_shellsort::exec_gaps(12);
    assert_eq!(&exec[8..12], &[1416, 3303, 7703, 17963]);
    assert_eq!(
        &exec[8..12],
        &[
            liu_shellsort::h_t(12),
            liu_shellsort::h_t(13),
            liu_shellsort::h_t(14),
            liu_shellsort::h_t(15)
        ]
    );
    // Tuned prefix replaces only h_9, h_10, h_11.
    assert_eq!(&exec[0..5], &backbone[0..5]);
    assert_eq!(&backbone[5..8], &[111, 260, 607]);
    assert_eq!(&exec[5..8], &[116, 300, 585]);
    assert_eq!(&exec[8..12], &backbone[8..12]);

    let gaps = liu_shellsort::gaps_below(700, 8);
    assert_eq!(gaps, vec![585, 300, 116, 47, 20, 8, 3, 1]);
    assert!(!gaps.contains(&111));
    assert!(!gaps.contains(&260));
    assert!(!gaps.contains(&607));
}

#[test]
fn gaps_below_20() {
    let gaps = liu_shellsort::gaps_below(20, 32);
    let from_exec: Vec<u64> = liu_shellsort::exec_gaps(16)
        .into_iter()
        .filter(|&h| (h as usize) < 20)
        .collect();
    let mut expected = from_exec;
    expected.sort_unstable_by(|a, b| b.cmp(a));
    expected.dedup();
    assert_eq!(gaps, expected);
    assert_eq!(gaps, vec![8, 3, 1]);
    assert!(gaps.windows(2).all(|w| w[0] > w[1]));
    assert_eq!(gaps.last().copied(), Some(1));
    assert!(gaps.iter().all(|&h| (h as usize) < 20));
}

#[test]
fn sorts_reversed_sorted_duplicates_and_permutations() {
    let mut cases: Vec<Vec<u64>> = Vec::new();
    cases.push((0..24).rev().collect());
    cases.push((0..24).collect());
    cases.push(vec![4, 4, 1, 1, 2, 2, 4, 0, 0, 3]);
    cases.push(vec![2, 1]);
    cases.push(vec![3, 1, 2]);
    cases.push(vec![5, 1, 4, 2, 8, 3, 7, 6]);
    cases.push(vec![9, 0, 9, 1, 8, 2, 7, 3, 6]);
    cases.push(vec![]);
    cases.push(vec![42]);

    for case in cases {
        let mut a = case.clone();
        liu_shellsort::shellsort(&mut a);
        let mut expected = case;
        expected.sort_unstable();
        assert_eq!(a, expected);
    }
}

#[test]
fn counts_match_independent_reference() {
    // Hand counts for the first four arrays, using the stated convention
    // and not the library sort: gaps strictly below N, descending.
    let cases = [
        vec![2u64, 1],
        vec![1, 2],
        vec![2, 2],
        vec![4, 3, 2, 1],
        vec![1, 3, 2, 3, 1],
        vec![8, 7, 6, 5, 4, 3, 2, 1, 0],
    ];
    let hand = [(1u64, 2u64), (1, 1), (1, 1), (5, 6)];

    for (idx, case) in cases.iter().enumerate() {
        let gaps = liu_shellsort::gaps_below(case.len(), 32);
        let mut by_lib = case.clone();
        let mut by_ref = case.clone();
        let lib = liu_shellsort::shellsort(&mut by_lib);
        let reference = reference_counter(&mut by_ref, &gaps);
        assert_eq!(by_lib, by_ref, "sorted arrays differ for {case:?}");
        assert_eq!(lib, reference, "counts differ for {case:?}");
        if idx < hand.len() {
            assert_eq!(reference, hand[idx], "hand count mismatch for {case:?}");
        }
        let mut sorted = case.clone();
        sorted.sort_unstable();
        assert_eq!(by_lib, sorted);
    }
}

#[test]
fn deterministic_counts() {
    let inputs = [
        vec![9u64, 4, 7, 1, 3, 8, 2, 6, 5],
        vec![1, 1, 1, 1],
        vec![6, 5, 4, 3, 2, 1],
    ];
    for input in inputs {
        let mut a = input.clone();
        let mut b = input.clone();
        let first = liu_shellsort::shellsort(&mut a);
        let second = liu_shellsort::shellsort(&mut b);
        assert_eq!(first, second);
        assert_eq!(a, b);
    }
}
