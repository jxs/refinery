//! Main entry point for the refinery cli tool

mod cli;
mod migrate;
mod setup;

use env_logger::Builder;
use exitfailure::ExitDisplay;
use failure::Error;
use log::LevelFilter;
use std::io::Write;

const APP_NAME: &'static str = "refinery";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), ExitDisplay<Error>> {
    human_panic::setup_panic!(Metadata {
        name: APP_NAME.into(),
        version: VERSION.into(),
        authors: "Katharina Fey <kookie@spacekookie.de>, João Oliveira <hello@jxs.pt>".into(),
        homepage: "https://github.com/rust-db/refinery/".into(),
    });

    let mut builder = Builder::new();
    builder
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(Some("refinery_migrations::traits"), LevelFilter::Info)
        .init();

    let matches = cli::create_cli().get_matches();

    match matches.subcommand() {
        ("migrate", Some(matches)) => migrate::handle_migration_command(matches)?,
        ("setup", Some(matches)) => setup::handle_setup(matches)?,
        _ => unreachable!("Can't touch this..."),
    };
    Ok(())
}
