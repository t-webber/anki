use crate::{file::Line, now, Res, MAX_WORDS_PER_SESSION};
use std::io::{self, Write as _};

fn interval_heuristic(line: &mut Line, success: Success) -> Res<()> {
    let last_interval = std::cmp::max(1, line.next - line.last);
    line.last = now()?;
    line.next = line.last
        + match success {
            Success::Yes => last_interval * 2,
            Success::No => last_interval / 4,
            Success::Kinda => last_interval,
        };
    Ok(())
}

enum Success {
    Yes,
    No,
    Kinda,
}

impl<T> From<T> for Success
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref().trim().to_lowercase().as_str() {
            "y" => Success::Yes,
            "o" => Success::Kinda,
            _ => Success::No,
        }
    }
}

fn io_prompt(line: &Line) -> Res<Success> {
    let mut buf = String::new();
    write!(io::stdout(), "{} ? ", line.key)?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    buf.clear();
    write!(io::stdout(), "\t>>> {} \t[y/N/o] ", line.value)?;
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    Ok(buf.into())
}

fn prompt_word_wrapper(line: &mut Line) -> Res<()> {
    interval_heuristic(line, io_prompt(line)?)
}

pub fn prompt_word(line: &mut Line) {
    if let Err(e) = prompt_word_wrapper(line) {
        eprintln!("{e}");
    }
}
