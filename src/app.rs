use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub process: Option<String>,

    #[clap(short, long)]

    /// Show raw output
    pub raw: bool,

    #[clap(short, long, default_value_t = 0)]
    /// Min swap to show (in kB)
    pub min_swap: u64,
}
