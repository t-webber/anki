use crate::{file::Line, now, Res};

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
    print!("\t {} \t => ?", line.key);
    std::io::stdin().read_line(&mut String::new())?;
    print!("\t {} \t => \t {} \t[y/N/o]", line.key, line.value);
    let buf = &mut String::new();
    std::io::stdin().read_line(buf)?;
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
