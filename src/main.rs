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

fn main() -> Res<()> {
    let filename = &get_file_name();
    let mut data = get_csv(filename)?;
    data.iter_mut().for_each(prompt_word);
    update_csv(filename, data)
}
