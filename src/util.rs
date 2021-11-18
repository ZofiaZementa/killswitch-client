use anyhow::{anyhow, Error, Result};
use shell_words;
use std::{
    fmt,
    process::{Command, Output},
};

#[derive(Debug)]
pub struct VecError {
    errs: Vec<Error>,
}

impl VecError {
    pub fn new(errs: Vec<Error>) -> Self {
        VecError { errs }
    }
}

impl fmt::Display for VecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in self.errs.iter() {
            writeln!(f, "{}", e)?;
        }
        Ok(())
    }
}

impl std::error::Error for VecError {}
pub fn exec(cmd: &str) -> Result<Output> {
    let parts = shell_words::split(cmd)?;
    let res = Command::new(parts.first().ok_or(anyhow!("Command was empty"))?)
        .args(&parts[1..])
        .output()?;
    Ok(res)
}
