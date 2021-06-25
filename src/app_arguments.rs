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
    /// Input config file to read
    #[structopt(short = "f", default_value = "./reverse.yml")]
    pub file: String,
    /// Input template path
    #[structopt(short = "p", default_value = "templates/*")]
    pub template_path: String,
    /// Input template name
    #[structopt(short = "n", default_value = "base.tera")]
    pub template_name: String,
}
