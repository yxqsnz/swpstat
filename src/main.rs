use app::Cli;
use clap::Parser;
use see::see;

mod app;
mod see;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    see(&cli)
}
