fn main() {
    let gaps = liu_shellsort::exec_gaps(16);
    println!(
        "{}",
        gaps.iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let mut example = [8u64, 3, 1, 7, 2, 6, 4, 5];
    let (comparisons, moves) = liu_shellsort::shellsort(&mut example);
    println!("{example:?}");
    println!("comparisons {comparisons}");
    println!("moves {moves}");
}
