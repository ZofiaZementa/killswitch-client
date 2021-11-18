mod check;
mod config;
mod kill;
mod util;

use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use config::Config;

fn kill_s(cfg: &Config, _sub_matches: &ArgMatches) -> Result<()> {
    kill::kill(&cfg.kill_cmd)
}

fn check_s(cfg: &Config, sub_matches: &ArgMatches) -> Result<()> {
    let ress;
    if sub_matches.is_present("check") {
        ress = check::check_selected(sub_matches.values_of("check").unwrap(), &cfg.checks);
    } else {
        ress = check::check_all(cfg.checks.iter().map(|(n, c)| (n.as_str(), c)));
    }
    let (k, ress) = ress.context("No check could be executed")?;
    if ress.iter().any(|res| res.is_err()) {
        println!("Some errors were encountered during running the checks");
        for res in ress.iter().filter_map(|res| res.as_ref().err()) {
            println!("{:?}", res);
        }
    }
    if !k && !sub_matches.is_present("dryrun") {
        kill::kill(&cfg.kill_cmd)
    } else {
        Ok(())
    }
}

fn main() -> Result<()> {
    let matches = App::new("Killswitch client")
        .author("Maximilian Fischer")
        .about("Provides capabilities for the device to be killed remotely")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .default_value("/etc/killswitch/client.yaml")
                .help("Location of the config file"),
        )
        .subcommand(
            SubCommand::with_name("kill")
                .author("Maximilian Fischer")
                .about("Kill this device"),
        )
        .subcommand(
            SubCommand::with_name("check")
                .author("Maximilian Fischer")
                .about("Execute the checks and kill the device if any one fails")
                .arg(
                    Arg::with_name("dryrun")
                        .short("d")
                        .long("dry-run")
                        .help("Only execute the check(s), don't act on it"),
                )
                .arg(
                    Arg::with_name("check")
                        .takes_value(true)
                        .multiple(true)
                        .help("Check(s) which to execute. If none are given, all are executed"),
                ),
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let cfg = config::get_config(matches.value_of("config").unwrap())?;

    match matches.subcommand() {
        ("kill", Some(sub_matches)) => kill_s(&cfg, sub_matches),
        ("check", Some(sub_matches)) => check_s(&cfg, sub_matches),
        _ => panic!("No subcommand"),
    }
}
