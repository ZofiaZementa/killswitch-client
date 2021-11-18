use crate::util;
use anyhow::{bail, Result};

pub fn kill(cmd: &str) -> Result<()> {
    let res = util::exec(cmd)?;
    if res.status.success() {
        Ok(())
    } else {
        bail!(String::from_utf8(res.stderr)?)
    }
}
