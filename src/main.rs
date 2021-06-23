use quicli::prelude::*;
use structopt::StructOpt;
use app_arguments::{ApplicationArguments, Command};
mod app_arguments;
mod custom_config;
mod common;
mod mysql_struct;

extern crate inflector;

#[async_std::main]
async fn main() -> CliResult {
    let args = ApplicationArguments::from_args();
    let key = args.command;
    match key {
        Command::Mysql(opt) => {
            let s =read_file(&opt.file)?;
            let config:custom_config::CustomConfig = serde_yaml::from_str(&s)?;
            mysql_struct::gen_struct::run(config).await?;
        }
    }
    Ok(())
}
