use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "classify")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "mysql")]
    Mysql(Mysql),
}

#[derive(Debug, StructOpt)]
pub struct Mysql {
    /// Input file to read
    #[structopt(short = "f", default_value="./reverse.yml")]
    pub file: String,
}