use anyhow::Result;
use colored::Colorize;
use humansize::{FormatSize, DECIMAL};
use once_cell::sync::Lazy;
use procfs::process::{self, Process};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;

use crate::app::Cli;
static PROCESS: Lazy<String> = Lazy::new(|| "Process".green().to_string());

fn print_proc(cli: &Cli, proc: &Process) -> Result<()> {
    let Cli { min_swap, raw, .. } = cli;
    let status = proc.status()?;

    if let Some(swap) = status.vmswap {
        if swap > *min_swap {
            let exe = proc
                .exe()
                .ok()
                .map_or_else(|| String::from("none"), |e| e.to_string_lossy().to_string());

            if *raw {
                println!("{} {} {} {swap}", exe, status.name, status.pid);
            } else {
                println!(
                    " {} {} ({}) {}",
                    PROCESS.as_str(),
                    status.name,
                    status.pid,
                    (swap * 1024).format_size(DECIMAL)
                );
            }
        }
    }

    Ok(())
}

pub fn see(cli: &Cli) -> Result<()> {
    process::all_processes()?
        .par_bridge()
        .filter_map(std::result::Result::ok)
        .for_each(|proc| {
            if let Some(ref name) = cli.process {
                if let Ok(status) = proc.status() {
                    if status.pid.to_string() == *name || status.name == *name {
                        print_proc(cli, &proc).ok();
                    }
                }
            } else {
                print_proc(cli, &proc).ok();
            }
        });
    Ok(())
}
