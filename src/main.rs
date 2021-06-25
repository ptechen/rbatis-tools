use app_arguments::{ApplicationArguments, Command};
use quicli::prelude::*;
use structopt::StructOpt;
mod app_arguments;
mod custom_config;
mod gen_struct;
use gen_struct::GenStruct;

extern crate inflector;

#[async_std::main]
async fn main() -> CliResult {
    let args = ApplicationArguments::from_args();
    let key = args.command;
    match key {
        Command::Mysql(opt) => {
            let s = read_file(&opt.file)?;
            let config: custom_config::CustomConfig = serde_yaml::from_str(&s)?;
            let mysql = gen_struct::mysql_struct::MysqlStruct::new(
                config,
                opt.template_path,
                opt.template_name,
            )?;
            mysql.run().await?;
        }
    }
    Ok(())
}
