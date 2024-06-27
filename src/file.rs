use crate::{now, Res};

pub struct Line {
    pub key: Box<str>,
    pub value: Box<str>,
    // next time the question must be asked
    pub next: u64,
    // last time the question was asked
    pub last: u64,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{};{}", self.key, self.value, self.next, self.last)
    }
}

pub fn get_csv(filename: &str) -> Res<Vec<Line>> {
    std::fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split(';'))
        .map(|mut splited| {
            Ok(Line {
                key: splited.next().ok_or("Invalid synthax for key.")?.into(),
                value: splited.next().ok_or("Invalid synthax for value.")?.into(),
                next: splited
                    .next()
                    .map(str::parse)
                    .and_then(Result::ok)
                    .unwrap_or(now()?),
                last: splited
                    .next()
                    .map(str::parse)
                    .and_then(Result::ok)
                    .unwrap_or(now()? - 1),
            })
        })
        .collect()
}

pub fn update_csv(filename: &str, data: Vec<Line>) -> Res<()> {
    std::fs::write(
        filename,
        data.iter()
            .map(Line::to_string)
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .map_err(|e| e.into())
}
