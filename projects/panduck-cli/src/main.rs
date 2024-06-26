use clap::Parser;
pub use notedown_ast::Result;

use self::{builder::PanduckConfig, subs::SubCommands};

mod builder;
mod subs;
mod sync;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Arguments {
    /// Sets a custom config file, derault name is `panduck.toml`.
    /// Then search following extensions: `json5`, `json`, `yaml`
    #[clap(short, long)]
    config: Option<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: u32,
    /// Warning without output, notice still do effects like downloads
    #[clap(short('r'), long)]
    dry_run: bool,
    /// Record running time, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    timing: u32,
    /// Record running time, and can be used multiple times
    #[clap(long)]
    ignore_global: bool,
    #[clap(subcommand)]
    cmd: SubCommands,
}

fn main() -> Result<()> {
    let args: Arguments = Arguments::parse();
    let mut config = match args.config {
        None => PanduckConfig::default(),
        Some(_) => {
            todo!()
        }
    };
    config.verbose = args.verbose;
    args.cmd.dispatch(&mut config)
}
