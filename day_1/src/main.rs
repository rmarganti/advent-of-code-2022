// use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> Result<()> {
    let contents = fs::read_to_string("calories.txt")?;
    let mut elf_counts: Vec<u16> = vec![];

    let mut current_elf_total = 0;

    for line in contents.lines() {
        // Empty line means new elf just dropped
        if line.trim().len() == 0 {
            elf_counts.push(current_elf_total);
            current_elf_total = 0;
            continue;
        }

        let line_count = line.parse::<u16>()?;
        current_elf_total += line_count;
    }

    // Add final elf count to vector
    elf_counts.push(current_elf_total);

    // Find max but keep index
    let max_elf = elf_counts
        .iter()
        .enumerate()
        .max_by(|(_, val0), (_, val1)| val0.cmp(val1));

    if let Some((idx, val)) = max_elf {
        println!("Elf {} has {} calories", idx + 1, val);
    } else {
        return Err(Box::new(AppError()));
    }

    Ok(())
}

#[derive(Debug)]
struct AppError();

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to get calorie counts for elves")
    }
}

impl Error for AppError {}

type Result<T> = std::result::Result<T, Box<dyn Error>>;
