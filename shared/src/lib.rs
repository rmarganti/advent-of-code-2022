use std::error::Error;
use std::{env, fmt, fs};

/// Read the contents provided via the first argument.
///
/// # Errors
///
/// This function will return an error if:
///   - A file name is not supplied
///   - We are unable to read the file
pub fn read_file_from_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.get(1) {
        Some(f) => f,
        _ => return Err(Box::new(AppError("Please supply a file name".to_string()))),
    };

    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

#[derive(Debug)]
pub struct AppError(pub String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AppError {}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
