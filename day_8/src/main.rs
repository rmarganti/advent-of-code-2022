mod grid;

use grid::Grid;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> shared::Result<()> {
    let contents = shared::read_file_from_args()?;
    let grid = Grid::new(&contents);

    // ------------------------------------------------
    // Part 1
    // ------------------------------------------------

    let mut visible_count: u16 = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.is_visible(x, y) {
                visible_count += 1;
            }
        }
    }

    println!("Part 1: {}", visible_count);

    // ------------------------------------------------
    // Part 2
    // ------------------------------------------------

    let mut max_scenic_score: usize = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let scenic_score = grid.scenic_score(x, y);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("Part 2: {}", max_scenic_score);

    Ok(())
}

