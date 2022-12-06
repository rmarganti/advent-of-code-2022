// use std::env;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.get(1) {
        Some(f) => f,
        _ => return Err(Box::new(AppError("Please supply a file name".to_string()))),
    };

    let contents = fs::read_to_string(file_name)?;
    let mut elf_counts: Vec<u32> = vec![];

    let mut current_elf_total = 0;

    for line in contents.lines() {
        // Empty line means new elf just dropped
        if line.trim().len() == 0 {
            elf_counts.push(current_elf_total);
            current_elf_total = 0;
            continue;
        }

        let line_count = line.parse::<u32>()?;
        current_elf_total += line_count;
    }

    // Add final elf count to vector
    elf_counts.push(current_elf_total);

    // --------------------------------
    // Part 1
    // --------------------------------

    // Find max but keep index
    let max_elf = elf_counts
        .iter()
        .enumerate()
        .max_by(|(_, val0), (_, val1)| val0.cmp(val1));

    if let Some((idx, val)) = max_elf {
        println!("Elf {} has {} calories", idx + 1, val);
    } else {
        return Err(Box::new(AppError(
            "Unable to get calorie counts for elves".to_string(),
        )));
    }

    // --------------------------------
    // Part 2
    // --------------------------------

    elf_counts.sort_by(|val0, val1| val1.cmp(val0));

    println!("{:#?}", elf_counts);

    let top_3_total: u32 = elf_counts.iter().take(3).sum();
    println!("The top 3 elves have a total of {} calories", top_3_total);

    Ok(())
}

#[derive(Debug)]
struct AppError(String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AppError {}

type Result<T> = std::result::Result<T, Box<dyn Error>>;
