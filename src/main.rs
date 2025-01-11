mod error;
mod proxy;
mod skeleton;

use clap::{Args, Parser, Subcommand};

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Opt {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run jproxy
    Run(Bootstrap),
}

#[derive(Args, Clone)]
pub struct Bootstrap {
    /// Debug mode
    #[clap(long, env = "JPROXY_DEBUG")]
    debug: bool,

    /// Config path, eg: --conf ./configs
    #[clap(short, long, default_value = "configs")]
    conf: String,

    /// Concurrent connections
    #[clap(long, default_value = "1024")]
    concurrent: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    match opt.commands {
        Commands::Run(args) => proxy::run(args)?,
    };

    Ok(())
}
