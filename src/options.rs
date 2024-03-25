use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "REDIS-TOOLS", about = "A CLI tool to send commands to redis")]
pub struct Options {
    #[structopt(short = "u", long = "uri")]
    pub uri: Option<String>,
    #[structopt(short = "l", long = "live", help = "runs commands for every new line found in stdin.")]    
    pub live : bool
}
