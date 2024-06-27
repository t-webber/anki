mod file;
use file::{get_csv, update_csv};
use logic::prompt_word;
mod logic;

fn get_file_name() -> String {
    format!(
        "./data/{}.csv",
        std::env::args().nth(1).unwrap_or("en".into())
    )
}

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn now() -> Res<u64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs()
        / 600)
}

const MAX_WORDS_PER_SESSION: usize = 10;

fn main() -> Res<()> {
    let filename = &get_file_name();
    let mut data = get_csv(filename)?;
    let mut prompted = 0usize;
    for line in data.iter_mut() {
        if line.next <= now()? && prompted < MAX_WORDS_PER_SESSION {
            prompt_word(line);
            prompted += 1;
        }
    }
    update_csv(filename, data)
}
