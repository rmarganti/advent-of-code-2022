use std::{env, fs};

fn main() {
    if let Err(e) = do_main() {
        eprintln!("{}", e);
    }
}

fn do_main() -> shared::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.get(1) {
        Some(f) => f,
        _ => {
            return Err(Box::new(shared::AppError(
                "Please supply a file name".to_string(),
            )))
        }
    };

    let contents = fs::read_to_string(file_name)?;

    // Part 1
    let mut rounds = parse_contents_part1(&contents)?;
    let mut total_score = tally_scores(&rounds);
    println!("Total score part 1: {}", total_score);

    // Part 2
    rounds = parse_contents_part2(&contents)?;
    total_score = tally_scores(&rounds);
    println!("Total score part 2: {}", total_score);

    Ok(())
}

// The result of a single round. The first item is the elf play,
// and the second is the player's response.
type Round = (Play, Play);

/// Parse the contents of the file into a vector of rounds.
/// For elf:
/// A -> Rock
/// B -> Paper
/// C -> Scissors
///
/// For player:
/// X -> Rock
/// Y -> Paper
/// Z -> Scissors
fn parse_contents_part1(contents: &str) -> shared::Result<Vec<Round>> {
    let result = contents
        .lines()
        .map(|line| {
            // Split a string at spaces
            let round = line.split(" ").collect::<Vec<&str>>();

            let elf_play = match round[0] {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                play => panic!("Invalid input: {}", play),
            };

            let user_play = match round[1] {
                "X" => Play::Rock,
                "Y" => Play::Paper,
                "Z" => Play::Scissors,
                play => panic!("Invalid response: {}", play),
            };

            return (elf_play, user_play);
        })
        .collect();

    Ok(result)
}

/// Parse the contents of the file into a vector of rounds.
/// For elf:
/// A -> Rock
/// B -> Paper
/// C -> Scissors
///
/// For player:
/// X -> Lose
/// Y -> Win
/// Z -> Draw
///
/// The player's response is based on the elf's play.
fn parse_contents_part2(contents: &str) -> shared::Result<Vec<Round>> {
    let result = contents
        .lines()
        .map(|line| {
            // Split a string at spaces
            let round = line.split(" ").collect::<Vec<&str>>();

            let elf_play = match round[0] {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                play => panic!("Invalid input: {}", play),
            };

            let user_play = match round[1] {
                "X" => match elf_play {
                    Play::Rock => Play::Scissors,
                    Play::Paper => Play::Rock,
                    Play::Scissors => Play::Paper,
                },
                "Y" => match elf_play {
                    Play::Rock => Play::Rock,
                    Play::Paper => Play::Paper,
                    Play::Scissors => Play::Scissors,
                },
                "Z" => match elf_play {
                    Play::Rock => Play::Paper,
                    Play::Paper => Play::Scissors,
                    Play::Scissors => Play::Rock,
                },
                play => panic!("Invalid response: {}", play),
            };

            return (elf_play, user_play);
        })
        .collect();

    Ok(result)
}

fn get_round_result(round: &Round) -> RoundResult {
    let (elf, player) = round;
    match elf {
        Play::Rock => match player {
            Play::Rock => RoundResult::Draw,
            Play::Paper => RoundResult::Win,
            Play::Scissors => RoundResult::Loss,
        },
        Play::Paper => match player {
            Play::Rock => RoundResult::Loss,
            Play::Paper => RoundResult::Draw,
            Play::Scissors => RoundResult::Win,
        },
        Play::Scissors => match player {
            Play::Rock => RoundResult::Win,
            Play::Paper => RoundResult::Loss,
            Play::Scissors => RoundResult::Draw,
        },
    }
}

fn tally_scores(rounds: &[Round]) -> u32 {
    let mut total_score = 0;

    for round in rounds {
        let choice_score = match round.1 {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        total_score += choice_score;

        let result_score = match get_round_result(round) {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };

        total_score += result_score;
    }

    total_score
}

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Loss,
    Draw,
}
