use anyhow::{Context, Result};
use std::collections::BTreeMap;

use crate::{config::Check, util, util::VecError};

pub fn check(c: &Check) -> Result<bool> {
    let res = util::exec(&c.cmd)?;
    // 1 1 -> 0
    // 1 0 -> 1
    // 0 1 -> 1
    // 0 0 -> 0
    Ok(
        (res.status.success() && !c.kill_on_success)
            || (!res.status.success() && c.kill_on_success),
    )
}

pub fn check_selected<'a, T>(
    names: T,
    checks: &BTreeMap<String, Check>,
) -> Result<(bool, Vec<Result<bool>>)>
where
    T: IntoIterator<Item = &'a str>,
{
    let mut ress = Vec::new();
    let mut res = false;
    for name in names {
        let c = checks
            .get(name)
            .with_context(|| format!("Check {} not found", name));
        if c.is_err() {
            ress.push(Err(c.err().unwrap()));
            continue;
        }
        ress.push(
            check(c.ok().unwrap()).with_context(|| format!("Check {} could not be executed", name)),
        );
        if let Some(&ok) = ress.last().unwrap().as_ref().ok() {
            if ok {
                res = true;
            } else {
                return Ok((false, ress));
            }
        }
    }
    if res || ress.is_empty() {
        Ok((true, ress))
    } else {
        Err(VecError::new(ress.into_iter().map(|r| r.err().unwrap()).collect()).into())
    }
}

pub fn check_all<'a, T>(checks: T) -> Result<(bool, Vec<Result<bool>>)>
where
    T: IntoIterator<Item = (&'a str, &'a Check)>,
{
    let mut ress = Vec::new();
    let mut res = false;
    for (name, c) in checks {
        ress.push(check(c).with_context(|| format!("Check {} could not be executed", name)));
        if let Some(&ok) = ress.last().unwrap().as_ref().ok() {
            if ok {
                res = true;
            } else {
                return Ok((false, ress));
            }
        }
    }
    if res || ress.is_empty() {
        Ok((true, ress))
    } else {
        Err(VecError::new(ress.into_iter().map(|r| r.err().unwrap()).collect()).into())
    }
}
