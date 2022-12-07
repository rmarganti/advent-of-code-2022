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

    let sacks: shared::Result<Vec<_>> = contents.lines().map(|line| find_common(&line)).collect();
    let sacks = sacks?;

    let mut points: u32 = sacks.iter().map(points_for_type).sum();

    println!("{:#?}", points);

    // --------------------------------
    // Part 2
    // --------------------------------

    let groups = find_groups(&contents)?;
    points = groups.iter().map(points_for_type).sum();

    println!("{:#?}", points);

    Ok(())
}

fn find_common(line: &str) -> shared::Result<u8> {
    let length = line.len();
    let left = &line[0..length / 2];
    let right = &line[length / 2..length];

    for c in left.as_bytes() {
        if right.as_bytes().contains(c) {
            return Ok(*c);
        }
    }

    return Err(Box::new(shared::AppError(
        "Unable to find common character".to_string(),
    )));
}

fn find_groups(content: &str) -> shared::Result<Vec<u8>> {
    let content: Vec<&str> = content.lines().collect();
    let mut groups: Vec<u8> = vec![];

    for chunk in content.chunks(3) {
        let first = chunk[0].as_bytes();
        let second = chunk[1].as_bytes();
        let third = chunk[2].as_bytes();

        for c in first {
            if second.contains(c) && third.contains(c) {
                groups.push(*c);
                break;
            }
        }
    }

    Ok(groups)
}

fn points_for_type(item_type: &u8) -> u32 {
    if item_type >= &b'a' {
        let point = item_type - b'a' + 1;
        let point = point as u32;
        point
    } else {
        let point = item_type - b'A' + 27;
        let point = point as u32;
        point
    }
}
