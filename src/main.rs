use tracing::Level;
use vfs::{PhysicalFS, VfsPath};
use zapp::{zapp, Config};

use clap::Parser;

#[derive(Parser)]
#[command(name = "zapp", about = "A simple CLI example", version = "1.0")]
struct Cli {
    #[arg(short, long)]
    ruff: bool,

    #[arg(short, long)]
    directory: Option<String>,

    #[arg(short, long)]
    log_level: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let filesystem: Option<VfsPath> = match args.directory {
        Some(dir) => Some(PhysicalFS::new(dir).into()),
        None => None,
    };

    let config = Config {
        ruff_format: args.ruff,
        filesystem: filesystem,
        log_level: Some(Level::TRACE),
    };

    zapp(config);
}
