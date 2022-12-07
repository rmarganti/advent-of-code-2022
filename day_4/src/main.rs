use std::ops::RangeInclusive;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> shared::Result<()> {
    let contents = shared::read_file_from_args()?;

    // --------------------------------
    // Part 1
    // --------------------------------

    let has_contained: Vec<bool> = contents
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .filter(|(left, right)| {
            if left.start() >= right.start() && left.end() <= right.end() {
                return true;
            }

            if right.start() >= left.start() && right.end() <= left.end() {
                return true;
            }

            false
        })
        .map(|(_, __)| true)
        .collect();

    println!("Contain: {:#?}", has_contained.len());

    // --------------------------------
    // Part 2
    // --------------------------------

    let has_overlap: Vec<u32> = contents
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .map(|(left, right)| {
            let mut count = 0;

            for i in left {
                if right.contains(&i) {
                    count += 1;
                }
            }

            count
        })
        .filter(|val| val > &0)
        .collect();

    println!("Overlap: {:#?}", has_overlap.len());

    Ok(())
}

/// Convert a line of text like this: `17-57,55-96`
/// into a tuple of left and right inclusive Ranges: `(17..=55, 55..=96)`
///
/// # Panics
///
/// Panics if it can't split and parse the ranges.
fn parse_range(range: &str) -> RangeInclusive<u32> {
    let (low, high) = range.split_once('-').unwrap();
    let low: u32 = low.parse().unwrap();
    let high: u32 = high.parse().unwrap();

    low..=high
}
