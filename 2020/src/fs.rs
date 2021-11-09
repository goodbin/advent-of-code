use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::Error;

#[derive(Debug, thiserror::Error)]
#[error("{path}: {source}")]
pub struct IoError {
    path: PathBuf,
    #[source]
    source: io::Error,
}

trait WithContext<C, T, E> {
    fn with_context<CC: Into<C>>(self, ctx: CC) -> Result<T, E>;
}

impl<T> WithContext<PathBuf, T, IoError> for Result<T, io::Error> {
    fn with_context<CC: Into<PathBuf>>(self, ctx: CC) -> Result<T, IoError> {
        self.map_err(|e| IoError {
            path: ctx.into(),
            source: e,
        })
    }
}

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    Ok(std::fs::read_to_string(&path).with_context(path.as_ref())?)
}

pub fn parse_input<R, P>(path: P, pat: &str) -> Result<Vec<R>, Error>
where
    R: FromStr,
    P: AsRef<Path>,
{
    read_input(path)?
        .trim()
        .split(pat)
        .map(|v| R::from_str(v).map_err(|_| Error::Parse(String::from(v))))
        .collect()
}
