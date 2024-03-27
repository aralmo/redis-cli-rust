use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "REDIS-TOOLS", about = "A CLI tool to send commands to redis")]
pub struct Options {
    #[structopt(short, long)]
    pub uri: Option<String>,
    #[structopt(short, long, help = "runs commands for every new line found in stdin.")]    
    pub live : bool,
    #[structopt(short, long, help = "subscribes to a channel.")]    
    pub subscribe : Option<String>
}
